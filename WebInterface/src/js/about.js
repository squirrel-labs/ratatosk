import Interface from './modules/interface';
import UIManager from './modules/ui/uiManager';

let iface = new Interface();
let uiMan = new UIManager(iface);
uiMan.initAbout();
