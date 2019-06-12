/**
 * Stores an object and it's public methods
 */
class InterfaceAccessor {
  /**
   * Creates new accessor for object with publicMethods being exposed
   * @param {object} object
   * @param {array<String>} publicMethods
   */
  constructor(object, publicMethods) {
    this.object = object;
    this.publicMethods = publicMethods;
  }

  /**
   * Executes method if it is a public method
   * @param {string} method Name of method to call
   * @return {number} 0 success, 1 method not public, 2 method not found
   */
  execute(method, ...args) {
    if (!this.publicMethods.includes(method)) return 1;
    if (typeof this.object[method] != 'function') return 2;

    this.object[method](...args);
    return 0;
  }
}

/**
 * Implements communication between objects
 */
export default class Interface {
  /**
   * Initializes interface
   */
  constructor() {
    this.objects = {};
  }

  /**
   * Adds a new object to array at objKey and assigns public methods
   * @param {object} object Object to reference in Interface
   * @param {String} objKey Key to reference the object under
   * @param {Array} publicMethods Names of public methods
   */
  addObject(object, objKey, publicMethods) {
    if (!this.objects[objKey]) this.objects[objKey] = [];
    this.objects[objKey].push(new InterfaceAccessor(object, publicMethods));
  }

  /**
   * Unregisters object
   * @param {Object} object
   * @param {String} objKey
   */
  removeObject(object, objKey) {
    if (!this.objects[objKey]) return;

    // Remove all instances of object from objKey
    objects[objKey] = objects[objKey].filter(elt => elt.object != object);

    // Remove reference, if none remain
    if (objects[objKey].length == 0) objects[objKey] = undefined;
  }

  /**
   * Calls a method on all objects with the key objKey
   * @param {String} objKey Object Key of objects to call method on
   * @param {String} method Method name to call on the objects
   * @param {...*} args Arguments to pass
   * @return {number} 0 Success, 1 no objects with objKey, 2 method not public
   */
  callMethod(objKey, method, ...args) {
    if (!this.objects[objKey]) return 1;

    let returnCode = 0;
    for (let obj of this.objects[objKey]) {
      if (obj.execute(method, ...args) != 0) returnCode = 2;
    }
    return returnCode;
  }
}
