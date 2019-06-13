let data = null;

onmessage = function (e) {
    data = e.data;

    importScripts('../bin/webhogg-wasm.js');
    wasm_bindgen(data.source).then(ctx => {
        if (data.type === 'graphics') {
            wasm_bindgen.start_graphics(data.canvas);
            setInterval(wasm_bindgen.loop_graphics, data.dt);
        } else if (data.type === 'logic') {
            wasm_bindgen.start_logic();
            setInterval(wasm_bindgen.loop_logic, data.dt);
        }

    });
}
