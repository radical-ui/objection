const { prepareComponents, executeCommandOnComponent } = require('./dev-tools');

(async function() {
	const component = process.argv[2];

	if (!component)
		throw new Error(
			`You must specify a component name.  Example: "npm run dev line-ripple".`
		);

	await prepareComponents();

	executeCommandOnComponent(
		component,
		`npm run lint && &CD_HERE& && rollup -c -w`
	);
})();
