const {isSwear, isSwearLinearRegression, isSwearNaiveBayes} = require('./swear.js');
const colors = require('colors');
const fs = require('fs');
const list = JSON.parse(fs.readFileSync(__dirname + '/test.json'));
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
	},
	{
		word: "S H I T",
		swear: true
	},
	{
		word: "SH IT",
		swear: true
	},
	{
		word: "CR AP",
		swear: true
	},
	{
		word: "Heck",
		swear: true
	},
	{
		word: "can we talk about the tits",
		swear: true
	},
	{
		word: "I told jack it was a moral choice",
		swear: false
	},
	{
		word: "picking one of them first",
		swear: false
	},
	{
		word: "he freaked out",
		swear: false
	},
	{
		word: "it was great",
		swear: false
	},
	{
		word: "they shunned him for his long dong, now he stabs them with his sharp shlong",
	},
	{
		word: "shlongslayer420",
	},
	{
		word: "i fuckiung said albert only",
		swear: true
	},
	{
		word: "not a fucking block party",
		swear: true
	},
	{
		word: "m ears are being raped",
		swear: true
	},
	{
		word: "Somebody made a joke on the Internet",
		swear: false
	},
	{
		word: "You had a fucking mess with your cables",
		swear: true
	},
	{
		word: "As long as this fuck responds",
		swear: true
	},
	{
		word: "fuck me right in the asshole",
		swear: true
	},
	{
		word: "go duck yourself",
		swear: true
	},
	{
		word: "feggit",
		swear: true
	},
	{
		word: "fecker",
		swear: true
	},
	{
		word: "fuckasaurus rex",
		swear: true
	},
	{
		word: "gofuckyourself",
		swear: true
	},
	{
		word: "soifidonttypeanyspacesicanswearasmuchasiwantyoursystemthingreallyfuckingsuckslolwhatashittysystem",
		swear: true
	},
	{
		word: "motherfuckin easterbacca",
		swear: true
	},
	{
		word: "fuckmerighthahahaha",
		swear: true
	},
	{
		word: "fuckmerightintheasshole",
		swear: true
	},
	{
		word: "stupidassmotherfucker",
		swear: true
	},
	{
		word: "fuckshitfuckshit",
		swear: true
	},
	{
		word: "hellyeah",
		swear: true
	},
	{
		word: "stupidshit",
		swear: true
	},
	{
		word: "nowlistenupyoufatfack",
		swear: true
	},
	{
		word: "asswipe",
		swear: true
	},
	{
		word: "motherducker",
		swear: true
	},
	{
		word: "duck you",
		swear: true
	},
	{
		word: "duckyou",
		swear: true
	},
	{
		word: "gotohellyoustupidfuck",
		swear: true
	},
	{
		word: "duckyoutoo",
		swear: true
	},
	{
		word: "heckyou",
		swear: true
	},
	{
		word: "ihavethetightestass",
		swear: true
	},
	{
		word: "fuckmesilly",
		swear: true
	},
	{
		word: "fuckfuckfuck",
		swear: true
	},
	{
		word: "stupidassnigger",
		swear: true
	},
	{
		word: "fuckyouandeverythingyoustandfor",
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
	let startTime = new Date();
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
	let endTime = new Date();
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
	data +=  displayFraction+ '\t' + displayPercentage + "\t" + (endTime.getTime() - startTime.getTime()) + 'ms';
	console.log(data + '\n');
}
