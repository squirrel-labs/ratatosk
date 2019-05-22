console.log('js> create import object');
let importObject = { imports: { imported_func: arg => console.log(arg) } };

console.log('js> create fetch object');

let asm_paint_bg = fetch('asm_paint_bg.wasm');

console.log('js> instantiate streaming');

function and_then(obj) {
   console.log('js> reached instantiate streaming\'s then');
   return obj.instance.exports.exported_func();
}

WebAssembly.instantiateStreaming(asm_paint_bg, importObject)
           .then(and_then);

