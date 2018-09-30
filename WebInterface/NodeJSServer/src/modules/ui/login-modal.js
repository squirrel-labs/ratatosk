import Modal from './modal.js';
import '../hash.js';

export default class LoginModal extends Modal {
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
    passwordLabel.title = 'Das Passwort des Spiels'
    passwordInput.id = 'password-input';
    passwordInput.type = 'password';
    passwordInput.placeholder = 'Passwort';

    let nameLabel = document.createElement('label');
    let nameInput = document.createElement('input');
    nameLabel.setAttribute('for', 'name-input');
    nameLabel.textContent = 'Benutzername:';
    nameLabel.title = 'Dein Anzeigename'
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

  registerLoginBtn() {
    this.loginButton.addEventListener('click', () => {
      console.log('button pressed')
      let userName = this.nameInput.value;
      this.passwordInput.value.getHash()
          .then((result) => {
            this.serverClient.sendLogin(this.serverName, result, userName);
            this.close();
          });
    });
  }
}
