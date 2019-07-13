const WORKER_URI = 'site/worker.js'
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
        module.deltaTime = 100;
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

function createCanvas() {
    let canvas = document.getElementById('c');
    canvas = canvas.transferControlToOffscreen();
    return canvas;
}

function generateMemory() {
    // memory pages of 65,536 bytes = 64 KiB
    const MiB = 16;
    const GiB = 1024 * MiB;
    const memoryDescriptor = {
        initial: 64  * MiB,
        maximum: 1   * GiB,
        shared: true
    };
    return new WebAssembly.Memory(memoryDescriptor);
}

(async function() {
    let canvas = createCanvas();
    memory = generateMemory();

    spawnModules(canvas, memory);
})();

// ========= DEBUG FUNCTIONS =========================================

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
