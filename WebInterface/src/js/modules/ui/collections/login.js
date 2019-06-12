import Backdrop from '../components/backdrop';
import BannerController from '../components/notification-banner';
import ServerListing from '../components/server-listing';

/**
 * UI Loader for login page
 */
export default class Login {
  /**
   * Registers components for login page
   * @param {Interface} iface Interface to enable comm. with notifications
   */
  constructor(iface) {
    this.backdrop = new Backdrop('menu', 'front-layer', 'show-menu');
    this.bannerController = new BannerController(iface, 'notifications',
        'banner-info', 'dismiss-banner', 'notification-amount');
    this.serverListing = new ServerListing(iface, 'server-list',
        'refresh-button');

    this.backdrop.initialize();
    this.bannerController.initialize();
    this.serverListing.initialize();
  }
}
