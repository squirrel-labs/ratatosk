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

onmessage = async function(e) {
    let desc = e.data;

    let source = fetch(desc.wasmSourceLocation);
    importScripts(desc.jsSourceLocation);
    source = await source;
    source = await source.arrayBuffer();

    wasm_bindgen(source, desc.memory).then(ctx => {
        //this.setTimeout(() => {
            onwasm(ctx, desc, wasm_bindgen)
        //}, 2);
    });
}
