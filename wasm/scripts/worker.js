let mem;
let decoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
let u8mem;
let u32mem;
let f32mem;
let wasm;
let canvas;
let gl;
let programs = [];
let shaders = [];
let posLoc, matLoc, texBoundLoc, texLayerLoc;  // TODO: Query them
let matBuffer, texBuffer;
// vertex and fragment shader
let vs, fs;

function str_from_mem(ptr, len) {
    return decoder.decode(u8mem.slice(ptr, ptr + len));
}

function arr_from_mem(ptr, len) {
    return u32mem.slice(ptr >> 2, (ptr >> 2) + (len >> 2));
}

const imports = {
    log_debug: function(msg, len) {
        console.debug(str_from_mem(msg, len), 'color:plum', '');
    },
    log_info: function(msg, len) {
        console.info(str_from_mem(msg, len), 'color:#1b8', '');
    },
    log_warn: function(msg, len) {
        console.warn(str_from_mem(msg, len), 'color:#fa2', '');
    },
    log_error: function(msg, len) {
        console.error(str_from_mem(msg, len), 'color:red', '');
    },
    log_panic: function(msg, len, file, flen, line, column) {
        console.error('%cPANIC\tpanic at line ' + line + ' and column ' + column + ' in file "' + str_from_mem(file, flen) + '"\n' + str_from_mem(msg, len), 'color:red');
    },
    post_to_main: function(msg, len) {
        this.postMessage(arr_from_mem(msg, len));
    },
    get_canvas_size: function() {
        return (canvas.width << 16) | canvas.height;
    },
    set_canvas_size: function(w, h) {
        canvas.width = w;
        canvas.height = h;
    },
    gl_get_error: function() {
        return gl.getError();
    },
    gl_create_vertex_array_and_buffer_with_data: function(ptr, len) {
        // ATTENTION: The pointer must be 32-bit aligned.
        const vao = gl.createVertexArray();
        if (!(vao instanceof WebGLVertexArrayObject)) return 1;
        gl.bindVertexArray(vao);
        const vbo = gl.createBuffer();
        if (!(vbo instanceof WebGLBuffer)) return 2;
        gl.bindBuffer(gl.ARRAY_BUFFER, vbo);
        gl.bufferData(gl.ARRAY_BUFFER, f32mem, gl.STATIC_DRAW, ptr >> 2, len);
        gl.enableVertexAttribArray(posLoc);
        gl.vertexAttribPointer(0, 2, gl.FLOAT, false, 0, 0);
        return 0;
    },
    gl_allocate_buffers: function(matPtr, texBoundPtr, texLayerPtr, instances) {
        // ATTENTION: All pointers must be 32-bit aligned.
        if (typeof matBuffer === 'undefined') {
            matBuffer = gl.createBuffer();
            if (!(matBuffer instanceof WebGLBuffer)) return 1;
        }
        gl.bindBuffer(gl.ARRAY_BUFFER, matBuffer);
        gl.bufferData(gl.ARRAY_BUFFER, f32mem, gl.DYNAMIC_DRAW, matPtr >> 2, instances * 3 * 3);
        for (let i = 0; i < 3; i++) {
            gl.enableVertexAttribArray(matLoc + i);
            gl.vertexAttribPointer(matLoc + i, 3, gl.FLOAT, false, 4 * 3 * 3, i * 4 * 3);
            gl.vertexAttribDivisor(matLoc + i, 1);
        }
        if (typeof texBuffer === 'undefined') {
            texBuffer = gl.createBuffer();
            if (!(texBuffer instanceof WebGLBuffer)) return 2;
        }
        gl.bindBuffer(gl.ARRAY_BUFFER, texBuffer);
        gl.bufferData(gl.ARRAY_BUFFER, (4 * 4 + 4) * instances, gl.STATIC_DRAW);
        gl.bufferSubData(gl.ARRAY_BUFFER, 0, f32mem, texBoundPtr >> 2, instances * 4);
        gl.bufferSubData(gl.ARRAY_BUFFER, instances * 4 * 4, u32mem, texLayerPtr >> 2, instances);
        gl.enableVertexAttribArray(texBoundLoc);
        gl.vertexAttribPointer(texBoundLoc, 4, gl.FLOAT, false, 0, 0);
        gl.vertexAttribDivisor(texBoundLoc, 1);
        gl.enableVertexAttribArray(texLayerLoc);
        gl.vertexAttribIPointer(texLayerLoc, 1, gl.UNSIGNED_INT, 0, 4 * 4 * instances);
        gl.vertexAttribDivisor(texLayerLoc, 1);
    },
    gl_create_program: function() {
        const prog = gl.createProgram();
        if (!(prog instanceof WebGLProgram)) return -1;
        programs.push(prog);
        return programs.length - 1;
    },
    gl_attach_new_shader: function(progHandle, shaderType) {
        const prog = programs[progHandle];
        if (typeof prog === 'undefined') return 1;
        let source, typeName;
        if (shaderType === gl.VERTEX_SHADER) {
            source = vs;
            typeName = 'vertex';
        } else if (shaderType === gl.FRAGMENT_SHADER) {
            source = fs;
            typeName = 'fragment';
        } else return 2;
        const shader = gl.createShader(shaderType);
        if (!(shader instanceof WebGLShader)) return 3;
        gl.shaderSource(shader, source);
        gl.compileShader(shader);
        gl.attachShader(prog, shader);
        shaders.push([typeName, shader]);
        return 0;
    },
    gl_link_program: function(progHandle) {
        const prog = programs[progHandle];
        if (typeof prog === 'undefined') return 1;
        gl.linkProgram(prog);
        gl.validateProgram(prog);
        if (!gl.getProgramParameter(prog, gl.LINK_STATUS)) {
            for (const [typeName, shader] of shaders) {
                if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
                    const logInfo = gl.getShaderInfoLog(shader) || '<empty message>';
                    console.error(typeName + ' shader compilation failed with\n' + logInfo);
                }
            }
            const logInfo = gl.getProgramInfoLog(prog) || '<empty message>';
            console.error('program shader failed with\n' + logInfo);
            return 2;
        }
        return 0;
    }
};

// handle the initialisation
onmessage = async function({ data }) {
    // set global memory for function imports
    mem = data.memory;
    const is_logic = typeof data.canvas === 'undefined';
    u8mem = new Uint8Array(mem.buffer);
    u32mem = new Uint32Array(mem.buffer);
    f32mem = new Float32Array(mem.buffer);
    wasm = await WebAssembly.instantiate(data.compiled, {env: {
        ...imports,
        memory: mem
    }});
    if (is_logic) {
        wasm.exports.__sp.value -= 1024 * 64;
        wasm.exports.__wasm_init_memory();
        wasm.exports.__wasm_init_tls();
        wasm.exports.init(wasm.exports.__heap_base.value);
        wasm.exports.run_logic();
    } else {
        canvas = data.canvas;
        vs = data.shader.vertex;
        fs = data.shader.fragment;
        // see 'https://www.khronos.org/registry/webgl/specs/latest/1.0/' for documentation
        gl = canvas.getContext('webgl2', {
            alpha: false,
            depth: false,
            stencil: true,
            antialias: true,
            premultipliedAlpha: true,
            preserveDrawingBufferd: true,
            powerPreference: "high-performance",
            failIfMajorPerformanceCaveat: false,
            // desynchronized seems to remove multibuffering that reduces latency at cost of extreme tearing effects
            desynchronized: false,
        });
        if (gl instanceof WebGL2RenderingContext) {
            this.setInterval(wasm.exports.draw_frame, 100);
        } else {
            console.error('failed to create a webgl2 context');
        }
    }
}
