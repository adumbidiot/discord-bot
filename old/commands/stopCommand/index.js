const {Command, HelpData} = require('../Command.js');
module.exports = new Command({
	onMessageTest(bot, msg){
		return msg.content.startsWith('!stop');
	},
	onMessage(bot, msg){
		const guildID = msg.guild.id;
		let conn = bot.client.voiceConnections.get(guildID);
		if(conn){
			msg.reply('Disconnected from voice channel');
			conn.disconnect();
		}else{
			msg.reply('Not currently in a voice channel in this server');
		}
	},
	helpData: new HelpData({
		name: '!stop',
		summary: 'Stops a currenly playing voice stream',
		description: 'WIP'
	})
	
});