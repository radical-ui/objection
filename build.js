const {
	prepareComponents,
	executeCommandOnComponents,
} = require('./dev-tools');

(async function() {
	await prepareComponents();
	executeCommandOnComponents(`rollup -c`);
})();
