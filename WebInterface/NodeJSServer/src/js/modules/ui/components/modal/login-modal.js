import Modal from './modal';
import '../../../networking/hash';

/**
 * Class to implement a login modal from the parent modal class
 */
export default class LoginModal extends Modal {
  /**
   * Creates necessary elements for login modal
   * @param {Interface} iface Interface for Interactions with other Objects
   * @param {string} serverName Name of the server to connect to
   */
  constructor(iface, serverName) {
    super(serverName);
    this.serverName = serverName;

    iface.addObject(this, 'modal', ['loginFailed', 'close']);
    this.iface = iface;

    let passBox = document.createElement('div');
    let nameBox = document.createElement('div');
    let sendBox = document.createElement('div');

    let passwordLabel = document.createElement('label');
    let passwordInput = document.createElement('input');
    let passwordInvalid = document.createElement('span');
    passwordLabel.setAttribute('for', 'password-input');
    passwordLabel.textContent = 'Passwort:';
    passwordLabel.title = 'Das Passwort des Spiels';
    passwordInput.id = 'password-input';
    passwordInput.type = 'password';
    passwordInput.placeholder = 'Passwort';
    passwordInput.title = 'Das Passwort des Spiels';
    passwordInvalid.className = 'invalid hidden';
    passwordInvalid.textContent = 'Das eingegebene Passwort ist falsch.';

    let nameLabel = document.createElement('label');
    let nameInput = document.createElement('input');
    let nameInvalid = document.createElement('span');
    nameLabel.setAttribute('for', 'name-input');
    nameLabel.textContent = 'Benutzername:';
    nameLabel.title = 'Dein Anzeigename';
    nameInput.id = 'name-input';
    nameInput.type = 'text';
    nameInput.autocomplete = 'on';
    nameInput.placeholder = 'Name';
    nameInput.title = 'Dein Anzeigename';
    nameInvalid.className = 'invalid hidden';
    nameInvalid.textContent =
        'Der eingegebene Nutzername ist bereits vergeben.';

    let sendButton = document.createElement('button');
    sendButton.className = 'btn';
    sendButton.textContent = 'Login';
    sendButton.id = 'login-button';

    passBox.appendChild(passwordLabel);
    passBox.appendChild(passwordInput);
    passBox.appendChild(passwordInvalid);
    nameBox.appendChild(nameLabel);
    nameBox.appendChild(nameInput);
    nameBox.appendChild(nameInvalid);
    sendBox.appendChild(sendButton);

    this.body.appendChild(passBox);
    this.body.appendChild(nameBox);
    this.body.appendChild(sendBox);

    this.nameInput = nameInput;
    this.passwordInput = passwordInput;
    this.loginButton = sendButton;

    this.passwordInvalid = passwordInvalid;
    this.nameInvalid = nameInvalid;

    this.registerLoginBtnEvent();
  }

  /**
   * Method that gets called, if login fails
   * @param {number} result Error Code
   */
  loginFailed(result) {
    if (result == 1) {
      this.invalid('Name');
      this.loginButton.addEventListener('click', this.event);
    } else if (result == 2) {
      this.invalid('Pass');
      this.loginButton.addEventListener('click', this.event);
    } else {
      this.iface.callMethod('notifications', 'show', 'failed',
          'Ein unbekannter Fehler ist aufgetreten');
      this.close();
    }
  }

  /**
   * Registers event to send login, on button press
   */
  registerLoginBtnEvent() {
    this.event = () => {
      this.invalid(); // Remove 'invalid' messages
      this.loginButton.removeEventListener('click', this.event);
      this.userName = this.nameInput.value;
      this.passwordInput.value.getHash()
          .then((result) => {
            this.iface.callMethod('login', 'sendLogin', this.serverName,
                result, this.userName);
          });
    };
    this.loginButton.addEventListener('click', this.event);
  }

  /**
   * Displays text under invalid password / username
   * @param {string} inv Which field to display under (Pass / Name)
   *  Blank inv will hide both
   */
  invalid(inv) {
    this.body.classList.remove('frst-warning');
    this.body.classList.remove('scnd-warning');

    this.passwordInvalid.classList.add('hidden');
    this.nameInvalid.classList.add('hidden');

    this.passwordInput.style.borderColor = 'none';
    this.nameInput.style.borderColor = 'none';

    if (inv == 'Pass') {
      this.body.classList.add('frst-warning');
      this.passwordInvalid.classList.remove('hidden');
      this.passwordInput.style.borderColor = '#ef5350';
    } else if (inv == 'Name') {
      this.body.classList.add('scnd-warning');
      this.nameInvalid.classList.remove('hidden');
      this.nameInput.style.borderColor = '#ef5350';
    }
  }
}
