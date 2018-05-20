const {isSwear, isSwearLinearRegression, isSwearNaiveBayes, trainingData} = require('./swear.js');
const colors = require('colors');
const fs = require('fs');
const list = JSON.parse(fs.readFileSync(__dirname + '/test.json'));
let oldList = [
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
	},
	{
		word: "",
		swear: false
	},
	{
		word: "god",
		swear: false
	},
	{
		word: "fuckyoutoo",
		swear: true
	},
	{
		word: "dumb",
		swear: false
	},
	{
		word: "dumbass",
		swear: true
	},
	{
		word: "dumb@ss",
		swear: true
	},
	{
		word: "dumb@$$",
		swear: true
	},
	{
		word: "@sshole",
		swear: true
	},
	{
		word: "@$$hole",
		swear: true
	},
	{
		word: "sexyassmotherfucker",
		swear: true
	},
	{
		word: "dumbasscunt",
		swear: true
	},
	{
		word: "dumbasswordswordswords",
		swear: true
	},
	{
		word: "godlike",
		swear: false
	},
	{
		word: "fuckgod",
		swear: true
	},
	{
		word: "godisgreat",
		swear: false
	},
	{
		word: "godisgood",
		swear: false
	},
	{
		word: "goddamn",
		swear: true
	},
	{
		word: "goddriven",
		swear: false
	},
	{
		word: "the existence of god",
		swear: false
	},
	{
		word: "theexistenceofgod",
		swear: false
	},
	{
		word: "god is one",
		swear: false
	},
	{
		word: "godisone",
		swear: false
	},
	{
		word: "only god can judge me",
		swear: false
	},
	{
		word: "onlygodcanjudgeme",
		swear: false
	},
	{
		word: "god sees all",
		swear: false
	},
	{
		word: "godseesall",
		swear: false
	},
	{
		word: "god will judge all",
		swear: false
	},
	{
		word: "godwilljudgeall",
		swear: false
	},
	{
		word: "godhatesme",
		swear: false
	},
	{
		word: "godlovesall",
		swear: false
	},
	{
		word: "whydoesgodhateme?",
		swear: false
	},
	{
		word: "do you thing god stays in heaven because hes afraid of what he created?",
		swear: false
	},
	{
		word: "doyouthinggodstaysinheavenbecausehesafraidofwhathecreated?",
		swear: false
	},
	{
		word: "broke ass nigga",
		swear: true
	},
	{
		word: "brokeassnigga",
		swear: true
	},
	{
		word: "why has god abandoned us?",
		swear: false
	},
	{
		word: "whyhasgodabandonedus?",
		swear: false
	},
	{
		word: "any idea what could've caused my computer to do this?",
		swear: false
	},
	{
		word: "Also is this seriously triggering after everything",
		swear: false
	},
	{
		word: "After rebooting it's acting normal",
		swear: false
	},
	{
		word: "Potato",
		swear: false
	},
	{
		word: "Pikachu",
		swear: false
	},
	{
		word: "Djqkgnsjf",
		swear: false
	},
	{
		word: "bot is borked",
		swear: false
	},
	{
		word: "bork",
		swear: false
	},
	{
		word: "what the fuck happened here",
		swear: true
	},
	{
		word: "rebooting works because it clears the ram, and any errors that might have been there",
		swear: false
	},
	{
		word: "this normally happens during a shutdown, but windows does a \"fake shutdown\" and stores the contents of the ram to the hard disk to start faster",
		swear: false
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
	data +=  displayFraction+ '\t' + displayPercentage + "\t" + (endTime.getTime() - startTime.getTime()) + 'ms\t' + trainingData + ' entries';
	console.log(data + '\n');
}
