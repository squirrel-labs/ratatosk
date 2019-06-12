import ListServers from './login/listServers';
import CreateServer from './login/createServer';
import Login from './login/login';

/**
 * Manages commands related to the login page
 */
export default class LoginCommands {
  /**
   * Initializes the login commands
   * @param {Interface} iface Interface for inter-object communication
   */
  constructor(iface) {
    this.iface = iface;
    this.cmds = [];
    this.registerCommands();
  }

  /**
   * Registers all the available commands
   */
  registerCommands() {
    this.cmds.push(new ListServers(this.iface));
    this.cmds.push(new CreateServer(this.iface));
    this.cmds.push(new Login(this.iface));
  }

  /**
   * Destroys all attached commands
   */
  destroy() {
    for (let cmd of this.cmds) {
      cmd.destroy();
    }
  }
}
