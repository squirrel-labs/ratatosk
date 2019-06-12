/**
 * Parent Command class which all commands inherit from
 */
export default class Command {
  /**
   * Constructs basic command object
   * @param {Interface} iface Interface to communicate over
   */
  constructor(iface) {
    this.iface = iface;
  }

  /**
   * Registers public command names to interface
   * @param {String} name Name to register under
   * @param {...String} commandNames Names of public commands
   */
  registerPublic(name, ...commandNames) {
    this.iface.addObject(this, name, ['destroy'].concat(commandNames));
  }

  /**
   * Removes from iface
   */
  destroy() {
    this.iface.removeObject(this);
  }
}
