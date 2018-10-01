/**
 * Parent class to create Modals on the screen
 * Contains no content, as that is implemented by child classes
 */
export default class Modal {
  /**
   * Creates a new modal with a title and empty content
   * @param {string} titleString Title to show at the top of the modal
   */
  constructor(titleString) {
    let modalBackground = document.createElement('div');
    let modal = document.createElement('div');
    let title = document.createElement('h1');
    let body = document.createElement('div');

    modalBackground.className = 'modal-container';
    modal.className = 'modal';
    title.className = 'modal-title';
    body.className = 'modal-body';

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

  /**
   * Register event to close if clicked outside of modal
   * Clicking on the modal itself should not close it though
   */
  registerEvents() {
    this.modal.addEventListener('click', (e) => {
      e.stopPropagation();
    });

    this.bg.addEventListener('click', () => {
      this.close();
    });
  }

  /**
   * Fades modal out and removes it from the flow of the document
   */
  close() {
    this.bg.classList.add('hidden');
    this.bg.addEventListener('transitionend', () => {
      document.body.removeChild(this.bg);
    });
  }

  /**
   * Puts text in the body
   * @param {string} text Text to put into the body
   */
  setBodyText(text) {
    this.body.textContent = text;
  }
}
