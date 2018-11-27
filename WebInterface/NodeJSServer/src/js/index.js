import Interface from './modules/interface';
import UIManager from './modules/ui/uiManager';
import Networker from './modules/networking/networker';

const SERVERURL = 'http://127.0.0.1:5000/chatHub';

let iface = new Interface();
let uiMan = new UIManager(iface);
uiMan.initLogin();

let netMan = new Networker(iface, SERVERURL, true); // TODO: Remove debug flag
netMan.initLogin();
