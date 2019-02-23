const fs = require('fs');
const swearLib = JSON.parse(fs.readFileSync( __dirname + "/swears.json"));
const cleanLib = JSON.parse(fs.readFileSync( __dirname + "/clean.json"));
//const blacklist = JSON.parse(fs.readFileSync( __dirname + "/blacklist.json"));

module.exports.isSwear = function(str){
	for(let i = 0; i != swearLib.length; i++){
		let test = str.toLowerCase();
		if(test.includes(swearLib[i].toLowerCase())){
			for(let j = 0; j != cleanLib.length; j++){
				if(test.includes(cleanLib[j].toLowerCase())){
					return false;
				}
			}
			return true;
		}
	}
	return false;
}
//################################NPL stuff##################################
const natural = require('natural');
const classifierBayes = new natural.BayesClassifier();
const classifier = new natural.LogisticRegressionClassifier();

for(let i = 0; i != swearLib.length; i++){
	classifier.addDocument(swearLib[i], 'swear');
	classifierBayes.addDocument(swearLib[i], 'swear');
}

for(let i = 0; i != cleanLib.length; i++){
	classifier.addDocument(cleanLib[i], 'clean');
	classifierBayes.addDocument(cleanLib[i], 'clean');
}

classifier.train();
classifierBayes.train();

classifier.save('classifier.json', function(err, c) {

});

classifierBayes.save('classifierBayes.json', function(err, c) {

});


module.exports.isSwearLinearRegression = function(str){
	return ((classifier.classify(str) === 'swear') && !cleanLib.includes(str)) || swearLib.includes(str);
}

module.exports.trainingData = swearLib.length + cleanLib.length;
module.exports.trainingDataSwear = swearLib.length;
module.exports.trainingDataClean = cleanLib.length;

module.exports.isSwearNaiveBayes = function(str){
	return ((classifierBayes.classify(str) === 'swear') && !cleanLib.includes(str)) || swearLib.includes(str);
}

