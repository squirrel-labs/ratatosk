/**
 * Object containing a message for the notification banner
 */
class BannerItem {
  /**
   * Creates new Banner Message Items
   * @param {String} name Name the message will be referenced under
   * @param {String} content Content, either formatted as plain text or html
   * @param {Boolean} html Is content formatted as html?
   */
  constructor(name, content, html) {
    this.name = name;
    this.content = content;
    this.html = html;
  }
}

/**
 * Class for controlling the Notification banner
 */
export default class BannerController {
  /**
   * Creates references to objects and hides notification banner
   * @param {Interface} iface Interface to receive comm. from
   * @param {string} bannerId ID of Notification Banner
   * @param {string} textP ID of Notification Banner text field
   * @param {string} dismissBtn ID of dismiss button
   * @param {string} badge ID of badge (# of notifications)
   */
  constructor(iface, bannerId, textP, dismissBtn, badge) {
    iface.addObject(this, 'notifications', ['show', 'hide']);
    this.iface = iface;

    this.ids = {bannerId, textP, dismissBtn, badge};
  }

  /**
   * Initializes the Banner in the DOM
   */
  initialize() {
    this.banner = document.getElementById(this.ids.bannerId);
    this.bannerText = document.getElementById(this.ids.textP);
    this.dismissBtn = document.getElementById(this.ids.dismissBtn);
    this.notificationBadge = document.getElementById(this.ids.badge);
    this.bannerMsgs = [];

    this.banner.classList.add('hidden'); // Hide banner by default
    this.registerEvents();
  }

  /**
   * Registers events for notification banner
   */
  registerEvents() {
    this.registerDismissEvent();
  }

  /**
   * Registers dismissing via dismiss button
   */
  registerDismissEvent() {
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
    let bannerItem = new BannerItem(name, text, false);
    this.bannerMsgs.push(bannerItem);

    this.update();
  }

  /**
   * Removes notification from banner
   * @param {string} name The name the notification was registered under
   */
  hide(name) {
    if (name) this.bannerMsgs = this.bannerMsgs.filter(elt => elt.name != name);
    else this.bannerMsgs = [];

    this.update();
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
    const text = lastNotification.content;
    const isHtml = lastNotification.html;
    this.banner.classList.remove('hidden');

    if (isHtml) this.bannerText.innerHTML = text;
    else this.bannerText.innerText = text;

    this.current = name;
    this.updateNotificationBadge();
  }

  /**
   * Updates the notification badge number
   */
  updateNotificationBadge() {
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
