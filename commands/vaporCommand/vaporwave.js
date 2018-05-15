//Shamelessly snatched from https://github.com/prasiman/vaporwave-text-generator/blob/master/script.js
function vaporwave(data) {
	let output = '';
	for (let i = 0; i < data.length; i++) {
		if (data.charCodeAt(i) >= 33 && data.charCodeAt(i) <= 270) {
			output += String.fromCharCode(data.charCodeAt(i) + 65248);
		} else if (data.charCodeAt(i) == 32) {
			output += String.fromCharCode(data.charCodeAt(i));
		}
	}
	return output;
}

module.exports = vaporwave;