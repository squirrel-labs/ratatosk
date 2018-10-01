import Backdrop from './modules/ui/backdrop.js';
import BannerController from './modules/ui/notification-banner.js';
import ServerClient from './modules/server-client.js';

let backdrop = new Backdrop('menu', 'front-layer', 'show-menu');
backdrop.register();

let notifications = new BannerController('notifications',
    'banner-info', 'dismiss-banner', 'notification-amount');
notifications.register();

let client = new ServerClient('http://89.183.101.117:5000/chatHub',
    'server-list', notifications, [backdrop, notifications], true);
document.getElementById('refresh-button')
    .addEventListener('click',
        client.loadServers.bind(client));

window.client = client; // TODO: REMOVE, JUST FOR DEBUGGING
