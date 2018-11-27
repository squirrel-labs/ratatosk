import Command from '../_command';

/**
 * Handles serverList commands
 */
export default class ListServers extends Command {
  /**
   * Registers interface for communication with other objects
   * @param {Interface} iface
   */
  constructor(iface) {
    super(iface);
    this.registerPublic('listServers', 'listServers');
    this.refreshing = false;
  }

  /**
   * Requests server list from the server
   */
  listServers() {
    if (this.refreshing) return; // If already refreshing, no new request

    let listFn = (groups) => {
      // Populate server listing
      this.iface.callMethod('serverListing', 'flushElements');
      this.iface.callMethod('serverListing', 'addElements', groups, this.iface);
      // Unbind network function
      this.iface.callMethod('networker', 'removeHandler', 'ListGroups');
      this.refreshing = false;
    };
    let errorHandler = (err) => {
      this.refreshing = false;
      console.error(err.toString());
    };

    this.iface.callMethod('networker', 'registerHandler',
        'ListGroups', listFn);
    this.iface.callMethod('networker', 'sendRequest',
        'GetGroups', errorHandler);

    this.refreshing = true;
  }
}
