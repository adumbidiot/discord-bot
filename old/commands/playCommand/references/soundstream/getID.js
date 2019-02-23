const https = require('https');
const url = require('url');

module.exports = function(link, cb){
	let opts = {
		protocol: 'https:',
		host: 'soundcloud.com',
		method: 'GET',
		path: '',
		headers: {
			"user-agent":"Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/44.0.2403.107 Safari/537.36"
		}
	};
	let urlObj = url.parse(link);
	opts.path = urlObj.pathname;
	
	https.get(opts, function(res){
		let body = "";

		res.on('data', function(data){
			body += data;
		});
	
		res.on('end', function(){
			let a = body.indexOf('"id":64'); //65
			let subA = body.substr(a + 4);
			//console.log(subA);

			let b = subA.indexOf('</script>');
			let subB = subA.substr(0, b);

			let c = subB.indexOf('"id":');
			let subC = subB.substr(c + 5);

			let d = subC.indexOf(',');
			let subD = subC.substr(0, d);

			cb(subD);
		});
	});
};