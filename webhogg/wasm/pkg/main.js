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

    const modules = [
        ['graphics', source, [offCanvas], 100],
        ['logic', source, [], 1000]
    ];
    for (var module of modules) {
        let worker = new Worker('pkg/worker.js');
        worker.postMessage(module, module[2]);
        workers.push(worker);
    }
}
main();
