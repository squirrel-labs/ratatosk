/**
 * Class for handling the server list
 */
export default class ServerListing {
  /**
   * Creates reference to container
   * @param {Interface} iface Interface for comm. with other objects
   * @param {string} serverListId ID of the server list div
   * @param {string} refreshBtnId ID of the refresh btn
   */
  constructor(iface, serverListId, refreshBtnId) {
    this.ids = {serverListId, refreshBtnId};

    iface.addObject(this, 'serverListing', ['flushElements', 'addElements']);
    this.iface = iface;
  }

  /**
   * Initializes Server List DOM Element
   */
  initialize() {
    this.serverListing = document.getElementById(this.ids.serverListId);
    this.refreshBtn = document.getElementById(this.ids.refreshBtnId);
    this.registerEvents();
  }

  /**
   * Registers events associated with server list UI
   */
  registerEvents() {
    this.registerRefreshEvent();
  }

  /**
   * Registers event for pushing the refresh button
   */
  registerRefreshEvent() {
    this.refreshBtn.addEventListener('click', () => {
      this.iface.callMethod('listServers', 'listServers');
    });
  }

  /**
   * Removes all elements currently in the server listing
   */
  flushElements() {
    this.serverListing.innerHTML = '';
  }

  /**
   * Populates servers from a given array of games
   * @param {array} array Array of available games
   */
  addElements(array) {
    for (let server of array) {
      const name = server['name'];
      const playerAmount = server['userCount'];

      let serverDiv = document.createElement('div');
      let nameSpan = document.createElement('span');
      let rightAlignDiv = document.createElement('div');
      let onlineDot = document.createElement('div');
      let playerCountSpan = document.createElement('span');
      let playerCountStaticSpan = document.createElement('span');
      let joinButton = document.createElement('button');
      serverDiv.className = 'server';
      nameSpan.className = 'server-name';
      rightAlignDiv.className = 'right-aligned-items';
      onlineDot.className = 'player-count-dot';
      playerCountSpan.className = 'player-count';
      playerCountStaticSpan.className = 'player-count-static';
      joinButton.className = 'btn join-btn';
      joinButton.id = 'join';
      nameSpan.textContent = name;
      playerCountSpan.textContent = playerAmount;
      playerCountStaticSpan.textContent = 'Spieler online';
      joinButton.textContent = 'Beitreten';
      joinButton.addEventListener('click', () => {
        this.iface.callMethod('login', 'showLogin', name);
      });

      rightAlignDiv.appendChild(onlineDot);
      rightAlignDiv.appendChild(playerCountSpan);
      rightAlignDiv.appendChild(playerCountStaticSpan);
      rightAlignDiv.appendChild(joinButton);
      serverDiv.appendChild(nameSpan);
      serverDiv.appendChild(rightAlignDiv);
      this.serverListing.appendChild(serverDiv);
    }
  }
}
