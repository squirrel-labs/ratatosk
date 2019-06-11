import LoginModal from './login-modal.js';

/**
 * Class for handling the server list
 */
export default class ServerListing {
  /**
   * Creates reference to container
   * @param {string} serverListId ID of the server list div
   * @param {BannerController} notifications Notification Manager
   */
  constructor(serverListId, notifications) {
    this.serverListing = document.getElementById(serverListId);
    this.notifications = notifications;
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
   * @param {ServerClient} serverClient Server Client to handle login
   * @param {array} ui UI Elements to reload after login
   */
  addElements(array, serverClient, ui) {
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
        new LoginModal(name, serverClient, this.notifications, ui);
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
