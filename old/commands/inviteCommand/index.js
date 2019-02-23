const {Command, HelpData} = require('../Command.js');

module.exports = new Command({
	onMessageTest(bot, msg){
		return msg.content.startsWith('!invite');
	},
	onMessage(bot, msg){
		bot.client.generateInvite(['SEND_MESSAGES', 'VIEW_CHANNEL']).then(function(invite){
			msg.reply(invite);
		});
	},
	helpData: new HelpData({
		name: '!invite',
		summary: 'Generates an invite link',
		description: 'WIP'
	})
});