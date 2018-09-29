import Backdrop from './modules/ui/backdrop.js';
import BannerController from './modules/ui/notification-banner.js';
import ServerClient from './modules/server-client.js'
import Modal from './modules/ui/modal.js'; // TODO: DEBUGGING

let backdrop = new Backdrop('menu', 'front-layer', 'show-menu');
backdrop.register();

let notifications = new BannerController('notifications',
    'banner-info', 'dismiss-banner');
notifications.register();

let client = new ServerClient('http://89.183.117.197:5000/chatHub', 'server-list', true);
document.getElementById('refresh-button')
    .addEventListener('click', client.loadServers.bind(client));

new Modal('Test Titel');
