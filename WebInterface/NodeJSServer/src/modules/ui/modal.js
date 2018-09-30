export default class Modal {
  constructor(titleString) {
    let modalBackground = document.createElement('div');
    let modal = document.createElement('div');
    let title = document.createElement('h1');
    let body = document.createElement('div');

    modalBackground.className = 'modal-container';
    modal.className = 'modal';
    title.className = 'modal-title';
    body.className = 'modal-body'

    title.textContent = titleString;

    modal.appendChild(title);
    modal.appendChild(body);
    modalBackground.appendChild(modal);
    document.body.appendChild(modalBackground);

    this.bg = modalBackground;
    this.modal = modal;
    this.title = title;
    this.body = body;

    this.registerEvents();
  }

  registerEvents() {
    this.modal.addEventListener('click', (e) => {
      e.stopPropagation();
    });

    this.bg.addEventListener('click', () => {
      this.bg.classList.add('hidden');
      this.bg.addEventListener('transitionend', () => {
        document.body.removeChild(this.bg);
      });
    });
  }

  setBodyText(text) {
    this.body.textContent = text;
  }
}
