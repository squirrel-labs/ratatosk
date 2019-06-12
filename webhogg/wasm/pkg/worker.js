onmessage = async function (e) {
    importScripts('../bin/webhogg-wasm.js');
    let type = e.data[0];
    let source = e.data[1];
    let args = e.data[2];
    let dt = e.data[3];
    let ctx = await wasm_bindgen(source);

    ctx['start_' + type].apply(args);
    setInterval(ctx['loop_' + type], dt);
}
