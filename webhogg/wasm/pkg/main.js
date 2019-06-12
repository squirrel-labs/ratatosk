async function main() {
    let fetchingSource = fetch('bin/webhogg-wasm.wasm');
    let fetchedSource = await fetchingSource;
    source = await fetchedSource.arrayBuffer();

    let workers = [];
    for (var type of ['graphics', 'logic']) {
        let worker = new Worker('pkg/worker.js');
        worker.postMessage([type, source]);
        workers.push(worker);
    }
}
main();
