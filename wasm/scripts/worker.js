let mem;
let decoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
let u8mem;
let u32mem;
let f32mem;
let wasm;
let canvas;
let gl;

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
        if (!(vao instanceof WebGLVertexArrayObject)) { return 1; }
        gl.bindVertexArray(vao);
        const vbo = gl.createBuffer();
        if (!(vbo instanceof WebGLBuffer)) { return 2; }
        gl.bindBuffer(gl.ARRAY_BUFFER, vbo);
        gl.bufferData(gl.ARRAY_BUFFER, f32mem, gl.STATIC_DRAW, ptr >> 2, len);
        gl.enableVertexAttribArray(0);
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
