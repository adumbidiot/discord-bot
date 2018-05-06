let {Command, HelpData} = require('../Command.js');
module.exports = new Command({
		onMessage(bot, msg){
			msg.reply('Pong');
		},
		onMessageTest(bot, msg){
			return msg.content.toLowerCase() === 'ping';
		},
		helpData: new HelpData({
				name: '!ping',
				summary: 'Replies with pong',
				description: 'Replies with pong'
		})
});