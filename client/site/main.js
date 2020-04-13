const WORKER_URI = 'site/worker.js'
const WEBSOCKET_URI = 'ws://localhost:3000/'
// synchronization memory address (see client/shared/src/mem.rs)
const SYNCHRONIZATION_MEMORY = 0x50fc00 / 4;
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
    const page = 65536;
    let mem = memoryParameters.max_memory;
    let max = Math.floor((mem + page -1 ) / page);
    const memoryDescriptor = {
        initial: max,
        maximum: max,
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
let memoryView32 = new Int32Array(memory.buffer);

spawnModules(canvas, memory);

function wakeUpAt(addr) {
    Atomics.notify(memory.buffer, addr, +Infinity);
}

const START_TIME = Date.now();
async function wakeLogic() {
    Atomics.store(memoryView32, SYNCHRONIZATION_MEMORY, Date.now() - START_TIME);
    Atomics.notify(memoryView32, SYNCHRONIZATION_MEMORY, +Infinity);
}

const KEYDOWN = 0x0101;
const KEYUP   = 0x0102;

function sendEvent(e) {
}

function getKey(val) {
    let code = new Uint8Array(4);
    for (let i = 0; i < 4; i++) {
        code[i] = val.charCodeAt(i);
    }
    return code;
}

function keyMod(key) {
    return +key.shiftKey + (key.ctrlKey << 1) + (key.altKey << 2) + (key.metaKey << 3);
}

function evalKey(key) {
    if (key.isComposing && key.repeat) { return; }
    const scode = key.code;
    if (scode.startsWith('Key') && scode.length == 4) { return getKey(scode); }
    else if (scode.startsWith('Digit') && scode.length == 6) { return getKey('Key' + scode[5]); }
    else if (scode.startsWith('Numpad')) { return getKey('Num' + scode[6]); }
    else {
        switch (scode) {
            case 'Minus': return getKey('Key-');
            case 'Plus': return getKey('Key+');
            case 'BracketLeft': return getKey('Key[');
            case 'BracketRight': return getKey('Key]');
            case 'Enter': return getKey('Key\n');
            case 'Backspace': return getKey('Bcks');
            case 'Tab': return getKey('Tabu');
            case 'ControlLeft': return getKey('CtrL');
            case 'ControlRight': return getKey('CtrR');
            case 'ShiftLeft': return getKey('ShiL');
            case 'ShiftRight': return getKey('ShiR');
            case 'ArrowUp': return getKey('ArrU');
            case 'ArrowDown': return getKey('ArrD');
            case 'ArrowLeft': return getKey('ArrL');
            case 'ArrowRight': return getKey('ArrR');
            case 'Equal': return getKey('Key=');
            case 'Unidentified': return;
            default: return getKey(scode.padEnd(4, ' ').substr(0, 4));
        }
    }
}

window.addEventListener('keydown', function(e) {
    const key = evalKey(e);
    const mod = keyMod(e);
    if (key !== undefined && mod !== undefined) { sendEvent([KEYDOWN, key, mod]); }
});

window.addEventListener('keyup', function(e) {
    const key = evalKey(e);
    const mod = keyMod(e);
    if (key !== undefined && mod !== undefined) { sendEvent([KEYUP, key, mod]); }
});

window.setInterval(wakeLogic, 100);
