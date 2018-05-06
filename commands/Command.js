module.exports.Command = class Command{
	constructor(opts){
		this.onMessageTest = opts.onMessageTest || this.onMessageTest;
		this.onMessage = opts.onMessage || this.onMessage;
		this.helpData = opts.helpData || null;
	}
	onMessage(bot, msg){
		return;
	}
	onMessageTest(bot, msg){
		return false;
	}
	onHelp(bot, msg){
		return this.helpData;
	}
}
module.exports.HelpData = class HelpData{
	constructor(opts){
		opts = opts || {};
		this.name = opts.name || '';
		this.summary = opts.summary || '';
		this.description = this.description || '';
	}
}
