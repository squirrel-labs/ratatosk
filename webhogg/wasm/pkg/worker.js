let data = null;

onmessage = function (e) {
    data = e.data;

    importScripts('../bin/webhogg-wasm.js');
    console.log(data.memory);
    wasm_bindgen(data.source, data.memory).then(ctx => {
        console.log('hey wasm');
        if (data.type === 'graphics') {
            wasm_bindgen.start_graphics(data.canvas);
            setInterval(function (...x) {
                //console.log('gmem ', wasm_bindgen.memory);
                /*console.log('mimi graphics', WebAssembly.Module.imports(
                    wasm_bindgen.__wbindgen_wasm_module)
                );*/
                let y = wasm_bindgen.loop_graphics(...x);
                console.log('graphics counter: ', y);
            }, data.dt);
        } else if (data.type === 'logic') {
            wasm_bindgen.start_logic();
            setInterval(function (...x) {
                //console.log('lmem ', wasm_bindgen.memory);
                let z = wasm_bindgen.loop_logic(...x);
                console.log('logic counter: ', z);
            }, data.dt);
        }

    });
}
