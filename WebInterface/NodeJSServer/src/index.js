import Backdrop from './modules/ui/backdrop.js';
import * as signalR from "@aspnet/signalr";

let backdrop = new Backdrop('menu', 'front-layer', 'show-menu');
backdrop.register();

const connection = new signalR.HubConnectionBuilder()
    .withUrl("http://89.183.31.151:5000/chatHub")
    .configureLogging(signalR.LogLevel.Information)
    .build();

connection.on('ReceiveMessage', (user, message) => {
  let msg = message.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
  let encodedMsg = user + " says " + msg;
  let li = document.createElement("li");
  li.textContent = encodedMsg;
  document.getElementById('message-list').appendChild(li);
});

document.getElementById('send-button').addEventListener('click', () => {
  let user = document.getElementById("user-input").value;
  let message = document.getElementById("message-input").value;
  connection.invoke("SendMessage", user, message).catch(function (err) {
    return console.error(err.toString());
  });
  event.preventDefault();
});








connection.start().catch(err => console.error(err.toString()));
