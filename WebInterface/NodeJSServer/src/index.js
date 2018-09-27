import Backdrop from './modules/ui/backdrop.js';
import * as signalR from "@aspnet/signalr";

let backdrop = new Backdrop('menu', 'front-layer', 'show-menu');
backdrop.register();

const connection = new signalR.HubConnectionBuilder()
    .withUrl("https://89.183.31.151:5001/chatHub")
    .configureLogging(signalR.LogLevel.Information)
    .build();
connection.start().catch(err => console.error(err.toString()));
