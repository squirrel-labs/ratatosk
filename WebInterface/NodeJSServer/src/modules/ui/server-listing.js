/**
 * Class for handling the server list
 */
export default class ServerListing {
  /**
   * Creates reference to container
   * @param {string} serverListId ID of the server list div
   */
  constructor(serverListId) {
    this.serverListing = document.getElementById(serverListId);
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
      const playerList = server['users'];
      const playerAmount = playerList.length;

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

      rightAlignDiv.appendChild(onlineDot);
      rightAlignDiv.appendChild(playerCountSpan);
      rightAlignDiv.appendChild(playerCountStaticSpan);
      rightAlignDiv.appendChild(joinButton);
      serverDiv.appendChild(nameSpan);
      serverDiv.appendChild(rightAlignDiv)
      this.serverListing.appendChild(serverDiv);
    }
  }
}
