const writeFile = require('write');
const fs = require('fs');

function writeCSS() {
	const css = fs.readFileSync('./src/styles/internal/mdc.css', 'utf-8');

	const content = `
export default \`
${css}
\`
	`;

	writeFile('./src/styles/internal/wrapped-styles.js', content, (err) => {
		if (err) console.log(err);
		else console.log('Wrapped CSS into `wrapped-styles.js`.');
	});
}

writeCSS();
