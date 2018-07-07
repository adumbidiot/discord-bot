const fs = require('fs');
const spellchecker = require('spellchecker');
const shortDictionary = (fs.readFileSync(__dirname + '/shortDictionary.txt')).toString().split('\r\n');
console.log('Dictionary Loaded');
let list = [];
let max = 20;

function naturalize1(str){
	if(str.length == 0) return "";
	let data = [];
	let last = 0;
	let i = 0;
	while(data.length < 4){
		while(i < max && i < str.length){
			i++;
			console.log(str.substr(last, i), i);
			if(shortDictionary.includes(str.substr(last, i))){
				break;
			}
			
		}
		data.push(str.substr(last, i));
		last = i;
	}
	return data;
}

function naturalize2(str){
	while(!inDic(str)){
		
	}
}

function inDic(arr){
	for(let i = 0; i != arr.length; i++){
		if(shortDictionary.includes(arr[i])){
			return i;
		}
	}
	return true
}

function naturalize3(str){
	let mis = spellchecker.checkSpelling(str);
	if(mis[0]){
		return spellchecker.getCorrectionsForMisspelling(str.substr(mis[0].start, mis[0].end));
	}
	return mis;
}


//console.log(shortDictionary);
console.log(shortDictionary.includes('sm'));
console.log(naturalize("smartass"));
console.log(naturalize("ithinkyoushouldgofuckyourself"));