const fs = require('fs');
const lib = JSON.parse(fs.readFileSync( __dirname + "/swears.json"));
const cleanLib = JSON.parse(fs.readFileSync( __dirname + "/clean.json"));

module.exports.isSwear = function(str){
	for(let i = 0; i != lib.length; i++){
		let test = str.toLowerCase();
		if(test.includes(lib[i].toLowerCase())){
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

for(let i = 0; i != lib.length; i++){
	classifier.addDocument(lib[i], 'swear');
	classifierBayes.addDocument(lib[i], 'swear');
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

//TODO: Remove
module.exports.isSwearNatural = function(str){
	return classifier.classify(str) === 'swear';
}

module.exports.isSwearNaturalBayes = function(str){
	return classifierBayes.classify(str) === 'swear';
}

module.exports.isSwearLinearRegression = function(str){
	return classifier.classify(str) === 'swear';
}

module.exports.isSwearNaiveBayes = function(str){
	return classifierBayes.classify(str) === 'swear';
}

