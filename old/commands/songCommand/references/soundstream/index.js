const getID = require('./getID.js');
const getStreamURL = require('./getStreamURL.js');
const https = require('https');
const fs = require('fs');

let mp3 = fs.createWriteStream('./hey.mp3');

let a = function(data){
	console.log(data);
	https.get(data, function(res){
		let body = "";
		/*res.on('data', function(data){
			body += data;
		});
	
		res.on('end', function(){
			console.log(body);
		});*/
		res.pipe(mp3);	
	});
}
let url1 = 'https://soundcloud.com/grey/where-are-u-now-grey-remix';
let url2 = 'https://soundcloud.com/l1lkleenex/leanonyobitch';


getID(url1, function(id){
	console.log("ID:" + id);
	getStreamURL(id, function(data){
		a(data);
	});
});

