const discord = require('discord.js');
const fs = require('fs');
const log = require('./logger.js');

module.exports = class Bot {
	constructor(opts){
		opts = opts || {};
		this.client = new discord.Client();
		this.log = log;
		this.commands = opts.commands || [];
		this.token = getToken();
		if(this.token){
			log.info('Logging in using token: ' + this.token);
			this.client.login(this.token);
		}
		this.client.on('message', (msg) => {
			for(let i = 0; i != this.commands.length; i++){
				if(this.commands[i].onMessageTest(this, msg)){
					this.commands[i].onMessage(this, msg);
				}	
			}
		});
		
		this.client.on('ready', () => {
			log.info(`Logged in as ${this.client.user.tag}!`);
		});
	}
}

function getToken(){
	log.info("Attempting to get token from TOKEN env var...");
	let token = process.env.TOKEN;
	if(token){
		log.info('TOKEN loaded from env TOKEN!');
		return token;
	}else{
		log.warn('TOKEN env not found, attempting to load from token.txt...');
		try{
			token = fs.readFileSync('token.txt', 'utf8');
		}catch(e){
			if(e.code === 'ENOENT'){
				log.error('token.txt not found, TOKEN not set!');
			}else{
				console.log(e);
			}
		}
		if(token){
			log.info('TOKEN loaded from file token.txt!');
			return token;
		}
	}
}