const fs = require('fs');
const write = require('write');
const { exec } = require('child_process');

function getComponents() {
	return new Promise((resolve, reject) => {
		fs.readFile(`components.txt`, 'utf-8', (err, data) => {
			if (err) reject(err);
			else resolve(data.trim().split('\n'));
		});
	});
}

function readComponent(name) {
	return new Promise((resolve, reject) => {
		fs.readdir(name, (err, files) => {
			if (err) {
				if (err.code === `ENOENT`) {
					resolve([]);
				} else reject(err);
			} else {
				if (files.find((f) => f === 'test')) {
					fs.readdir(name + '/test', (err, testFiles) => {
						if (err) reject(err);
						else {
							files.push(
								...testFiles.map((file) => 'test/' + file)
							);
							resolve(files);
						}
					});
				} else resolve(files);
			}
		});
	});
}

function classNameFromName(name) {
	return (
		'ST' +
		name
			.split('-')
			.map((section) => {
				const chars = section.split('');
				chars[0] = chars[0].toUpperCase();
				return chars.join('');
			})
			.join('')
	);
}

function createHTMLForComponentServe() {
	return `<html>
		<head>
			<title>Svelte Component</title>
			<link rel="stylesheet" href="bundle.css">
			<script src="main.js"></script>
		</head>
		<body>
			<div id="app-root"></div>

			<script>
				new Test({ target: document.querySelector('#app-root') })
			</script>
		</body>
	</html>`;
}

const filesThatShouldBe = ({ name }) => ({
	'package.json': `{
		"main": "index.js",
		"svelte": "component.svelte"
	}`,

	'component.svelte': `Hello world!`,

	'index.js': `import Component from './dist/component.js';
	export default Component;`,

	'rollup.config.js': `import RollupConfiguration from '../rollup-configuration';
	export default RollupConfiguration({ className: '${classNameFromName(
		name
	)}' });`,

	'dist/index.html': createHTMLForComponentServe(),

	'test.svelte': `<script>
		import ${classNameFromName(name)} from './component.svelte';
	</script>
	
	<${classNameFromName(name)} />`,
});

function missingKeys(source, test) {
	let missing = [];

	source.forEach((s) => {
		if (!test.find((t) => t === s)) missing.push(s);
	});

	return missing;
}

async function prepareComponents() {
	const components = await getComponents();
	let promises = [];

	components.forEach(async (component) => {
		const files = await readComponent(component);
		const expectedFiles = filesThatShouldBe({ name: component });

		const missing = missingKeys(Object.keys(expectedFiles), files);

		missing.forEach(async (file) => {
			promises.push(write(`${component}/${file}`, expectedFiles[file]));
		});
	});

	await Promise.all(promises);
}

function executeCommandOnComponent(
	name,
	command = 'echo Invalid',
	onLog = (data) => console.log(data)
) {
	if (command.indexOf(`&CD_HERE&`) !== -1)
		command = command.replace(`&CD_HERE&`, `cd ${name}`);
	else command = `cd ${name} && ${command}`;

	const { stdout, stderr } = exec(command);

	stdout.on(`data`, onLog);
	stderr.on(`data`, onLog);
}

async function executeCommandOnComponents(command) {
	const components = await getComponents();

	components.forEach((component) => {
		executeCommandOnComponent(component, command);
	});
}

module.exports = {
	prepareComponents,
	executeCommandOnComponent,
	executeCommandOnComponents,
};
