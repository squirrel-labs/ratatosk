let canvas = document.getElementById('canvas');

let game_logic = new Worker(
    './game_logic.js',
    {type: 'module', credentials: 'include'}
);

let ofc = canvas.transferControlToOffscreen();
game_logic.postMessage({canvas: ofc}, [ofc]);
