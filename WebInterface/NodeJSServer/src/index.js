import Backdrop from './modules/ui/backdrop.js';
import BannerController from './modules/ui/notification-banner.js';
import ServerClient from './modules/server-client.js';
import LoginModal from './modules/ui/login-modal.js'; // TODO: JUST DEBUG

let backdrop = new Backdrop('menu', 'front-layer', 'show-menu');
backdrop.register();

let notifications = new BannerController('notifications',
    'banner-info', 'dismiss-banner');
notifications.register();

let client = new ServerClient('http://89.183.8.51:5000/chatHub', 'server-list', true);
document.getElementById('refresh-button')
    .addEventListener('click', client.loadServers.bind(client));

new LoginModal('The Crew', client);

window.client = client; // TODO: REMOVE, JUST FOR DEBUGGING
