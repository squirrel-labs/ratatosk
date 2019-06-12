/**
 * Class for adding functionality to backdrop elements
 */
export default class Backdrop {
  /**
   * Registers all important elements in the backdrop
   * @param {string} backdropMenu ID of Backdrop Menu
   * @param {string} frontLayer ID of Front Layer
   * @param {string} menuButton ID of Show / Hide Menu Button
   */
  constructor(backdropMenu, frontLayer, menuButton) {
    this.ids = {backdropMenu, frontLayer, menuButton};
  }

  /**
   * Initializes the components from the ids defined in the constructor
   */
  initialize() {
    this.open = false;
    this.backdrop = document.getElementById(this.ids.backdropMenu);
    this.frontLayer = document.getElementById(this.ids.frontLayer);
    this.menuButton = document.getElementById(this.ids.menuButton);

    this.registerEvents();
  }

  /**
   * Registers all neccessary events
   */
  registerEvents() {
    this.registerButtonEvent();
    this.registerFrontLayerEvent();
  }

  /**
   * Registers showing / hiding through menu button
   */
  registerButtonEvent() {
    this.menuButton.addEventListener('click', () => {
      // Change open state
      this.open = !this.open;

      // Hide / Unhide Backdrop Menu
      this.open ? this.backdrop.classList.remove('hidden') :
          this.backdrop.classList.add('hidden');

      // Set open state for menu button
      this.open ? this.menuButton.classList.add('open') :
          this.menuButton.classList.remove('open');
    });
  }

  /**
   * Registers hiding upon front layer interaction
   */
  registerFrontLayerEvent() {
    this.frontLayer.addEventListener('click', () => {
      if (!this.open) return; // It's already closed

      this.open = false;
      this.backdrop.classList.add('hidden');
      this.menuButton.classList.remove('open');
    });
  }
}
