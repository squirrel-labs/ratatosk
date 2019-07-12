workers = [];
gMemory = null;

function exit() {
    for (var worker of workers) {
        worker.terminate();
    }
    //console.clear();
}

async function main() {
    let fetchingSource = fetch('bin/webhogg-wasm.wasm');

    let canvasElement = document.getElementById('c');
    let offCanvas = canvasElement.transferControlToOffscreen();

    let fetchedSource = await fetchingSource;
    source = await fetchedSource.arrayBuffer();

    let sharedMemory = new WebAssembly.Memory({
        initial: 24,
        maximum: 1024,
        shared: true
    });
    gMemory = sharedMemory;
    sharedMemory.buffer = new SharedArrayBuffer(65000);

    const modules = [
        { type: 'graphics',
            source: source,
            canvas: offCanvas,
            memory: sharedMemory,
            dt: 100 },
        { type: 'logic',
            source: source,
            canvas: [],
            memory: sharedMemory,
            dt: 100 },
    ];
    for (var module of modules) {
        let worker = new Worker('pkg/worker.js');
        if (module.type === 'graphics') {
            worker.postMessage(module, [module.canvas]);
        } else {
            worker.postMessage(module);
        }
        workers.push(worker);
    }
}
main();
