function onwasm(ctx, desc, module) {
    if (desc.canvas === undefined) {
        module.init();
    } else {
        module.init(desc.canvas);
    }

    setInterval(module.frame, desc.deltaTime);
}

onmessage = async function(e) {
    let desc = e.data;

    let source = fetch(desc.wasmSourceLocation);
    importScripts(desc.jsSourceLocation);
    source = await source;
    source = await source.arrayBuffer();

    wasm_bindgen(source, desc.memory).then(ctx =>
        onwasm(ctx, desc, wasm_bindgen));
}
