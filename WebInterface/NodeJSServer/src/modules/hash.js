/**
 * Creates Base64 String with SHA-256 Hash of given string
 */
String.prototype.getHash = async function() {
  let data = new ArrayBuffer(this.length * 2);
  let bufferView = new Uint16Array(data);
  for (let i = 0; i < this.length; i++) {
    bufferView[i] = this.charCodeAt(i);
  }

  let encrypted = await crypto.subtle.digest('SHA-256', bufferView);
  let byteArray = new Uint8Array(encrypted);
  let base64String = '';

  for (let byte of byteArray) {
    base64String += String.fromCharCode(byte);
  }

  return btoa(base64String);
};
