import {default as init_r} from './webhogg.js'
import {init_x} from './webhogg.js'

let module = init_r('./pkg/webhogg_bg.wasm');

init_x(module, 'graphics', null);
