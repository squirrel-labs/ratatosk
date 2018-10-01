// Showing / Hiding the backdrop menu

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
    this.backdrop = document.getElementById(backdropMenu);
    this.frontLayer = document.getElementById(frontLayer);
    this.menuButton = document.getElementById(menuButton);
  }

  /**
   * Registers all neccessary events
   */
  register() {
    this.registerButtonEvent();
    this.registerFrontLayerEvent();
  }

  /**
   * Reloads the object
   */
  refresh() {
    this.backdrop = document.getElementById(this.ids.backdropMenu);
    this.frontLayer = document.getElementById(this.ids.frontLayer);
    this.menuButton = document.getElementById(this.ids.menuButton);
    this.register();
  }

  /**
   * Registers showing / hiding through menu button
   */
  registerButtonEvent() {
    this.menuButton.addEventListener('click', () => {
      // Hide / Unhide Backdrop Menu
      if (this.backdrop.classList.contains('hidden')) {
        this.backdrop.classList.remove('hidden');
      } else {
        this.backdrop.classList.add('hidden');
      }

      // Set open state for menu button
      if (this.menuButton.classList.contains('open')) {
        this.menuButton.classList.remove('open');
      } else {
        this.menuButton.classList.add('open');
      }
    });
  }

  /**
   * Registers hiding, when clicking on the front layer
   */
  registerFrontLayerEvent() {
    // Hide menu when interacting with front layer
    this.frontLayer.addEventListener('click', () => {
      this.backdrop.classList.add('hidden');
      this.menuButton.classList.remove('open');
    });
  }
}
