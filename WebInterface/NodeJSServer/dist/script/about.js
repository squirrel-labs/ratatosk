!function(t){var e={};function n(r){if(e[r])return e[r].exports;var o=e[r]={i:r,l:!1,exports:{}};return t[r].call(o.exports,o,o.exports,n),o.l=!0,o.exports}n.m=t,n.c=e,n.d=function(t,e,r){n.o(t,e)||Object.defineProperty(t,e,{enumerable:!0,get:r})},n.r=function(t){"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(t,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(t,"__esModule",{value:!0})},n.t=function(t,e){if(1&e&&(t=n(t)),8&e)return t;if(4&e&&"object"==typeof t&&t&&t.__esModule)return t;var r=Object.create(null);if(n.r(r),Object.defineProperty(r,"default",{enumerable:!0,value:t}),2&e&&"string"!=typeof t)for(var o in t)n.d(r,o,function(e){return t[e]}.bind(null,o));return r},n.n=function(t){var e=t&&t.__esModule?function(){return t.default}:function(){return t};return n.d(e,"a",e),e},n.o=function(t,e){return Object.prototype.hasOwnProperty.call(t,e)},n.p="../script/",n(n.s=0)}([function(t,e,n){"use strict";n.r(e);new class{constructor(t,e,n){this.ids={backdropMenu:t,frontLayer:e,menuButton:n},this.backdrop=document.getElementById(t),this.frontLayer=document.getElementById(e),this.menuButton=document.getElementById(n)}register(){this.registerButtonEvent(),this.registerFrontLayerEvent()}refresh(){this.backdrop=document.getElementById(this.ids.backdropMenu),this.frontLayer=document.getElementById(this.ids.frontLayer),this.menuButton=document.getElementById(this.ids.menuButton),this.register()}registerButtonEvent(){this.menuButton.addEventListener("click",()=>{this.backdrop.classList.contains("hidden")?this.backdrop.classList.remove("hidden"):this.backdrop.classList.add("hidden"),this.menuButton.classList.contains("open")?this.menuButton.classList.remove("open"):this.menuButton.classList.add("open")})}registerFrontLayerEvent(){this.frontLayer.addEventListener("click",()=>{this.backdrop.classList.add("hidden"),this.menuButton.classList.remove("open")})}}("menu","front-layer","show-menu").register()}]);