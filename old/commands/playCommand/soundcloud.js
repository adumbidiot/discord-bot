const https = require('https');
//Should aquire client id. If a request fails, it should attempt to reaquire BEFORE rjecting a promise
module.exports.getStreamLink = function(url){
	return new Promise(function(resolve, reject){
		https.get(`https://api.soundcloud.com/resolve?url=${url}&client_id=cwK1HUhK9ooDrM8vYSBttf49FL0remHc`, function(response){
			let body = '';
			
			if(response.statusCode === 401 || response.statusCode === 404){
				return reject(`Invalid Status Code: ${response.statusCode}`); //Probably bad client id
			}
			response.on('data', function(data){
				body += data;
			});
		
			response.on('end', function(){
				resolve(JSON.parse(body));
			});
		});
	});
}

module.exports.getSongData = function(url){
	return new Promise(function(resolve, reject){
		https.get(url, function(response){
			let body = '';
			
			response.on('data', function(data){
				body += data;
			});
			
			response.on('end', function(){
				resolve(JSON.parse(body));
			});
		});
	});
}

module.exports.getSongLink = function(url){
	return new Promise(function(resolve, reject){
		https.get(url + '?client_id=cwK1HUhK9ooDrM8vYSBttf49FL0remHc' , function(response){
			let body = '';
			
			response.on('data', function(data){
				body += data;
			});
			
			response.on('end', function(){
				resolve(JSON.parse(body));
			});
		});
	});
}

module.exports.getSongStream = function(url){
	return new Promise(function(resolve, reject){
		https.get(url , function(response){
			resolve(response);
		});
	});
}

module.exports.linkToSong = async function(url){
	let stream_url = await module.exports.getStreamLink(url);
	let song_data = await module.exports.getSongData(stream_url.location);
	let song_link = await module.exports.getSongLink(song_data.stream_url);
	return {song_link, song_data};
}

module.exports.getClientId = function(){
	return 'cwK1HUhK9ooDrM8vYSBttf49FL0remHc'; //TEMP
}
