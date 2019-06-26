workers = [];

function exit() {
    for (var worker of workers) {
        worker.terminate();
    }
    console.clear();
}

async function main() {
    let fetchingSource = fetch('bin/webhogg-wasm.wasm');

    let canvasElement = document.getElementById('c');
    let offCanvas = canvasElement.transferControlToOffscreen();

    let fetchedSource = await fetchingSource;
    source = await fetchedSource.arrayBuffer();

    let sharedMemory = new WebAssembly.Memory({
        initial: 0,
        maximum: 65536,
        shared: true
    });
    sharedMemory.buffer = new SharedArrayBuffer();
    //sharedMemory = 'haah enaude';

    const modules = [
        { type: 'graphics',
            source: source,
            canvas: offCanvas,
            memory: sharedMemory,
            dt: 16 },
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
    console.log(sharedMemory.buffer);
}
main();
