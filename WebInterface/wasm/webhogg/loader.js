import {default as init_r} from './webhogg.js'
import {init_x} from './webhogg.js'

let m1 = init_r('./pkg/webhogg_bg.wasm');
let m2 = init_r('./pkg/webhogg_bg.wasm');

init_x(m1, 1);
init_x(m2, 2);
