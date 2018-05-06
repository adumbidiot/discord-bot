const winston = require('winston');

const config = {
  levels: {
    error: 0,
    debug: 1,
    warn: 2,
    data: 3,
    info: 4,
    verbose: 5,
    silly: 6
  },
  colors: {
    error: 'red',
    debug: 'blue',
    warn: 'yellow',
    data: 'grey',
    info: 'green',
    verbose: 'cyan',
    silly: 'magenta'
  }
};

winston.addColors(config.colors);

const bracket = winston.format.printf(info => {
	return `[${info.level}] ${info.message}`;
});

module.exports = winston.createLogger({
	levels: config.levels,
	transports: [
		new winston.transports.Console({colorize: true})
	],
	format: winston.format.combine(
		winston.format.colorize(),
		bracket
	)
});