import Modal from './modal.js';
import '../hash.js';

/**
 * Class to implement a login modal from the parent modal class
 */
export default class LoginModal extends Modal {
  /**
   * Creates necessary elements for login modal
   * @param {string} serverName Name of the server, used for login and displayed
   *  in title
   * @param {ServerClient} serverClient Server client object used to send the
   *  login
   */
  constructor(serverName, serverClient) {
    super(serverName);
    this.serverName = serverName;
    this.serverClient = serverClient;

    let passBox = document.createElement('div');
    let nameBox = document.createElement('div');
    let sendBox = document.createElement('div');

    let passwordLabel = document.createElement('label');
    let passwordInput = document.createElement('input');
    passwordLabel.setAttribute('for', 'password-input');
    passwordLabel.textContent = 'Passwort:';
    passwordLabel.title = 'Das Passwort des Spiels';
    passwordInput.id = 'password-input';
    passwordInput.type = 'password';
    passwordInput.placeholder = 'Passwort';

    let nameLabel = document.createElement('label');
    let nameInput = document.createElement('input');
    nameLabel.setAttribute('for', 'name-input');
    nameLabel.textContent = 'Benutzername:';
    nameLabel.title = 'Dein Anzeigename';
    nameInput.id = 'name-input';
    nameInput.type = 'text';
    nameInput.autocomplete = 'on';
    nameInput.placeholder = 'Name';

    let sendButton = document.createElement('button');
    sendButton.className = 'btn';
    sendButton.textContent = 'Login';
    sendButton.id = 'login-button';


    passBox.appendChild(passwordLabel);
    passBox.appendChild(passwordInput);
    nameBox.appendChild(nameLabel);
    nameBox.appendChild(nameInput);
    sendBox.appendChild(sendButton);

    this.body.appendChild(passBox);
    this.body.appendChild(nameBox);
    this.body.appendChild(sendBox);

    this.nameInput = nameInput;
    this.passwordInput = passwordInput;
    this.loginButton = sendButton;

    this.registerLoginBtn();
  }

  /**
   * Registers event to send login, on button press
   */
  registerLoginBtn() {
    let eventListener = () => {
      this.loginButton.removeEventListener('click', eventListener);
      let userName = this.nameInput.value;
      this.passwordInput.value.getHash()
          .then((result) => {
            this.serverClient.sendLogin(this.serverName, result, userName);

            // TODO: Wait for response, if error keep window intact,
            // and reenable event listener
            this.close();
          });
    };
    this.loginButton.addEventListener('click', eventListener);
  }
}
