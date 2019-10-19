import svelte from 'rollup-plugin-svelte';
import resolve from 'rollup-plugin-node-resolve';
import commonjs from 'rollup-plugin-commonjs';
import livereload from 'rollup-plugin-livereload';
import { terser } from 'rollup-plugin-terser';
import serve from 'rollup-plugin-serve';

const production = !process.env.ROLLUP_WATCH;

export default ({ className }) => {
	return {
		input: production ? `component.svelte` : `test.svelte`,
		output: {
			sourcemap: !production,
			format: 'iife',
			name: production ? className : 'Test',
			file: production ? `dist/component.js` : `dist/main.js`,
		},
		plugins: [
			svelte({
				// enable run-time checks when not in production
				dev: !production,
				// we'll extract any component CSS out into
				// a separate file — better for performance
				css: (css) => {
					css.write(
						production ? `dist/component.css` : `dist/bundle.css`,
						!production
					);
				},
			}),

			// If you have external dependencies installed from
			// npm, you'll most likely need these plugins. In
			// some cases you'll need additional configuration —
			// consult the documentation for details:
			// https://github.com/rollup/rollup-plugin-commonjs
			resolve({
				browser: true,
				// dedupe: importee => importee === 'svelte' || importee.startsWith('svelte/')
			}),
			commonjs(),

			// Watch the `dist` directory and refresh the
			// browser on changes when not in production
			!production && livereload(`dist`),

			// If we're building for production (npm run build
			// instead of npm run dev), minify
			production && terser(),

			// Serve the dist folder if we are not in production
			!production &&
				serve({
					open: true,
					contentBase: 'dist',
				}),
		],
		watch: {
			clearScreen: false,
		},
	};
};
