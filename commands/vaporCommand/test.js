let v = require('./vaporwave.js');
console.log(v('hi'));
let http = require('http');
let server = http.createServer();
server.on('request', function(req, res){
	res.setHeader("Content-Type", "text/html; charset=utf-8");
	res.write('<html>');
	res.write('<head>');
	res.write('</head><body><p>');
	res.write(v('hi'));
	res.end('</p></textarea></body></html>');
});

server.listen('80');
