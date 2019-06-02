const __exports = {};
let wasm;

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
* @param {any} ecanvas
* @returns {void}
*/
function graphics_entry(worker, ecanvas) {
    return wasm.graphics_entry(addHeapObject(worker), addHeapObject(ecanvas));
}
__exports.graphics_entry = graphics_entry

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

function getObject(idx) { return heap[idx]; }

function __widl_f_set_onmessage_DedicatedWorkerGlobalScope(arg0, arg1) {
    getObject(arg0).onmessage = getObject(arg1);
}
__exports.__widl_f_set_onmessage_DedicatedWorkerGlobalScope = __widl_f_set_onmessage_DedicatedWorkerGlobalScope

function __widl_f_data_MessageEvent(arg0) {
    return addHeapObject(getObject(arg0).data);
}
__exports.__widl_f_data_MessageEvent = __widl_f_data_MessageEvent

function isLikeNone(x) {
    return x === undefined || x === null;
}

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

function __widl_f_get_context_OffscreenCanvas(arg0, arg1, arg2, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    try {

        const val = getObject(arg0).getContext(varg1);
        return isLikeNone(val) ? 0 : addHeapObject(val);

    } catch (e) {
        handleError(exnptr, e);
    }
}
__exports.__widl_f_get_context_OffscreenCanvas = __widl_f_get_context_OffscreenCanvas

function __widl_instanceof_WebGL2RenderingContext(idx) { return getObject(idx) instanceof WebGL2RenderingContext ? 1 : 0; }
__exports.__widl_instanceof_WebGL2RenderingContext = __widl_instanceof_WebGL2RenderingContext

function __widl_f_clear_WebGL2RenderingContext(arg0, arg1) {
    getObject(arg0).clear(arg1 >>> 0);
}
__exports.__widl_f_clear_WebGL2RenderingContext = __widl_f_clear_WebGL2RenderingContext

function __widl_f_clear_color_WebGL2RenderingContext(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).clearColor(arg1, arg2, arg3, arg4);
}
__exports.__widl_f_clear_color_WebGL2RenderingContext = __widl_f_clear_color_WebGL2RenderingContext

function __widl_f_post_message_Worker(arg0, arg1, exnptr) {
    try {
        getObject(arg0).postMessage(getObject(arg1));
    } catch (e) {
        handleError(exnptr, e);
    }
}
__exports.__widl_f_post_message_Worker = __widl_f_post_message_Worker

function __wbg_new0_b4c0f6100aa61878() {
    return addHeapObject(new Date());
}
__exports.__wbg_new0_b4c0f6100aa61878 = __wbg_new0_b4c0f6100aa61878

function __wbg_toISOString_580e1bcc780bf968(arg0) {
    return addHeapObject(getObject(arg0).toISOString());
}
__exports.__wbg_toISOString_580e1bcc780bf968 = __wbg_toISOString_580e1bcc780bf968

function __wbg_get_48d637c66043532c(arg0, arg1, exnptr) {
    try {
        return addHeapObject(Reflect.get(getObject(arg0), getObject(arg1)));
    } catch (e) {
        handleError(exnptr, e);
    }
}
__exports.__wbg_get_48d637c66043532c = __wbg_get_48d637c66043532c

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

function __wbindgen_string_get(i, len_ptr) {
    let obj = getObject(i);
    if (typeof(obj) !== 'string') return 0;
    const ptr = passStringToWasm(obj);
    getUint32Memory()[len_ptr / 4] = WASM_VECTOR_LEN;
    return ptr;
}
__exports.__wbindgen_string_get = __wbindgen_string_get

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

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

function __wbindgen_cb_drop(i) {
    const obj = takeObject(i).original;
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return 1;
    }
    return 0;
}
__exports.__wbindgen_cb_drop = __wbindgen_cb_drop

function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}
__exports.__wbindgen_throw = __wbindgen_throw

function __wbindgen_closure_wrapper55(a, b, _ignored) {
    const f = wasm.__wbg_function_table.get(23);
    const d = wasm.__wbg_function_table.get(24);
    const cb = function(arg0) {
        this.cnt++;
        let a = this.a;
        this.a = 0;
        try {
            return f(a, b, addHeapObject(arg0));

        } finally {
            if (--this.cnt === 0) d(a, b);
            else this.a = a;

        }

    };
    cb.a = a;
    cb.cnt = 1;
    let real = cb.bind(cb);
    real.original = cb;
    return addHeapObject(real);
}
__exports.__wbindgen_closure_wrapper55 = __wbindgen_closure_wrapper55

function __wbindgen_object_clone_ref(idx) {
    return addHeapObject(getObject(idx));
}
__exports.__wbindgen_object_clone_ref = __wbindgen_object_clone_ref

function __wbindgen_object_drop_ref(i) { dropObject(i); }
__exports.__wbindgen_object_drop_ref = __wbindgen_object_drop_ref

//!IMPORTANT_STUFF
const WASM_URL = './pkg/webhogg_bg.wasm';

const imports = { './webhogg': __exports };

let res = WebAssembly.instantiateStreaming(fetch(WASM_URL), imports);

let first = true;
let msg = null;
onmessage = function (e) {
    if (first) {
        first = false;
        console.log('got context: ', e.data);
        msg = e.data;
    }
}
while (msg === null) {
    console.log('yay');
    res.then(result => {
        wasm = result.instance.exports;
        console.log('context sndng: ', msg);
        graphics_entry(self, msg);
    });
    break;
}
