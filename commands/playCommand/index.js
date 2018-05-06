const {Command, HelpData} = require('../Command.js');

module.exports = new Command({
	onMessageTest(bot, msg){
		return msg.content.startsWith('!play');
	},
	onMessage(bot, msg){
		msg.member.voiceChannel.join().catch(function(e){
			bot.log.error(e);
		}).then(function(connection){
			msg.reply('Joined voice channel');
			return connection;
		});
	}
});