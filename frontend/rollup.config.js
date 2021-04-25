import inlineSvg from 'rollup-plugin-inline-svg';
import svelte from 'rollup-plugin-svelte';
import resolve from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';
import livereload from 'rollup-plugin-livereload';
import replace from '@rollup/plugin-replace';
import { terser } from 'rollup-plugin-terser';
import css from 'rollup-plugin-css-only';

const production = !process.env.ROLLUP_WATCH;

export default {
	input: 'src/main.js',
	output: {
		sourcemap: true,
		format: 'iife',
		name: 'app',
		file: 'public/build/bundle.js'
	},
	plugins: [
		inlineSvg({
			// Removes specified tags and its children. You can specify tags by setting removingTags query array.
			// default: false
			removeTags: true,

			// warning: this won't work unless you specify removeTags: true
			// default: ['title', 'desc', 'defs', 'style']
			removingTags: [ 'title', 'desc', 'defs', 'style' ],

			// warns about present tags, ex: ['desc', 'defs', 'style']
			// default: []
			warnTags: [],

			// Removes `width` and `height` attributes from <svg>.
			// default: true
			removeSVGTagAttrs: true,

			// Removes attributes from inside the <svg>.
			// default: []
			removingTagAttrs: [],

			// Warns to console about attributes from inside the <svg>.
			// default: []
			warnTagAttrs: []
		}),

		svelte({
			compilerOptions: {
				// enable run-time checks when not in production
				dev: !production
			},

			// Warnings are normally passed straight to Rollup. You can
			// optionally handle them here, for example to squelch
			// warnings with a particular code
			onwarn: (warning, handler) => {
				// e.g. don't warn on <marquee> elements, cos they're cool
				if (warning.code === 'a11y-distracting-elements') return;

				// let Rollup handle all other warnings normally
				handler(warning);
			}
		}),
		// we'll extract any component CSS out into
		// a separate file - better for performance
		css({ output: 'bundle.css' }),

		// If you have external dependencies installed from
		// npm, you'll most likely need these plugins. In
		// some cases you'll need additional configuration -
		// consult the documentation for details:
		// https://github.com/rollup/plugins/tree/master/packages/commonjs
		resolve({
			browser: true,
			dedupe: [ 'svelte' ]
		}),
		commonjs(),

		replace({
			IS_PRODUCTION: production
		}),

		// // In dev mode, call `npm run start` once
		// // the bundle has been generated
		// !production && serve(),

		// Watch the `public` directory and refresh the
		// browser on changes when not in production
		!production && livereload('public'),

		// If we're building for production (npm run build
		// instead of npm run dev), minify
		production && terser()
	],
	watch: {
		clearScreen: false
	}
};

/*
function serve() {
	let started = false;

	return {
		writeBundle() {
			if (!started) {
				started = true;

				require('child_process').spawn('npm', [ 'run', 'dev-server', '--', '--dev' ], {
					stdio: [ 'ignore', 'inherit', 'inherit' ],
					shell: true
				});
			}
		}
	};
}
			*/
