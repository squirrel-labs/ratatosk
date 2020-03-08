const WORKER_URI = 'site/worker.js'
const WEBSOCKET_URI = 'ws://localhost:3000/'
let workers = [];
let memory;  // global for debugging

function postWorkerDescriptor(worker, desc) {
    if (desc.canvas === undefined)
        worker.postMessage(desc);
    else
        worker.postMessage(desc, [desc.canvas]);
}

function spawnModule(type, memory, canvas) {
    let module = {};
    module.type = type;
    module.memory = memory;
    if (type === 'graphics') {
        module.canvas = canvas;
        module.jsSourceLocation = '../gen/graphics.js';
        module.wasmSourceLocation = '../gen/graphics.wasm';
        module.deltaTime = 10;
    } else if (type === 'logic') {
        module.jsSourceLocation = '../gen/logic.js';
        module.wasmSourceLocation = '../gen/logic.wasm';
        module.deltaTime = 100;
    } else return;
    let worker = new Worker(WORKER_URI);
    postWorkerDescriptor(worker, module);
    return worker;
}

function spawnModules(canvas, memory) {
    workers.push(spawnModule('logic', memory));
    workers.push(spawnModule('graphics', memory, canvas));
}

function throwMissingOffscreenCanvasSupport() {
    document.write('your browser does not seem to support OffscreenCanvas.');
}

function createCanvas() {
    let canvas = document.getElementById('c');
    if (canvas.transferControlToOffscreen === undefined)
        return throwMissingOffscreenCanvasSupport();
    try {
        canvas = canvas.transferControlToOffscreen();
    } catch (NS_ERROR_NOT_IMPLEMENTED) {
        return throwMissingOffscreenCanvasSupport();
    } return canvas;
}

function generateMemory() {
    // memory pages of 65,536 bytes = 64 KiB
    const MiB = 16;
    const GiB = 1024 * MiB;
    const memoryDescriptor = {
        initial: 256 * MiB,
        maximum: 1   * GiB,
        shared: true
    };
    return new WebAssembly.Memory(memoryDescriptor);
}

// kill everything
function kill() {
    let i = 0;
    console.log('initiate tribal genocide');
    for (let w of workers) {
        console.log('> kill worker #' + (i++));
        w.terminate();
    }
    console.log('genocide completed');
}

// memory debugging
function mem(addr) {
    return new Uint8Array(memory.buffer.slice(addr, addr + 1))[0];
}

let canvas = createCanvas();
if (canvas === undefined) throw Error('canvas creation failed');
memory = generateMemory();

spawnModules(canvas, memory);

function wakeUpAt(addr) {
    Atomics.notify(memory.buffer, addr, +Infinity);
}

async function wakeLogic() {
    console.log('wake logic');
}

window.setInterval(wakeLogic, 1000);
