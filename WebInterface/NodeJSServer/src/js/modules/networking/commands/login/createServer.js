import Command from '../_command';

/**
 * Handles creation of Servers
 */
export default class CreateServer extends Command {
  /**
   * Registers interface for communication with other objects
   * @param {Interface} iface
   */
  constructor(iface) {
    super(iface);
    this.registerPublic('createServer', 'createServer');
    this.refreshing = false;
  }

  /**
   * TODO:
   */
  createServer() {

  }
}
