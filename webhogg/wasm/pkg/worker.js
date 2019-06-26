let data = null;

onmessage = function (e) {
    data = e.data;

    importScripts('../bin/webhogg-wasm.js');
    wasm_bindgen.memory = data.memory;
    wasm_bindgen(data.source, data.memory).then(ctx => {
        console.log(ctx.memory);
        if (data.type === 'graphics') {
            wasm_bindgen.start_graphics(data.canvas);
            setInterval(function (...x) {
                console.log('gmem ', wasm_bindgen.memory);
                /*console.log('mimi graphics', WebAssembly.Module.imports(
                    wasm_bindgen.__wbindgen_wasm_module)
                );*/
                return wasm_bindgen.loop_graphics(...x);
            }, data.dt);
        } else if (data.type === 'logic') {
            wasm_bindgen.start_logic();
            setInterval(function (...x) {
                console.log('lmem ', wasm_bindgen.memory);
                return wasm_bindgen.loop_logic(...x);
            }, data.dt);
        }

    });
}
