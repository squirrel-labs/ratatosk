import Backdrop from '../components/backdrop';

/**
 * UI Loader for about page
 */
export default class About {
  /**
   * Registers components for about page
   */
  constructor() {
    this.backdrop = new Backdrop('menu', 'front-layer', 'show-menu');
    this.backdrop.initialize();
  }
}
