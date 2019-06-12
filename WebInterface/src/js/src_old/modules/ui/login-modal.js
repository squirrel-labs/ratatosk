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
   * @param {BannerController} notificationManager Object controlling the main
   *  notification banners
   * @param {array} ui UI elements to call refresh method on after login
   */
  constructor(serverName, serverClient, notificationManager, ui) {
    super(serverName);
    this.serverName = serverName;
    this.serverClient = serverClient;
    this.notificationManager = notificationManager;
    this.pageUI = ui;

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

    this.registerLoginBtn();
  }

  /**
   * Registers event to send login, on button press
   */
  registerLoginBtn() {
    let eventListener;
    let loginCallBack = (result, connection) => {
      console.log(result);
      if (result == 0) {
        this.redirectToPlay(connection);
        this.close();
      } else if (result == 1) {
        this.invalid('Name');
        this.loginButton.addEventListener('click', eventListener);
      } else if (result == 2) {
        this.invalid('Pass');
        this.loginButton.addEventListener('click', eventListener);
      } else {
        this.notificationManager.show('unknownLoginErr',
            'Ein unbekannter Fehler ist aufgetreten');
        this.close();
      }
    };

    eventListener = () => {
      this.invalid(); // Remove 'invalid' messages
      this.loginButton.removeEventListener('click', eventListener);
      this.userName = this.nameInput.value;
      this.passwordInput.value.getHash()
          .then((result) => {
            this.serverClient.sendLogin(this.serverName, result,
                this.userName, loginCallBack);
          });
    };
    this.loginButton.addEventListener('click', eventListener);
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

  /**
   * Loads play site
   * @param {HubConnection} connection Connection to the server
   */
  redirectToPlay(connection) {
    window.history.pushState('object or string', 'Game Page',
        'play#game=' + this.serverName);
    fetch('play').then((response) => {
      response.text().then((htmlString) => {
        // Replace all references, since we're starting one level farther up
        htmlString = htmlString.replace(/\.\.\//g, './');
        htmlString = /<body>((.)|(\n))*<\/body>/g.exec(htmlString)[0];
        htmlString = htmlString.replace(/<script src=".*"><\/script>/, '');
        htmlString = htmlString.replace(
            /<remove_if_redirected>((.)|\n)*?<\/remove_if_redirected>/g, '');
        document.body.innerHTML = htmlString;

        let stylesheet = document.createElement('link');
        stylesheet.rel = 'stylesheet';
        stylesheet.type = 'text/css';
        stylesheet.href = './style/play.css';
        document.head.appendChild(stylesheet);

        for (let ui of this.pageUI) {
          ui.refresh();
        }

        import(/* webpackChunkName: "/playModule" */ '../playModule')
            .then(({default: GameClient}) => {
              let gameClient = new GameClient(this.userName, connection);
              gameClient.registerChat('chat');
            });
      });
    });
  }
}
