const {Command} = require('../Command.js');
const {isSwear, isSwearNaiveBayes, isSwearLinearRegression} = require('./swear.js');
module.exports = new Command({
	onMessageTest(bot, msg){
		//Bit slow. Maybe add a cache?
		return isSwearLinearRegression(msg.content);//isSwearNaiveBayes(msg.content)/* && isSwearLinearRegression(msg)*/;
	},
	onMessage(bot, msg){
		msg.channel.send({
			files: ['https://cdn.discordapp.com/attachments/411016363306516481/441766175832014848/image.jpg']
		});
	}
});