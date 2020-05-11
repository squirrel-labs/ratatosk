const WORKER_URI = 'scripts/worker.js'
const WEBSOCKET_URI = 'ws://localhost:5001/'
const MESSAGE_ITEM_SIZE = 32;
let SYNCHRONIZATION_MEMORY;
let MESSAGE_QUEUE = null;
let MESSAGE_QUEUE_LENGTH;
let SYNC_MOUSE;
let SYNC_CANVAS_SIZE;
let SYNC_PLAYER_STATE;
let SYNC_OTHER_STATE;
let queue = null;
let params = new URLSearchParams(document.location.search.substring(1));
<<<<<<< HEAD:wasm/site/main.js
//let token = params.get("token");
let token = "Token-63208259";
=======
let token = params.get("token");
token = "Token-42";
>>>>>>> 84ee415... Remove wasm-bindgen:wasm/scripts/main.js
let workers = [];
let memory;  // global for debugging
let ws = new WebSocket(WEBSOCKET_URI, [token, "tuesday"]);
ws.binaryType = 'arraybuffer';
let connected = false

let mousex = 0;
let mousey = 0;


class MessageQueueWriter {
    constructor(pos, size) {
        this.pos = pos;
        this.size = size;
        this.index = 0;
        this._queue = [];
        this._locked = false;
    }
    _write_i32(args) {
        let ptr = (this.pos + MESSAGE_ITEM_SIZE * this.index++) >> 2;
        let iptr = ptr
        Atomics.store(memoryView32, ptr, 1);
        for (let i of args) {
            Atomics.store(memoryView32, ++iptr, i);
        }
        Atomics.store(memoryView32, ptr, 0);
        this.index %= this.size;
        this._dequeue()
    }
    write_i32(task) {
        if (this.pos === null) {
            return;
        }
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
    if (typeof desc.canvas === "undefined") {
        worker.postMessage(desc);
        worker.addEventListener("message", LogicMessage);
    } else worker.postMessage(desc, [desc.canvas]);
}

function spawnModule(module) {
    let worker = new Worker(WORKER_URI);
    postWorkerDescriptor(worker, module);
    return worker;
}
let wasm_module;
function spawnModules(canvas, memory) {
    WebAssembly.compileStreaming(fetch('../gen/client.wasm'))
    .then(compiled => {
        let module = {
            memory: memory,
            compiled: compiled
        };
        spawnModule(module);
        module.canvas = canvas;
        wasm_module = module;
    });
}

function throwMissingOffscreenCanvasSupport() {
    document.write('your browser does not seem to support OffscreenCanvas.');
}

function createCanvas() {
    let canvas = document.getElementById('c');
    if (typeof canvas.transferControlToOffscreen !== 'function')
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
    let mem = 1073741824;//memoryParameters.max_memory;
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

function onresize() {
    Atomics.store(memoryView32, SYNC_CANVAS_SIZE, window.innerWidth);
    Atomics.store(memoryView32, SYNC_CANVAS_SIZE + 1, window.innerHeight);
}

function LogicMessage(e) {
    let x = new Uint32Array(e.data);
    let optcode = x[0];
    if (optcode >= 128) {
        ws.send(x.slice(1));
    } else if (optcode === 0) {
        const id = x[1];
        let ptr = x[2] / 4;
        if (resource_map.has(id)) {
            let buffer = resource_map.get(id);
            let lenght = buffer.byteLength;
            let u32 = new Uint32Array(buffer, 0, lenght >> 2);
            let u8 = new Uint8Array(buffer, lenght & ~3, lenght & 3);
            for (let i of u32) {
                memoryViewU32[ptr++] = i;
            }
            ptr *= 4;
            for (let i of u8) {
                memoryViewU8[ptr++] = i;
            }
            resource_map.delete(id);
            queue.write_i32([8, id]);
        } else {
            console.error("Requested resource not in resource_map, id: " + id)
        }
    } else if (optcode === 1) {
        SYNCHRONIZATION_MEMORY = x[1] >> 2;
        MESSAGE_QUEUE = x[2];
        MESSAGE_QUEUE_LENGTH = x[3];
        SYNC_MOUSE = SYNCHRONIZATION_MEMORY + 1;
        SYNC_CANVAS_SIZE = SYNC_MOUSE + 2;
        SYNC_PLAYER_STATE = SYNC_CANVAS_SIZE + 2
        SYNC_OTHER_STATE = SYNC_PLAYER_STATE + 3
        spawnModule(wasm_module);
        queue = new MessageQueueWriter(MESSAGE_QUEUE, MESSAGE_QUEUE_LENGTH);
    } else if (optcode === 2) {
        if (x[1] === 0) {
            window.addEventListener('input', input);
        } else {
            window.removeEventListener('input', input);
        }
    }
}

resource_map = new Map();
function upload_resource(data) {
    let u32 = new Uint32Array(data, 0, 4);
    resource_map.set(u32[2], data)
    console.log("sending request to allocate " + data.byteLength + " bytes");
    queue.write_i32([7, u32[2], data.byteLength]);
}

let canvas = createCanvas();
if (typeof canvas === 'undefined') throw Error('canvas creation failed');
memory = generateMemory();
let memoryView32 = new Int32Array(memory.buffer);
let memoryViewU32 = new Uint32Array(memory.buffer);
let memoryView8 = new Int8Array(memory.buffer);
let memoryViewU8 = new Uint8Array(memory.buffer);

spawnModules(canvas, memory);

function wakeUpAt(addr) {
    Atomics.notify(memory.buffer, addr, +Infinity);
}

const START_TIME = Date.now();
let last_time = START_TIME;
const fps_sampling_count = 5;
let fps_sampling_n = 0;
async function wakeLogic() {
    if (connected) {
        let x = new Uint32Array(4);
        x[0] = 128;
        x[1] = Atomics.load(memoryView32, SYNC_PLAYER_STATE);
        x[2] = Atomics.load(memoryView32, SYNC_PLAYER_STATE + 1);
        x[3] = Atomics.load(memoryView32, SYNC_PLAYER_STATE + 2);
        //ws.send(x.buffer); TODO
    }

    const t = Date.now() - START_TIME;
    if (++fps_sampling_n >= fps_sampling_count) {
        document.getElementById('lfps').textContent = Math.round(100000 / ((t - last_time) / fps_sampling_count)) / 100;
        last_time = t;
        fps_sampling_n = 0;
    }
    Atomics.store(memoryView32, SYNCHRONIZATION_MEMORY, t);
    Atomics.store(memoryView32, SYNC_MOUSE, Math.floor(mousex));
    Atomics.store(memoryView32, SYNC_MOUSE + 1, Math.floor(mousey));
    Atomics.notify(memoryView32, SYNCHRONIZATION_MEMORY, +Infinity);
}

function setup_ws() {
    ws.addEventListener('open',  () => {
        console.log('ws connection to ' + WEBSOCKET_URI + ' established');
        connected = true;
    });
    ws.addEventListener('error', event => {
        console.error('ws error occured: "' + event + '"');
        connected = false;
    });
    ws.addEventListener('close', event => {
        console.error('ws is closed now: ' + event);
        connected = false;
    });
    ws.addEventListener('message', e => {
        let data = new Uint32Array(e.data, 0, 1);
        let optcode = data[0];
        if (optcode === 10) {
            upload_resource(e.data);
        } else if (optcode === 11) {
            Atomics.store(memoryView32, SYNC_OTHER_STATE, data[1]);
            Atomics.store(memoryView32, SYNC_OTHER_STATE + 1, data[2]);
            Atomics.store(memoryView32, SYNC_OTHER_STATE + 2, data[3]);
        } else {
            console.error("unknown optcode: " + optcode);
        }
    });
}
setup_ws();

function hashCode(str) {
    var hash = 0;
    if (str.length === 0) {
        return hash;
    }
    for (var i = 0; i < str.length; i++) {
        var char = str.charCodeAt(i);
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
    return hashCode(key.code)
}

function input(e) {
    let str = e.data;
    if (e.inputType !== "insertText") return;
    for (var i = 0; i < str.length; i++) {
        queue.write_i32([3, 1, str.charAt(i)]);
    }
    queue.write_i32([1, e.data]);
}

window.addEventListener('resize', onresize);

window.addEventListener('keydown', e => {
    const key = evalKey(e);
    const mod = keyMod(e);
    if (key !== undefined && key !== 0 && mod !== undefined) { queue.write_i32([1, mod, key]); }
});

window.addEventListener('keyup', e => {
    const key = evalKey(e);
    const mod = keyMod(e);
    if (key !== undefined && key !== 0 && mod !== undefined) { queue.write_i32([2, mod, key]); }
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

window.setInterval(wakeLogic, 50);
