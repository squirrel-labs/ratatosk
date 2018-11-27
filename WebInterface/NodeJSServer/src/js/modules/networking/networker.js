import * as signalR from '@aspnet/signalr';
import LoginCommands from './commands/loginCmds';
import PlayCommands from './commands/playCmds';

/**
 * Class for communication to server
 */
export default class Networker {
  /**
   * Creates new Networker and connects it to the Interface
   * @param {Interface} iface Interface for communication between objects
   * @param {String} url URL of the server backend
   * @param {Boolean} [debug=false] Should there be debug output
   */
  constructor(iface, url, debug = false) {
    this.url = url;

    // Register in Interface
    iface.addObject(this, 'networker',
        ['sendRequest', 'registerHandler', 'removeHandler']);
    this.iface = iface;

    const connectionBuilder = new signalR.HubConnectionBuilder()
        .withUrl(url);

    if (debug) {
      connectionBuilder.configureLogging(signalR.LogLevel.Debug);
    } else {
      connectionBuilder.configureLogging(signalR.LogLevel.Error);
    }

    this.connection = connectionBuilder.build();
    this.connection.start()
        .then(() => this.iface.callMethod('listServers', 'listServers'))
        .catch((err) => console.error(err.toString()));

    // Initialize refreshing (blocks new refreshes if true)
    this.refreshing = false;
  }

  /**
   * Sending a network request to the server
   * @param {String} methodName Method to call on server
   * @param {function} errorHandler Function to call on error
   * @param {...*} args Arguments to pass to server
   */
  sendRequest(methodName, errorHandler, ...args) {
    this.connection.invoke(methodName, ...args).catch(errorHandler);
  }

  /**
   * Register a new function to be called upon receival of message from server
   * @param {String} name Name of invoked method
   * @param {function} fn function to call with received data
   */
  registerHandler(name, fn) {
    this.connection.on(name, fn);
  }

  /**
   * Removes handler for receiving messages from the server
   * @param {String} name Name of the invoked method
   */
  removeHandler(name) {
    this.connection.off(name);
  }

  /**
   * Initializes Login Commands
   */
  initLogin() {
    this.loginCmd = new LoginCommands(this.iface);
  }

  /**
   * Initializes play commands
   */
  initPlay() {
    this.playCmd = new PlayCommands(this.iface);
  }

  /**
   * Clears all currently registered commands
   */
  clearCommands() {
    if (this.loginCmd) this.loginCmd.destroy();
    if (this.playCmd) this.playCmd.destroy();
  }
}
