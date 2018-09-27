import Backdrop from './modules/ui/backdrop.js';
import * as signalR from "@aspnet/signalr";

let backdrop = new Backdrop('menu', 'front-layer', 'show-menu');
backdrop.register();

console.log('HI')
