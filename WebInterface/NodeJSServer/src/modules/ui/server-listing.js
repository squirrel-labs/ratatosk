export default class ServerListing {
  constructor(serverListId) {
    this.serverListing = document.getElementById(serverListId);
  }

  flushElements() {
    this.serverListing.innerHTML = '';
  }

  addElements(array) {
    for (let server of array) {
      const name = server['name'];
      const playerList = server['users'];
      const playerAmount = playerList.length;

      let serverDiv = document.createElement('div');
      let nameSpan = document.createElement('span');
      let onlineDot = document.createElement('div');
      let playerCountSpan = document.createElement('span');
      let playerCountStaticSpan = document.createElement('span');
      let joinButton = document.createElement('button');
      serverDiv.className = 'server';
      nameSpan.className = 'server-name';
      onlineDot.className = 'player-count-dot';
      playerCountSpan.className = 'player-count';
      playerCountStaticSpan.className = 'player-count-static';
      joinButton.className = 'btn join-btn';
      joinButton.id = 'join';
      nameSpan.textContent = name;
      playerCountSpan.textContent = playerAmount;
      playerCountStaticSpan.textContent = 'Spieler online';
      joinButton.textContent = 'Beitreten';

      serverDiv.appendChild(nameSpan);
      serverDiv.appendChild(onlineDot);
      serverDiv.appendChild(playerCountSpan);
      serverDiv.appendChild(playerCountStaticSpan);
      serverDiv.appendChild(joinButton);
      this.serverListing.appendChild(serverDiv);
    }
  }
}
