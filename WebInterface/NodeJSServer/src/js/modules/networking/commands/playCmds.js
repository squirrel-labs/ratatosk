// import ListServers from './login/listServers';

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
  }

  /**
   * Registers all the available commands
   */
  registerCommands() {
    // this.cmds.push(new ListServers(iface));
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
