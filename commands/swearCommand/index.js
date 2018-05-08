const {Command} = require('../Command.js');
const {isSwear} = require('./swear.js');
module.exports = new Command({
	onMessageTest(bot, msg){
		return isSwear(msg.content);
	},
	onMessage(bot, msg){
		msg.channel.send({
			files: ['https://cdn.discordapp.com/attachments/411016363306516481/441766175832014848/image.jpg']
		});
	}
});