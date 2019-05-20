const socket = new WebSocket("ws://localhost:5001", "tuesday");

//var exampleSocket = new WebSocket("ws://192.168.3.42:5001", "tuesday");
socket.onopen = function (event) {
    console.log("Connection established");
    // Display user friendly messages for the successful establishment of connection
    socket.send("42");
};
socket.onmessage = function (event) {
    console.log(event.data);
    AddBrushPoint(JSON.parse(event.data));
    DrawBrush();
    SaveCanvasImage();
    RedrawCanvasImage();
};
window.onbeforeunload = function () {
    socket.onclose = function () { }; // disable onclose handler first
    console.log("Connection terminated");
    socket.close();
};

let canvas;
let context;
let savedImageData;
let dragging = false;
let strokeColor = 'black';
let fillColor = 'black';
let color = 'black';
let lineWidth = 2;
let currentTool = "brush";
let canvasHeight = 1000;
let canvasWidth = 1000;

let usingBrush = false;
let brushPoints = new Array();

class ShapeBoundingBox {
    constructor(left, top, width, height) {
        this.left = left;
        this.top = top;
        this.width = width;
        this.height = height;
    }
}

class DrawPoint {
    constructor(x, y, mouseDown) {
        this.x = x;
        this.y = y;
        this.mouseDown = mouseDown;
    }
}

class MouseDownPos {
    constructor(x, y) {
        this.x = x;
        this.y = y;
    }
}

class Location {
    constructor(x, y) {
        this.x = x;
        this.y = y;
    }
}


let shapeBoundingBox = new ShapeBoundingBox(0, 0, 0, 0);
let mousedown = new MouseDownPos(0, 0);
let loc = new Location(0, 0);

document.addEventListener('DOMContentLoaded', setupCanvas);

function setupCanvas() {
    canvas = document.getElementById('my-canvas');
    context = canvas.getContext('2d');
    context.strokeStyles = strokeColor;
    context.lineWidth = lineWidth;
    canvas.addEventListener("mousedown", ReactToMouseDown);
    canvas.addEventListener("mousemove", ReactToMouseMove);
    canvas.addEventListener("mouseup", ReactToMouseUp);
}


function GetMousePosition(x, y) {
    let canvasSizeData = canvas.getBoundingClientRect();
    return {
        x: (x - canvasSizeData.left) * (canvas.width / canvasSizeData.width),
        y: (y - canvasSizeData.top) * (canvas.height / canvasSizeData.height)
    };
}

function SaveCanvasImage() {
    savedImageData = context.getImageData(0, 0, canvas.width, canvas.height);
}

function RedrawCanvasImage() {
    context.putImageData(savedImageData, 0, 0);
}

function UpdateRubberbandSizeData(location) {
    shapeBoundingBox.width = Math.abs(location.x - mousedown.x);
    shapeBoundingBox.height = Math.abs(location.y - mousedown.y);

    if (location.x > mousedown.x) {
        shapeBoundingBox.left = mousedown.x;
    } else {
        shapeBoundingBox.left = location.x;
    }
    if (location.y > mousedown.y) {
        shapeBoundingBox.top = mousedown.y;
    } else {
        shapeBoundingBox.top = location.y;
    }
}

function AddNetBrushPoint(x, y, mouseDown) {
    let point = new DrawPoint(x, y, mouseDown);
    socket.send(JSON.stringify(point));

    AddBrushPoint(point);
}

function AddBrushPoint(point) {
    brushPoints.push(point);
}

function DrawBrush() {
    for (let i = 1; i < brushPoints.length; i++) {
        context.beginPath();
        if (brushPoints[i].mouseDown) {
            context.moveTo(brushPoints[i - 1].x, brushPoints[i - 1].y);
        } else {
            context.moveTo(brushPoints[i].x - 1, brushPoints[i].y);
        }
        context.lineTo(brushPoints[i].x, brushPoints[i].y)
        context.closePath();
        context.stroke();
    }
}

function UpdateRubberbandOnMove(location) {
    UpdateRubberbandSizeData(location);
    drawRubberbandShape(location);
}

function drawRubberbandShape(location) {
    context.strokeStyle = strokeColor;
    context.fillStyle = fillColor;

    if (currentTool === "brush") {
        DrawBrush();
    } else if (currentTool === "line") {
        context.beginPath();
        context.moveTo(mousedown.x, mousedown.y);
        context.lineTo(location.x, location.y);
        context.closePath();
        context.stroke();
    } else if (currentTool === "rectangle") {
        context.strokeRect(shapeBoundingBox.left, shapeBoundingBox.top,
            shapeBoundingBox.width, shapeBoundingBox.height);
    }
}

function ReactToMouseDown(e) {
    // Change the mouse pointer to a crosshair
    canvas.style.cursor = "crosshair";
    // Store location
    loc = GetMousePosition(e.clientX, e.clientY);
    // Save the current canvas image
    SaveCanvasImage();
    // Store mouse position when clicked
    mousedown.x = loc.x;
    mousedown.y = loc.y;
    // Store that yes the mouse is being held down
    dragging = true;

    if (currentTool === "brush") {
        usingBrush = true;
        AddNetBrushPoint(mousedown.x, mousedown.y);
    }
};

function ReactToMouseMove(e) {
    canvas.style.cursor = "crosshair";
    loc = GetMousePosition(e.clientX, e.clientY);

    if (currentTool === "brush" && dragging && usingBrush) {
        if (loc.x > 0 && loc.x < canvasWidth && loc.y > 0 && loc.y < canvasHeight) {
            AddNetBrushPoint(loc.x, loc.y, true);
        }
        RedrawCanvasImage();
        DrawBrush();
    } else if (dragging) {
        RedrawCanvasImage();
        UpdateRubberbandOnMove(loc);
    }
};

function ReactToMouseUp(e) {
    canvas.style.cursor = "default";
    loc = GetMousePosition(e.clientX, e.clientY);
    RedrawCanvasImage();
    UpdateRubberbandOnMove(loc);
    dragging = false;
    usingBrush = false;

    brushXPoints = new Array();
    brushYPoints = new Array();
    brushDownPos = new Array();
    if (currentTool === "brush") {
        AddBrushPoint(loc.x, loc.y);
    }
}
