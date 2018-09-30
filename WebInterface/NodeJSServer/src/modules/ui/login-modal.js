import Modal from './modal.js';

export default class LoginModal extends Modal {
  constructor(serverName) {
    super('Login: ' + serverName);

    let passBox = document.createElement('div');
    let nameBox = document.createElement('div');

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


    passBox.appendChild(passwordLabel);
    passBox.appendChild(passwordInput);
    nameBox.appendChild(nameLabel);
    nameBox.appendChild(nameInput);

    this.body.appendChild(passBox);
    this.body.appendChild(nameBox);
  }
}
