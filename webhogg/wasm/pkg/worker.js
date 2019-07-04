let data = null;

onmessage = function (e) {
    data = e.data;

    importScripts('../bin/webhogg-wasm.js');
    wasm_bindgen.memory = e.memory;
    wasm_bindgen(data.source).then(ctx => {
        //console.log(ctx.memory);
        //console.log(WebAssembly.Module.imports(wasm_bindgen.__wbindgen_wasm_module));
        if (data.type === 'graphics') {
            wasm_bindgen.start_graphics(data.canvas);
            setInterval(function (...x) {
                return wasm_bindgen.loop_graphics(...x);
            }, data.dt);
        } else if (data.type === 'logic') {
            wasm_bindgen.start_logic();
            setInterval(function (...x) {
                return wasm_bindgen.loop_logic(...x);
            }, data.dt);
        }

    });
}
