import * as signalR from '@aspnet/signalr';
import ServerListing from './ui/server-listing.js';

/**
 * Class for communication to server
 */
export default class ServerClient {
  /**
   * Creates new connection
   * @param {string} url URL of server running signalR
   * @param {string} serverListingId HTML ID of server-listing element,
   *    to populate with available games
   * @param {BannerController} notifications Notification Manager
   * @param {array} ui UI Elements to reload on login
   * @param {boolean} [debug=false] Enable debug output?
   */
  constructor(url, serverListingId, notifications, ui, debug = false) {
    this.ui = ui;
    const connectionBuilder = new signalR.HubConnectionBuilder()
        .withUrl(url);

    if (debug) {
      connectionBuilder.configureLogging(signalR.LogLevel.Debug);
    } else {
      connectionBuilder.configureLogging(signalR.LogLevel.Error);
    }

    this.connection = connectionBuilder.build();
    this.connection.start()
        .then(() => this.loadServers()) // Load games list, once connected
        .catch((err) => console.error(err.toString()));

    // Initialize refreshing (blocks new refreshes if true)
    this.refreshing = false;

    this.serverListing = new ServerListing(serverListingId, notifications);
  }

  /**
   * Requests list of avalable games on the server
   */
  loadServers() {
    if (this.refreshing) return; // If already refreshing, no new request

    this.connection.on('ListGroups', (groups) => {
      // Populate server listing
      this.serverListing.flushElements();
      this.serverListing.addElements(groups, this, this.ui);
      this.connection.off('ListGroups');

      this.refreshing = false;
    });

    this.connection.invoke('GetGroups')
        .catch((err) => {
          this.refreshing = false;
          console.error(err.toString());
        });
    this.refreshing = true;
  }

  /**
   * Sends a game creating request to the server
   * @param {string} name Name of the new game
   * @param {string} password Password
   */
  createServer(name, password) {
    // TODO: Create
  }

  /**
   * Sends a login request
   * @param {string} group Group name to join
   * @param {string} password Password to send as SHA-256 Base64 String
   * @param {string} username Display name to use
   * @param {ServerClient~loginCallback} callback Callback function to use
   */
  sendLogin(group, password, username, callback) {
    this.connection.on('LoginResponse', (result) => {
      callback(result, this.connection);
      this.connection.off('LoginResponse');
    });
    this.connection.invoke('Login', group, username, password);
  }
}

/**
 * Callback to call with response to login request
 * @callback ServerClient~loginCallback
 * @param {number} result 0: Success, 1: PasswordError, 2:UsernameTaken,
 *  3:Unknown Error
 * @param {ConnectionHub} connection Connection to the server
 */
