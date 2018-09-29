export default class BannerController {
  constructor(bannerId, textP, dismissBtn) {
    this.banner = document.getElementById(bannerId);
    this.bannerText = document.getElementById(textP);
    this.dismissBtn = document.getElementById(dismissBtn);
    this.bannerMsgs = [];

    // Hide Banner after JS loading finished
    this.banner.classList.add('hidden');
  }

  register() {
    this.dismissBtn.addEventListener('click', () => {
      this.dismissCurrent();
    });
  }

  show(name, text) {
    let bannerItem = {name, text, 'html': false};
    this.bannerMsgs.push(bannerItem);

    this.update();
  }

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

  dismissCurrent() {
    this.hide(this.current);
  }

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
  }

  showPersistence() {
    let text = `Storage persistence is disabled, so in-browser storage of created Wikis might not work.\n` +
        `Click <a href="#" onclick="
          event.preventDefault();
          navigator.storage.persist().then((persistent) => {
            if (persistent) notificationManager.show('storageSuccess', 'Storage persistence successfully turned on.');
            else notificationManager.show('storageFail', 'Storage persistence has been rejected.');
          });
        ">here</a> to enable storage persistence.`;
    let bannerItem = {'name': 'persistence', text, 'html': true};
    this.bannerMsgs.push(bannerItem);
    this.update();
  }
}
