const https = require('https');
const url = 'https://c.xkcd.com/random/comic';

module.exports.getRandom = function(){
	return new Promise(function(resolve, reject){
		https.get(url, function(res){
			if(res.statusCode !== 302){
				return reject(res.statusCode);
			}
			if(!res.headers){
				return reject(res);
			}
			if(!res.headers.location){
				return reject(res.headers);
			}
			
			return resolve(res.headers.location);
		});
	});
}