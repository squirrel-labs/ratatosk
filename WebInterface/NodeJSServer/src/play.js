import Backdrop from './modules/ui/backdrop.js';
import BannerController from './modules/ui/notification-banner.js';

// TODO: Implement login from the play page

let backdrop = new Backdrop('menu', 'front-layer', 'show-menu');
backdrop.register();

let notifications = new BannerController('notifications',
    'banner-info', 'dismiss-banner', 'notification-amount');
notifications.register();
