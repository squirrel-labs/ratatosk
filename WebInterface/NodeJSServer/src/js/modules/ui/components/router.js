/**
 * Class for routing between pages
 */
export default class Router {
  /**
   * @param {Interface} iface Interface for comm. with other objects
   */
  constructor(iface) {
    iface.addObject(this, 'serverListing', ['routePlay']);
    this.iface = iface;
  }

  /**
   * Routes to the play page
   * @param {HubConnection} connection Connection to the server
   */
  routePlay(connection) {
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


        this.iface.callMethod('uiMananger', 'initPlay');
        for (let ui of this.pageUI) {
          ui.refresh();
        }
      });
    });
  }
}
