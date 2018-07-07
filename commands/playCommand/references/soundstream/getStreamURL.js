const https = require('https');

module.exports = function(id, cb){
	https.get('https://api.soundcloud.com/i1/tracks/' + id + '/streams?client_id=x8i900C3Ub1JwHDYgZL7PpstNaqstDJ1', function(res){
		let body = "";
	
		res.on('data', function(data){
			body += data;
		});
	
		res.on('end', function(){
			let obj = JSON.parse(body);
			cb(obj.http_mp3_128_url);
		});
	});
};