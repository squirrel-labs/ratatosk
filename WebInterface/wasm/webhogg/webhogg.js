
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
/**
* @returns {void}
*/
export function game_logic_entry() {
    return wasm.game_logic_entry();
}
__exports.game_logic_entry = game_logic_entry

/**
* @returns {void}
*/
export function graphics_entry() {
    return wasm.graphics_entry();
}
__exports.graphics_entry = graphics_entry

function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}
__exports.__wbindgen_throw = __wbindgen_throw

function init_r(module) {
    let result;
    const imports = { './webhogg': __exports };

    if (module instanceof URL || typeof module === 'string' || module instanceof Request) {

        const response = fetch(module);
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            result = WebAssembly.instantiateStreaming(response, imports)
            .catch(e => {
                console.warn("`WebAssembly.instantiateStreaming` failed. Assuming this is because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
                return response
                .then(r => r.arrayBuffer())
                .then(bytes => WebAssembly.instantiate(bytes, imports));
            });
        } else {
            result = response
            .then(r => r.arrayBuffer())
            .then(bytes => WebAssembly.instantiate(bytes, imports));
        }
    } else {

        result = WebAssembly.instantiate(module, imports)
        .then(result => {
            if (result instanceof WebAssembly.Instance) {
                return { instance: result, module };
            } else {
                return result;
            }
        });
    }
    return result;
}

function _init_x(result, bx) {
    return result.then(({instance, module}) => {
        wasm = instance.exports;
        if (bx == 1) wasm.game_logic_entry();
        else         wasm.graphics_entry();
        //init_r.__wbindgen_wasm_module = module;
        //wasm.__wbindgen_start();
        return wasm;
    });
}

export default init_r;
export var init_x = _init_x;
