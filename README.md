# Beastify Rust

This is a Rust/WASM version of the [Your second extension](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Your_second_WebExtension) example from Mozilla.

This only converts the browser action to use WASM instead of JavaScript because the client-story for WASM doesn't seem to be as well fleshed out at the moment. As far as I can tell, you really want a page to use WASM, so that limits it to browser actions and background scripts (since you can supply those as a page instead of scripts).

## Requirements

Requires a working install of `Rust`, `npm`, and `wasm-pack`.

## Usage

This can be built/run with two terminals using:

* `npm run devwatch` - compile and watch for changes
* `npm run start` - launch the extension and reload on changes

## References

* [Your second extension](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Your_second_WebExtension)
* [The wasm-bindgen Guide](https://rustwasm.github.io/docs/wasm-bindgen/)
* [rust-webpack-template](https://github.com/rustwasm/rust-webpack-template)
* [rustwasm-addon](https://github.com/willdurand/rustwasm-addon)
