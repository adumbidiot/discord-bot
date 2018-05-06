const {Command, HelpData} = require('../Command.js');
const ytdl = require('ytdl-core');

module.exports = new Command({
	onMessageTest(bot, msg){
		return msg.content.startsWith('!play');
	},
	onMessage(bot, msg){
		msg.reply('Attempting to join voice channel...');
		msg.member.voiceChannel.join().catch(function(e){
			bot.log.error(e);
		}).then(function(connection){
			msg.reply('Joined voice channel');
			const url = msg.content.substr('!play '.length);
			const stream = ytdl(url, { filter : 'audioonly' });
			const streamOptions = { seek: 0, volume: 1 };
			const dispatcher = connection.playStream(stream, streamOptions);
			
			dispatcher.on('end', function(){
				connection.disconnect();
			});
			
			dispatcher.on('error', function(err){
				bot.log.error(err);
				bot.log.error(err);
			});
			
			dispatcher.on('start', function(){
				msg.reply('Starting stream from ' + url);
			});
			
			connection.on('warn', function(err){
				bot.log.warn(err);
			});
		});
	},
	helpData: new HelpData({
		name: '!play',
		summary: 'Plays a link from youtube',
		description: 'WIP'
	})
});