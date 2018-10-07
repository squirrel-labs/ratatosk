// TODO: Handle disconnect

/**
 * Handles ingame networking;
 */
export default class GameClient {
  /**
   * Defines basic attributes
   * @param {string} user The username of the player
   * @param {HubConnection} connection Already established connection to the
   *  server
   */
  constructor(user, connection) {
    this.user = user;
    this.connection = connection;
  }

  /**
   * Registers chat html component
   * @param {string} chatId Id of chat component
   */
  registerChat(chatId) {
    this.chat = document.getElementById(chatId);
    this.messageList = this.chat.querySelector('#message-list');
    this.messageInput = this.chat.querySelector('#input-message');
    this.messageSend = this.chat.querySelector('#send-message');

    this.connection.on('ReceiveMessage', (user, message) => {
      let msg = message.replace(/&/g, '&amp;')
          .replace(/</g, '&lt;')
          .replace(/>/g, '&gt;');
      let encodedMsg = user + ' sagt:  ' + msg;

      let messageP = document.createElement('p');
      messageP.class = 'message';
      messageP.textContent = encodedMsg;

      this.messageList.appendChild(messageP);
    });

    this.messageSend.addEventListener('click', () => {
      let message = this.messageInput.value;
      this.connection.invoke('SendMessage', this.user, message);
    });
  }
}
