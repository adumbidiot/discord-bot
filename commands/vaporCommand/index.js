const {Command, HelpData} = require('../Command.js');
const vaporwave = require('./vaporwave.js');

module.exports = new Command({
	onMessageTest(bot, msg){
		return msg.content.startsWith('!vapor');
	},
	onMessage(bot, msg){
		msg.reply(vaporwave(msg.content.substr('!vapor'.length)));
	},
	helpData: new HelpData({
		name: '!vapor',
		summary: 'Makes text really spaced out',
		description: 'WIP'
	})
});

