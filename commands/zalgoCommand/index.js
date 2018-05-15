const {Command, HelpData} = require('../Command.js');
const zalgoify = require('./zalgoify.js');
let commandName = '!zalgo';
module.exports = new Command({
	onMessageTest(bot, msg){
		return msg.content.startsWith(commandName);
	},
	onMessage(bot, msg){
		msg.reply(zalgoify(msg.content.substr(commandName.length)));
	},
	helpData: new HelpData({
		name: commandName,
		summary: 'Zalgoifies things',
		description: 'WIP'
	})
});