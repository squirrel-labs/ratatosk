

/**
 * Handles ingame networking;
 */
export default class GameClient {
  /**
   * Defines basic attributes
   * @param {HubConnection} connection Already established connection to the
   *  server
   */
  constructor(connection) {
    this.connection = connection;
  }
}
