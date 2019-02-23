let get = require('./xkcd.js').getRandom;
get().then((data) => {
	console.log(data);
});