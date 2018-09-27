// Showing / Hiding the backdrop menu

export default class Backdrop {
  constructor(backdropMenu, frontLayer, menuButton) {
    this.backdrop = document.getElementById(backdropMenu);
    this.frontLayer = document.getElementById(frontLayer);
    this.menuButton = document.getElementById(menuButton);
  }


  register() {
    this.registerButtonEvent();
    this.registerFrontLayerEvent();
  }

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

  registerFrontLayerEvent() {
    // Hide menu when interacting with front layer
    this.frontLayer.addEventListener('click', () => {
      this.backdrop.classList.add('hidden');
      this.menuButton.classList.remove('open');
    });
  }
}
