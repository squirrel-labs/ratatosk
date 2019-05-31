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
export function game_logic_entry(worker) {
    return wasm.game_logic_entry(addHeapObject(worker));
}
__exports.game_logic_entry = game_logic_entry

/**
* @returns {void}
*/
export function graphics_entry() {
    return wasm.graphics_entry();
}
__exports.graphics_entry = graphics_entry

function getObject(idx) { return heap[idx]; }

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}

function handleError(exnptr, e) {
    const view = getUint32Memory();
    view[exnptr / 4] = 1;
    view[exnptr / 4 + 1] = addHeapObject(e);
}

function __widl_f_post_message_Worker(arg0, arg1, exnptr) {
    try {
        getObject(arg0).postMessage(getObject(arg1));
    } catch (e) {
        handleError(exnptr, e);
    }
}
__exports.__widl_f_post_message_Worker = __widl_f_post_message_Worker

function __wbindgen_string_new(p, l) { return addHeapObject(getStringFromWasm(p, l)); }
__exports.__wbindgen_string_new = __wbindgen_string_new

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

let passStringToWasm;
if (typeof cachedTextEncoder.encodeInto === 'function') {
    passStringToWasm = function(arg) {


        let size = arg.length;
        let ptr = wasm.__wbindgen_malloc(size);
        let offset = 0;
        {
            const mem = getUint8Memory();
            for (; offset < arg.length; offset++) {
                const code = arg.charCodeAt(offset);
                if (code > 0x7F) break;
                mem[ptr + offset] = code;
            }
        }

        if (offset !== arg.length) {
            arg = arg.slice(offset);
            ptr = wasm.__wbindgen_realloc(ptr, size, size = offset + arg.length * 3);
            const view = getUint8Memory().subarray(ptr + offset, ptr + size);
            const ret = cachedTextEncoder.encodeInto(arg, view);

            offset += ret.written;
        }
        WASM_VECTOR_LEN = offset;
        return ptr;
    };
} else {
    passStringToWasm = function(arg) {


        let size = arg.length;
        let ptr = wasm.__wbindgen_malloc(size);
        let offset = 0;
        {
            const mem = getUint8Memory();
            for (; offset < arg.length; offset++) {
                const code = arg.charCodeAt(offset);
                if (code > 0x7F) break;
                mem[ptr + offset] = code;
            }
        }

        if (offset !== arg.length) {
            const buf = cachedTextEncoder.encode(arg.slice(offset));
            ptr = wasm.__wbindgen_realloc(ptr, size, size = offset + buf.length);
            getUint8Memory().set(buf, ptr + offset);
            offset += buf.length;
        }
        WASM_VECTOR_LEN = offset;
        return ptr;
    };
}

function __wbindgen_debug_string(i, len_ptr) {
    const debug_str =
    val => {
        // primitive types
        const type = typeof val;
        if (type == 'number' || type == 'boolean' || val == null) {
            return  `${val}`;
        }
        if (type == 'string') {
            return `"${val}"`;
        }
        if (type == 'symbol') {
            const description = val.description;
            if (description == null) {
                return 'Symbol';
            } else {
                return `Symbol(${description})`;
            }
        }
        if (type == 'function') {
            const name = val.name;
            if (typeof name == 'string' && name.length > 0) {
                return `Function(${name})`;
            } else {
                return 'Function';
            }
        }
        // objects
        if (Array.isArray(val)) {
            const length = val.length;
            let debug = '[';
            if (length > 0) {
                debug += debug_str(val[0]);
            }
            for(let i = 1; i < length; i++) {
                debug += ', ' + debug_str(val[i]);
            }
            debug += ']';
            return debug;
        }
        // Test for built-in
        const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
        let className;
        if (builtInMatches.length > 1) {
            className = builtInMatches[1];
        } else {
            // Failed to match the standard '[object ClassName]'
            return toString.call(val);
        }
        if (className == 'Object') {
            // we're a user defined class or Object
            // JSON.stringify avoids problems with cycles, and is generally much
            // easier than looping through ownProperties of `val`.
            try {
                return 'Object(' + JSON.stringify(val) + ')';
            } catch (_) {
                return 'Object';
            }
        }
        // errors
        if (val instanceof Error) {
        return `${val.name}: ${val.message}
        ${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}
;
const toString = Object.prototype.toString;
const val = getObject(i);
const debug = debug_str(val);
const ptr = passStringToWasm(debug);
getUint32Memory()[len_ptr / 4] = WASM_VECTOR_LEN;
return ptr;
}
__exports.__wbindgen_debug_string = __wbindgen_debug_string

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

let graphics = new Worker('./graphics.js', {type: 'module', credentials: 'include'});

res.then(result => {
    console.log(result);
    wasm = result.instance.exports;
    wasm.game_logic_entry(graphics);
});

