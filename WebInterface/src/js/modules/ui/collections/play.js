import Backdrop from '../components/backdrop';
import BannerController from '../components/notification-banner';

/**
 * UI Loader for play page
 */
export default class Play {
  /**
   * Registers components for play page
   */
  constructor() {
    this.backdrop = new Backdrop('menu', 'front-layer', 'show-menu');
    this.bannerController = new BannerController(iface, 'notifications',
        'banner-info', 'dismiss-banner', 'notification-amount');

    this.backdrop.initialize();
    this.bannerController.initialize();
  }
}
