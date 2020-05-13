let mem;
let decoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
let u8mem;
let u32mem;
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
};

const gl_imports = {
};

// handle the initialisation
onmessage = async function({ data }) {
    // set global memory for function imports
    mem = data.memory;
    const is_logic = typeof data.canvas === 'undefined';
    let imp = {env: imports};
    if (!is_logic) {
        imp.env = {...inp.env, ...gl_imports};
    }
    imp.env.memory = mem;
    u8mem = new Uint8Array(mem.buffer);
    u32mem = new Uint32Array(mem.buffer);
    wasm = await WebAssembly.instantiate(data.compiled, imp);
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
            this.setInterval(wasm.exports.draw_frame(), 100);
        } else {
            console.error('failed to create a webgl2 context');
        }
    }
}
