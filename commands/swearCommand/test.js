const {isSwear, isSwearLinearRegression, isSwearNaiveBayes} = require('./swear.js');
const colors = require('colors');
const fs = require('fs');
const list = JSON.parse(fs.readFileSync(__dirname + '/test.json'));
console.log(isSwearLinearRegression);
let oldList = [
	{
		word: "gotta birb parents randomly sending me to do random shit",
		swear: true
	},
	{
		word: "why not both? its an action rpg",
		swear: false
	},
	{
		word: "use the greatsword",
		swear: false
	},
	{
		word: "So if you aren't taking it this year you might want to consider taking it next year",
		swear: false
	},
	{
		word: "looks good to me",
		swear: false
	},
	{
		word: "I’ve been working on the  new level builder extremely slowly. I’ve been trying to pick a UI library in c++ they’re all shit. Finally picked a library and built the sample app. Wanted your thoughts on it",
		swear: true
	},
	{
		word: "I'm not sure if that guy is dumb or smart",
		swear: false
	},
	{
		word: "ill follow your advise and try to work around the need of a tilesheet/number system",
		swear: false
	},
	{
		word: "Let's say it was murder, very serious and the guilty people should definitely be severely punished",
		swear: false
	},
	{
		word: "Was moving sheeeit brah",
		swear: true
	},
	{
		word: "stop doing education",
		swear: false
	},
	{
		word: "NIGGER",
		swear: true
	},
	{
		word: "GET THE FUCK ON\\",
		swear: true
	},
	{
		word: "shitlick",
		swear: true
	},
	{
		word:"Eat a dick cuckstain",
		swear: true
	},
	{
		word: "mythic you disgusting fucking cunt",
		swear: true
	},
	{
		word: "nigger",
		swear: true
	},
	{
		word: "my moms vagina",
		swear: true
	},
	{
		word: "why am i a niggah pappi",
		swear: true
	},
	{
		word: "what a fucking nigger",
		swear: true
	},
	{
		word: "Hey is pikadick allowed back if he isn't a nigger",
		swear: true
	},
	{
		word: "yo wtf its says voice activity is automatic",
		swear: true
	}
];
/*

*/

Array.prototype.push.apply(list, oldList);

testClassifier('Include Classifier', isSwear, true);
testClassifier('Bayes Classifier', isSwearNaiveBayes, true);
testClassifier('Regression Classifier', isSwearLinearRegression, true);

function testClassifier(name, classifier, hide){
	console.log(name + ' :');
	let data = '';
	let pass = 0;
	let passTotal = 0;
	for(let i = 0; i != list.length; i++){
		let el = list[i];
		let result = classifier(el.word);
		let passBool = (result === el.swear);
		let passData = (passBool ? '[PASS]'.green : '[FAIL]'.red);
		if(passBool){
				passTotal++;
		}
		if(hide && passBool){
				pass++;
		}else if(hide && !passBool && pass > 0){
			data += ('...[ PASS +' + pass + ' ]').green + '\n';
			pass = 0;
		}
		if(!hide || !passBool){
			let output = '\t' + '"' + el.word +'": ' + result;
			while(output.length < 25){
				output += ' ';
			}
			data += passData + output + '\n';
		}
	}
	if(pass > 0){
		data += ('...[ PASS +' + pass + ' ]').green + '\n';
	}
	let percentage = (passTotal * 100/list.length).toFixed(2);
	let displayFraction = passTotal + '/' + list.length;
	let displayPercentage = percentage + '%';
	switch(true){
		case (percentage > 90): {
			displayFraction = displayFraction.green;
			displayPercentage = displayPercentage.green;
			break;
		}
		case (percentage > 80): {
			displayFraction = displayFraction.cyan;
			displayPercentage = displayPercentage.cyan;
			break;
		}
		case percentage > 70: {
			displayFraction = displayFraction.red;
			displayPercentage = displayPercentage.red;
			break;
		}
	}
	data +=  displayFraction+ '\t' + displayPercentage;
	console.log(data + '\n');
}
