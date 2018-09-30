import * as signalR from '@aspnet/signalr';
import ServerListing from './ui/server-listing.js';

export default class ServerClient {
  constructor(url, serverListingId, debug = false) {
    const connectionBuilder = new signalR.HubConnectionBuilder()
        .withUrl(url)

    if (debug) {
      connectionBuilder.configureLogging(signalR.LogLevel.Debug);
    } else {
      connectionBuilder.configureLogging(signalR.LogLevel.Error);
    }

    this.connection = connectionBuilder.build();
    this.connection.start()
        .then(() => this.loadServers())
        .catch(() => err => console.error(err.toString()));

    this.refreshing = false;

    this.serverListing = new ServerListing(serverListingId);

    this.messageHandling(); //TODO: REMOVE, JUST FOR DEBUGGING
  }

  loadServers() {
    if (this.refreshing) return;
    this.connection.on('ListGroups', (groups) => {
      console.log(groups)
      this.serverListing.flushElements();
      this.serverListing.addElements(groups);
      this.connection.off('ListGroups');

      this.refreshing = false;
    });

    this.connection.invoke('GetGroups')
        .catch(err => {
          this.refreshing = false;
          console.error(err.toString())
        });
    this.refreshing = true;
  }

  createServer(){
    // TODO: Create
  }

  sendLogin(group, password, username){
    this.connection.invoke('Login', group, username, password);
  }

  messageHandling(){
    this.connection.on('ReceiveMessage', (user, message) => {
      let msg = message.replace(/&/g, "&amp;")
          .replace(/</g, "&lt;")
          .replace(/>/g, "&gt;");
      let encodedMsg = user + " sagt:  " + msg;
      console.log(encodedMsg); //TODO: REMOVE, JUST FOR DEBUGGING
    });
  }
}



// connection.on('ReceiveMessage', (user, message) => {
//   let msg = message.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
//   let encodedMsg = user + " says " + msg;
//   let li = document.createElement("div");
//   li.classList.add('server');
//   li.textContent = encodedMsg;
//   document.getElementById('server-list').appendChild(li);
// });
