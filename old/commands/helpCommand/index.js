const {Command, HelpData} = require('../Command.js');
module.exports = new Command({
	onMessageTest(bot, msg){
		return msg.content.startsWith('!help');
	},
	onMessage(bot, msg){
		let reply = '**\nHelp Commands:**\n';
		for(let i = 0; i != bot.commands.length; i++){
			let info = bot.commands[i].onHelp(bot, msg);
			if(info){
				reply += `[**${info.name}**]\t${info.summary}\n`;
			}
		}
		msg.reply(reply);
	},
	helpData: new HelpData({
		name: '!help',
		summary: 'Displays help for different commands',
		description: 'WIP'
	})
});