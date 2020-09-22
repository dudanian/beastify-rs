# Browser API bindings (or lack thereof)

Let's back up for a second and talk about JavaScript APIs and Rust bindings. `wasm-bindgen` is awesome and has two important crates for accessing JavaScript APIs:

* `js-sys` - bindings for all global APIs (i.e. standard ECMAScript)
* `web-sys` - bindings for all of the [Web APIs](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API) (i.e. APIs given a `window`)

But what it _doesn't_ provide is bindings for [JavaScript APIs for web extensions](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API). And since we are writing a web extension, we kind of need these APIs.

I'm not sure why `wasm-bindgen` decided not to provide bindings for browser APIs in addition to the others, but I would assume it is because of [the incompatibilities between the implementations on Firefox and Chrome](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Chrome_incompatibilities). Or maybe they just forgot. Either way, we just have to provide our own bindings.

## Our bindings

There are a few ways we could have written our bindings, for example creating `type`s for our namespaces, but I decided to go with a simple nested module approach which more or less looks like:

```rust ,ignore
pub mod browser {
    pub mod runtime {
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_namespace = ["browser", "runtime"], js_name = getURL)]
            pub fn get_url(path: &str) -> JsValue;
        }
    }
}
```

Logically, Rust modules perform a similar function to JavaScript namespaces: housing other namespaces and/or functions. The bad part about this approach is that we end up with a lot of nested boilerplate code, but that's life.

If we look at the [`getURL` documentation](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/runtime/getURL), we can see that it expects the input argument to be a `string` and returns a `string`. In the above example, I made the input parameter a `&str` so that we can use it with raw Rust values instead of having to convert them to `JsValue`s first. However, I kept the output as a `JsValue` since we will not be directly using this value but passing it on to another API function later. It could also be written to return a `String` since `wasm-bindgen` will automatically convert the `JsValue` to a `String` for us, but this conversion is not free, and writing it this way prevents a needless conversion from `JsValue` -> `String` -> `JsValue`.

In the end, a JavaScript call like:

```js
let url = browser.runtime.getURL("somepath");
```

can now be written in Rust like:

```rust ,ignore
let url: JsValue = browser::runtime::get_url("somepath");
```
