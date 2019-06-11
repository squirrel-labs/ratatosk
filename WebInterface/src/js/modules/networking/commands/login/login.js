import Command from '../_command';
import LoginModal from '../../../ui/components/modal/login-modal';

/**
 * Handles login to server
 */
export default class Login extends Command {
  /**
   * Registers interface for communication with other objects
   * @param {Interface} iface
   */
  constructor(iface) {
    super(iface);
    this.registerPublic('login', 'sendLogin', 'showLogin');
    this.refreshing = false;
  }

  /**
   * Shows a login modal
   * @param {String} name
   */
  showLogin(name) {
    new LoginModal(this.iface, name);
  }

  /**
   * Registers login response method
   */
  registerLoginResponse() {
    this.iface.callMethod('networker', 'registerHandler', 'LoginResponse',
        (result) => {
          if (result == 0) {
            this.iface.callMethod('modal', 'close');
            this.iface.callMethod('router', 'routePlay');
            this.iface.callMethod('networker', 'removeHandler',
                'LoginResponse');
          } else {
            this.iface.callMethod('modal', 'loginFailed', result);
          }
        });
  }

  /**
   * Sends a login request
   * @param {string} group Group name to join
   * @param {string} password Password to send as SHA-256 Base64 String
   * @param {string} username Display name to use
   */
  sendLogin(group, password, username) {
    this.registerLoginResponse();
    this.iface.callMethod('networker', 'sendRequest', 'Login',
        (err) => console.error(err), group, username, password);
  }
}
