// Rollup 3 is used instead of Rollup 4 because it doesn't try to download wasm.
//
// While it is likely that running SWC via wasm is faster than acorn, the additionally overhead of having to download
// a massive wasm on every build is far worse. Ideally, we should download the wasm file ahead of time during the module
// graph stage (in rust) and cache it in the filesystem, rewriting `window.fetch` to use this downloaded file. However,
// this additionaly complexity does not seem worthwhile at the moment.
//
// It would be far better if one of the exiting rust bundlers were improved so that they could tree-shake to the extent of
// rollup, this would enable us to use our already-parsed module graph, doing away with this JS process and rollup altogether.
export * as rollup from 'https://esm.sh/@rollup/browser@3.29.4'

export * as colors from 'jsr:@std/fmt@0.225.6/colors'
export * as streamUtils from 'jsr:@std/streams@0.224.5'
