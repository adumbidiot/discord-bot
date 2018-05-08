const fs = require('fs');
const lib = JSON.parse(fs.readFileSync( __dirname + "/swears.json"));
module.exports.isSwear = function(str){
	for(let i = 0; i != lib.length; i++){
		if(str.includes(lib[i])){
			return true;
		}
	}
	return false;
}