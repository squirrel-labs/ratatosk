let mem;
let decoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
let u8mem;
let u32mem;
let wasm;

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

function onwasm(ctx, desc, module) {
    if (typeof desc.canvas === "undefined") {
        console.log("starting");
        module.run_main_loop();
        console.log("finished");
    } else {
        module.initialise_graphics_context(desc.canvas);
        setInterval(module.draw_frame, desc.deltaTime);
    }
}

onmessage = async function({ data }) {
    // set global memory for function imports
    mem = data.memory;
    let mod = data.compiled;
    let imp = {env: imports};
    imp.env.memory = mem;
    u8mem = new Uint8Array(mem.buffer);
    u32mem = new Uint32Array(mem.buffer);
    wasm = await WebAssembly.instantiate(mod, imp);
    if (typeof data.canvas === "undefined") {
        wasm.exports.__sp.value -= 1024 * 64;
        wasm.exports.__wasm_init_memory();
        wasm.exports.__wasm_init_tls();
        wasm.exports.init(wasm.exports.__heap_base.value);
        wasm.exports.run_logic();

    } else {
        this.setInterval(wasm.exports.draw_frame(), 100);
    }
}
