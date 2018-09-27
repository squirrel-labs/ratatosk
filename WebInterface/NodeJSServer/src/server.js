var express = require('express');
var app = express();
var http = require('http').Server(app);
var io = require('socket.io')(http);

app.use('/ressources', express.static(__dirname + '/ressources/'));

app.get('/', function(req, res){
  res.sendFile(__dirname + '/index.html');
});
app.get('/index.css', function(req, res){
  res.sendFile(__dirname + '/index.css');
});
app.get('/index.js', function(req, res){
  res.sendFile(__dirname + '/index.js');
});

io.on('connection', function(socket){
  console.log('Connection: ' + socket);
});

http.listen(3000, function(){
  console.log('Listening on *:3000');
});
