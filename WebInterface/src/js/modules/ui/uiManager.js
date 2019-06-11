import About from './collections/about';
import Login from './collections/login';
import Play from './collections/play';

/**
 * Controller class for Page UI
 */
export default class UIManager {
  /**
   * Initializes new UI Manager
   * @param {Interface} iface Interface for inter-object communication
   */
  constructor(iface) {
    this.currentUI = null;

    iface.addObject(this, 'uiMananger', ['initAbout', 'initLogin', 'initPlay']);
    this.iface = iface;
  }

  /**
   * Initializes UI Components of About Page
   */
  initAbout() {
    this.clearComponents();
    this.about = new About(this.iface);
    this.currentUI = 'about';
  }

  /**
   * Initializes UI Components of Login page
   */
  initLogin() {
    this.clearComponents();
    this.login = new Login(this.iface);
    this.currentUI = 'login';
  }

  /**
   * Initializes UI Components of Play page
   */
  initPlay() {
    this.clearComponents();
    this.play = new Play(this.iface);
    this.currentUI = 'play';
  }

  /**
   * Clears currently loaded components
   */
  clearComponents() {
    switch (this.currentUI) {
      case null: return;
      case 'about': this.about = null; break;
      case 'login': this.login = null; break;
      case 'play': this.play = null; break;
    }

    this.currentUI = null;
  }
}
