import Interface from './modules/interface';
import UIManager from './modules/ui/uiManager';
import Networker from './modules/networking/networker';

const SERVERURL = 'https://kobert.dev/dsa/api/chatHub';

let iface = new Interface();
let uiMan = new UIManager(iface);
uiMan.initPlay();
// Create Network Manager as well

// TODO: Implement login from the play page