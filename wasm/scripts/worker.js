let mem;
let decoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

function str_from_mem(ptr, len) {
    return decoder.decode(Uint8Array(mem, ptr, len));
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
    data.
}
