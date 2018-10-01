/**
 * Class for controlling the Notification banner
 */
export default class BannerController {
  /**
   * Creates references to objects and hides notification banner
   * @param {string} bannerId ID of Notification Banner
   * @param {string} textP ID of Notification Banner text field
   * @param {string} dismissBtn ID of dismiss button
   * @param {string} notificationBadge ID of badge (# of notifications)
   */
  constructor(bannerId, textP, dismissBtn, notificationBadge) {
    this.banner = document.getElementById(bannerId);
    this.bannerText = document.getElementById(textP);
    this.dismissBtn = document.getElementById(dismissBtn);
    this.notificationBadge = document.getElementById(notificationBadge);
    this.bannerMsgs = [];

    // Hide Banner after JS loading finished
    this.banner.classList.add('hidden');
  }

  /**
   * Registers dismissing via the dismiss button
   */
  register() {
    this.dismissBtn.addEventListener('click', () => {
      this.dismissCurrent();
    });
  }

  /**
   * Pushes a new message to the notification banner and shows it
   * @param {string} name Name to register notification (referenced in hide)
   * @param {string} text Notification text
   */
  show(name, text) {
    let bannerItem = {name, text, 'html': false};
    this.bannerMsgs.push(bannerItem);

    this.update();
  }

  /**
   * Removes notification from banner
   * @param {string} name Name, that the notification was registered under
   */
  hide(name) {
    if (!name) {
      this.bannerMsgs = [];
      this.banner.classList.add('hidden');
    } else {
      for (let i = 0; i < this.bannerMsgs.length; i++) {
        if (this.bannerMsgs[i].name == name) {
          this.bannerMsgs.splice(i, 1);
        }
      }

      this.update();
    }
  }

  /**
   * Dismisses the currently shown message
   */
  dismissCurrent() {
    this.hide(this.current);
  }

  /**
   * Updates the notification banner with the most recent message
   */
  update() {
    if (this.bannerMsgs.length === 0) {
      this.banner.classList.add('hidden');
      return;
    }

    const lastNotification = this.bannerMsgs[this.bannerMsgs.length - 1];
    const name = lastNotification.name;
    const text = lastNotification.text;
    const html = lastNotification.html;
    this.banner.classList.remove('hidden');

    if (html) this.bannerText.innerHTML = text;
    else this.bannerText.innerText = text;

    this.current = name;

    // Update notification badge
    if (this.bannerMsgs.length < 2) {
      this.notificationBadge.classList.add('hidden');
    } else if (this.bannerMsgs.length > 9) {
      this.notificationBadge.classList.remove('hidden');
      this.notificationBadge.textContent = '∞';
    } else {
      this.notificationBadge.classList.remove('hidden');
      this.notificationBadge.textContent = this.bannerMsgs.length.toString();
    }
  }
}
