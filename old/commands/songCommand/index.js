const {Command, HelpData} = require('../Command.js');
const ytdl = require('ytdl-core');
const soundcloud = require('./soundcloud.js');

async function joinChannel(bot, msg){
	if(!msg.member.voiceChannel){
		msg.reply('Must be in a voice channel!');
		return;
	}
	
	msg.reply('Attempting to join voice channel...');
	try{
		let connection = await msg.member.voiceChannel.join();
		msg.reply('Joined voice channel');
		return connection;
	}catch(e){
		bot.log.error(e);
		return null;
	}
}

async function leaveChannel(bot, msg){
	
}

async function playStream(stream){
	const streamOptions = { seek: 0, volume: 1 };
	const dispatcher = conn.playStream(stream, streamOptions);
	
	dispatcher.on('end', function(){
		conn.disconnect();
	});
			
	dispatcher.on('error', function(err){
		bot.log.error(err);
	});
			
	dispatcher.on('start', function(){
		msg.reply('Starting stream from ' + url);
	});
			
	conn.on('warn', function(err){
		bot.log.warn(err);
	});
}

async function playYoutube(bot, msg, command){
	let conn = bot.client.voiceConnections.get(msg.guild.id);
	if(!conn){
		conn = await joinChannel(bot, msg, command);
	}
	let url = command[2];
	const stream = ytdl(url, { filter : 'audioonly' });
	playStream(stream);
}

async function playSoundcloud(bot, msg, command){
	let conn = bot.client.voiceConnections.get(msg.guild.id);
	if(!conn){
		conn = await joinChannel(bot, msg, command);
	}
	
	let url = command[2];
	let songData = await soundcloud.linkToSong(url);
	const stream = await soundcloud.getSongStream(songData.song_link.location);
	playStream(stream);
}

async function play(bot, msg, command){
	let url = new URL(command[2]);
	let host = url.hostname;
	
	let conn = bot.client.voiceConnections.get(msg.guild.id);
	if(!conn){
		conn = await joinChannel(bot, msg, command);
	}
	
	let stream = null;
	switch(host){
		case 'soundcloud.com': {
			let songData = await soundcloud.linkToSong(url);
			stream = await soundcloud.getSongStream(songData.song_link.location);
			break;
		}
		case 'youtube.com': {
			stream = ytdl(url, { filter : 'audioonly' });
			break;
		}
		default: {
			msg.reply('Unknown Host!');
		}
	}
	playStream(stream);
}

module.exports = new Command({
	onMessageTest(bot, msg){
		return msg.content.startsWith('!song');
	},
	onMessage(bot, msg){
		let command = msg.content.split(' ');
		switch(command[1]){
			case 'join': {
				joinChannel(bot, msg, command).catch((e) => {
					bot.log.error(e);
				});
				break;
			}
			case 'play-youtube': {
				playYoutube(bot, msg, command).catch((e) => {
					bot.log.error(e);
				});
				break;
			}
			case 'play-soundcloud': {
				playSoundcloud(bot, msg, command).catch((e) => {
					bot.log.error(e);
				});
				break;
			}
			case 'play': {
				play(bot,msg, command).catch((e) => {
					bot.log.error(e);
				});
				break;
			}
			default: {
				msg.reply('Unknown Command.');
			}
		}
	},
	helpData: new HelpData({
		name: '!song',
		summary: 'Plays a link from youtube or soundcloud',
		description: '!song play < YT / SC URL >'
	})
});