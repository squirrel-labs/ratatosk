async function main() {
    let fetchingSource = fetch('bin/webhogg-wasm.wasm');
    let fetchedSource = await fetchingSource;
    let source = await fetchedSource.text();
    //alert(source)
    let workerGraphics = new Worker('pkg/worker-graphics.js');
}

main();
