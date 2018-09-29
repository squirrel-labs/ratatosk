import Backdrop from './modules/ui/backdrop.js';
import * as signalR from '@aspnet/signalr';
import BannerController from './modules/ui/notification-banner.js';

let backdrop = new Backdrop('menu', 'front-layer', 'show-menu');
backdrop.register();

let notifications = new BannerController('notifications',
    'banner-info', 'dismiss-banner');
notifications.register();

const connection = new signalR.HubConnectionBuilder()
    .withUrl("http://89.183.117.197:5000/chatHub")
    .configureLogging(signalR.LogLevel.Information)
    .build();

connection.on('ReceiveMessage', (user, message) => {
  let msg = message.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
  let encodedMsg = user + " says " + msg;
  let li = document.createElement("div");
  li.classList.add('server');
  li.textContent = encodedMsg;
  document.getElementById('server-list').appendChild(li);
});

document.getElementById('new-game-button').addEventListener('click', () => {
  let method = window.prompt('Please enter method:', 'SendMessage');
  let user = window.prompt('Please enter user:', 'Default');
  let message = window.prompt('Please enter message:', 'Super duper Nachricht');
  connection.invoke(method, user, message).catch(function (err) {
    return console.error(err.toString());
  });
  event.preventDefault();
}),

connection.start()
    .then(() => console.log('Connected'))
    .catch(err => console.error(err.toString()));
