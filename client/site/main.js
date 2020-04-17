const WORKER_URI = 'site/worker.js'
const WEBSOCKET_URI = 'ws://localhost:3000/'
// synchronization memory address (read from mem.json, see gen_mem_layout.rs)
const SYNCHRONIZATION_MEMORY = memoryParameters.sync_area / 4;
const MESSAGE_QUEUE = memoryParameters.queue_start;
const MESSAGE_ITEM_SIZE = 32;
const MESSAGE_QUEUE_LENGTH = memoryParameters.queue_size / MESSAGE_ITEM_SIZE;
const WS_URL = "wss://rask.rocks/api"
let params = new URLSearchParams(document.location.search.substring(1));
//let token = params.get("token");
let token = 42;
let workers = [];
let memory;  // global for debugging
let ws = new WebSocket(WS_URL, token);
ws.binaryType = 'arraybuffer';
let connected = false

let mousex = 0;
let mousey = 0;

class MessageQueueWriter {
    constructor(pos, elemetSize) {
        this.pos = pos;
        this.size = elemetSize;
        this.index = 0;
        this._queue = [];
        this._locked = false;
    }
    _write_i32(args) {
        let ptr = Math.floor((this.pos + this.size * this.index++) / 4);
        let iptr = ptr
        console.log(args)
        Atomics.store(memoryView32, ptr, 1);
        for (let i of args) {
            Atomics.store(memoryView32, ++iptr, i);
        }
        Atomics.store(memoryView32, ptr, 0);
        this.index = this.index % MESSAGE_QUEUE_LENGTH;
        this._dequeue()
    }
    write_i32(task) {
        this._queue.push(task);
        if(!this._locked) this._dequeue();
    }
    _dequeue() {
        this._busy = true;
        let next = this._queue.shift();

        if(next)
            this._write_i32(next)
        else
            this._busy = false;
    }
}

queue = new MessageQueueWriter(MESSAGE_QUEUE, MESSAGE_ITEM_SIZE);

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
let memoryViewU32 = new Uint32Array(memory.buffer);
let memoryView8 = new Int8Array(memory.buffer);

spawnModules(canvas, memory);

function wakeUpAt(addr) {
    Atomics.notify(memory.buffer, addr, +Infinity);
}

const START_TIME = Date.now();
async function wakeLogic() {
    if (connected) {
        let x = new UInt32Array(4);
        x[0] = 128;
        x[1] = Atomics.read(memoryView32, SYNCHRONIZATION_MEMORY + 3);
        x[2] = Atomics.read(memoryView32, SYNCHRONIZATION_MEMORY + 4);
        x[3] = Atomics.read(memoryView32, SYNCHRONIZATION_MEMORY + 5);
        ws.post(x.buffer);
    }

    Atomics.store(memoryView32, SYNCHRONIZATION_MEMORY, Date.now() - START_TIME);
    Atomics.store(memoryView32, SYNCHRONIZATION_MEMORY + 1, Math.floor(mousex));
    Atomics.store(memoryView32, SYNCHRONIZATION_MEMORY + 2, Math.floor(mousey));
    Atomics.notify(memoryView32, SYNCHRONIZATION_MEMORY, +Infinity);
}

function upload_resource(data) {

}

function setup_ws() {
    ws.addEventListener('open',  () => {
        add_text('ws connection to ' + WS_URL + 'established');
        connected = true;
    });
    ws.addEventListener('error', event => {
        add_text('ws error occured: "' + event + '"');
        connected = false;
    });
    ws.addEventListener('close', event => {
        add_text('ws is closed now: ' + event);
        connected = false;
    });
    ws.addEventListener('message', event => {
        let data = new UInt32Array(event.data.buffer);
        let type = data[0] & 255;
        if (type == 10) {
            upload_resource(data);
        } else if (type == 11) {
            Atomics.store(memoryView32, SYNCHRONIZATION_MEMORY + 6, data[1]);
            Atomics.store(memoryView32, SYNCHRONIZATION_MEMORY + 7, data[2]);
            Atomics.store(memoryView32, SYNCHRONIZATION_MEMORY + 8, data[3]);
        }
    });
}

String.prototype.hashCode = function() {
    var hash = 0;
    if (this.length == 0) {
        return hash;
    }
    for (var i = 0; i < this.length; i++) {
        var char = this.charCodeAt(i);
        hash = ((hash<<5)-hash)+char;
        hash = hash & hash; // Convert to 32bit integer
    }
    return hash;
}

function keyMod(key) {
    return key.shiftKey + (key.ctrlKey << 1) + (key.altKey << 2) + (key.metaKey << 3);
}

function evalKey(key) {
    if (key.isComposing && key.repeat) { return 0; }
    return key.code.hashCode()
}

window.addEventListener('keydown', e => {
    const key = evalKey(e);
    const mod = keyMod(e);
    if (key !== undefined && mod !== undefined) { queue.write_i32([1, mod, key]); }
});

window.addEventListener('keyup', e => {
    const key = evalKey(e);
    const mod = keyMod(e);
    if (key !== undefined && mod !== undefined) { queue.write_i32([2, mod, key]); }
});

window.addEventListener('mousemove', e => {
    mousex = e.clientX;
    mousey = e.clientY;
});

window.addEventListener('mousedown', e => {
    const mod = keyMod(e);
    if (mod !== undefined) {queue.write_i32([5, (keyMod(e) << 8) | e.buttons, e.clientX, e.clientY]);}
});

window.addEventListener('mouseup', e => {
    const mod = keyMod(e);
    if (mod !== undefined) {queue.write_i32([6, (keyMod(e) << 8) | e.buttons, e.clientX, e.clientY]);}
});

window.addEventListener('resize', () => {
    oncanvas();
    queue.write_i32([7, window.innerWidth, window.innerHeight]);
});

window.setInterval(wakeLogic, 50);
