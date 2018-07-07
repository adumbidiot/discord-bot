const soundcloud = require('./soundcloud.js');
const fs = require('fs');

let urls = [
	'https://soundcloud.com/wavey-hefner/welcome-to-the-party',
	'https://soundcloud.com/grey/where-are-u-now-grey-remix',
	'https://soundcloud.com/916frosty/916frosty-enough'
];

test();

async function test(){
	for(let i = 0; i != urls.length; i++){
		await loadAndSave(urls[i]);
	}
}



function loadAndSave(url){
	let song_data = null;
	soundcloud.linkToSong(url).then((data) => {
		console.log(`Song Found at ${data.song_link.location}`);
		song_data = data.song_data;
		return soundcloud.getSongStream(data.song_link.location);
	}).then((stream) => {
		stream.pipe(fs.createWriteStream(`songs/${song_data.title}.mp3`));
	});
}
