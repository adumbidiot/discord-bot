const {Command} = require('../Command.js');
module.exports = new Command({
	onMessage(bot, msg){
		bot.log.info(`(${msg.author.tag}) ${msg.content}`);
	},
	onMessageTest(bot, msg){
		return true;
	}
});