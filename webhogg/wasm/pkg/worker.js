onmessage = async function (e) {
    importScripts('../bin/webhogg-wasm.js');
    let ctx = await wasm_bindgen(e.data[1]);

    if (e.data[0] === 'graphics')
        ctx.start_graphics();
    else if (e.data[0] === 'logic')
        ctx.start_logic();
}
