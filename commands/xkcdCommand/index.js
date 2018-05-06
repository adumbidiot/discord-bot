const {Command, HelpData} = require('../Command.js');
const xkcd = require('./xkcd.js');
module.exports = new Command({
	onMessageTest(bot, msg){
		return msg.content.startsWith('!xkcd');
	},
	onMessage(bot, msg){
		xkcd.getRandom().catch(function(err){
			bot.error(err);
			msg.reply('Error: Could not get a random comic. This error has been logged and will be investigated.');
		}).then(function(url){
			msg.reply(url);
		});
	},
	helpData: new HelpData({
		name: '!xkcd',
		summary: 'Get a random comic from xkcd',
		description: 'WIP'
	})
});