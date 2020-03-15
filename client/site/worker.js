function onwasm(ctx, desc, module) {
    if (desc.canvas === undefined) {
        module.initialise();
    } else {
        module.initialise(desc.canvas);
    }

    //setInterval(module.frame, desc.deltaTime);
}

onmessage = async function(e) {
    let desc = e.data;

    let source = fetch(desc.wasmSourceLocation);
    importScripts(desc.jsSourceLocation);
    source = await source;
    source = await source.arrayBuffer();

    wasm_bindgen(source, desc.memory).then(ctx => {
        onwasm(ctx, desc, wasm_bindgen);
    });
}
