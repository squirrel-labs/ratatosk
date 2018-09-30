String.prototype.getHash = async function() {
  let data = new ArrayBuffer(this.length * 2);
  let bufferView = new Uint16Array(data);
  for (let i = 0; i < this.length; i++) {
    bufferView[i] = this.charCodeAt(i);
  }

  return await crypto.subtle.digest('SHA-256', bufferView);
}
