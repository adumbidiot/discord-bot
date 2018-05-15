let logCommand = require('./commands/logCommand/index.js');
let pingCommand = require('./commands/pingCommand/index.js');
let helpCommand = require('./commands/helpCommand/index.js');
let zalgoCommand = require('./commands/zalgoCommand/index.js');
let inviteCommand = require('./commands/inviteCommand/index.js');
let xkcdCommand = require('./commands/xkcdCommand/index.js');
let playCommand = require('./commands/playCommand/index.js');
let stopCommand = require('./commands/stopCommand/index.js');
let swearCommand = require('./commands/swearCommand/index.js');
let vaporCommand = require('./commands/vaporCommand/index.js');
let Bot = require('./bot.js');
let client = new Bot({
	commands: [
		logCommand,
		pingCommand,
		helpCommand,
		zalgoCommand,
		inviteCommand,
		xkcdCommand,
		playCommand,
		stopCommand,
		swearCommand,
		vaporCommand
	]
});

