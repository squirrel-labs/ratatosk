const __exports = {};
let wasm;

let cachedTextDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

function __wbg_debug_eacd5b227c4c01c7(arg0, arg1, arg2, arg3, arg4, arg5) {
    let varg0 = getStringFromWasm(arg0, arg1);
    let varg2 = getStringFromWasm(arg2, arg3);
    let varg4 = getStringFromWasm(arg4, arg5);
    console.debug(varg0, varg2, varg4);
}
__exports.__wbg_debug_eacd5b227c4c01c7 = __wbg_debug_eacd5b227c4c01c7

function __wbg_info_be654745b6a55079(arg0, arg1, arg2, arg3, arg4, arg5) {
    let varg0 = getStringFromWasm(arg0, arg1);
    let varg2 = getStringFromWasm(arg2, arg3);
    let varg4 = getStringFromWasm(arg4, arg5);
    console.info(varg0, varg2, varg4);
}
__exports.__wbg_info_be654745b6a55079 = __wbg_info_be654745b6a55079

function __wbg_warn_804a0523852c6d10(arg0, arg1, arg2, arg3, arg4, arg5) {
    let varg0 = getStringFromWasm(arg0, arg1);
    let varg2 = getStringFromWasm(arg2, arg3);
    let varg4 = getStringFromWasm(arg4, arg5);
    console.warn(varg0, varg2, varg4);
}
__exports.__wbg_warn_804a0523852c6d10 = __wbg_warn_804a0523852c6d10

function __wbg_error_56a861ecc80f27e1(arg0, arg1, arg2, arg3, arg4, arg5) {
    let varg0 = getStringFromWasm(arg0, arg1);
    let varg2 = getStringFromWasm(arg2, arg3);
    let varg4 = getStringFromWasm(arg4, arg5);
    console.error(varg0, varg2, varg4);
}
__exports.__wbg_error_56a861ecc80f27e1 = __wbg_error_56a861ecc80f27e1

const heap = new Array(32);

heap.fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}
/**
* @param {any} worker
* @returns {void}
*/
function game_logic_entry(worker) {
    return wasm.game_logic_entry(addHeapObject(worker));
}
__exports.game_logic_entry = game_logic_entry

/**
* @param {any} worker
* @returns {void}
*/
function graphics_entry(worker) {
    return wasm.graphics_entry(addHeapObject(worker));
}
__exports.graphics_entry = graphics_entry

function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}
__exports.__wbindgen_throw = __wbindgen_throw

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function __wbindgen_object_drop_ref(i) { dropObject(i); }
__exports.__wbindgen_object_drop_ref = __wbindgen_object_drop_ref

const WASM_URL = './pkg/webhogg_bg.wasm';

const imports = { './webhogg': __exports };
let res = WebAssembly.instantiateStreaming(fetch(WASM_URL), imports);

res.then(result => {
    console.log(result);
    wasm = result.instance.exports;
    wasm.graphics_entry();
});

onmessage = function (e) {
    console.log('gooot messaaage', e);
}
