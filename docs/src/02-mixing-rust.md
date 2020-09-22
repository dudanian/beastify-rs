# Mixing Rust functions

The most trivial function in the example is probably `beastNameToUrl`. It is a small static function that doesn't close over any values. In Rust, it would look something like:

```rust ,ignore
#[wasm_bindgen]
pub fn beast_name_to_url(beast_name: &str) -> JsValue {
    let path = match beast_name {
        "Frog" => "icons/beasts/frog.jpg",
        "Snake" => "icons/beasts/snake.jpg",
        "Turtle" => "icons/beasts/turtle.jpg",
        _ => return JsValue::null(),
    };

    browser::runtime::get_url(path)
}
```

Notice `wasm-bindgen` can automatically convert our arguments to `&str`s for us, although it will panic if the JavaScript doesn't end up passing a String value. Also just ignore the browser API function at the bottom for a bit, we will get to that later.

## Importing into JavaScript

If we wanted to use this function in our JavaScript, we have to import it. Our WASM code is compiled into a [JavaScript module](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Modules), which means we can `import` certain `export`ed functions from it. Marking our function as `pub` will automatically add it to our list of exported functions.

One way we can import our function is to wrap the entire `choose_beast.js` in an async [IIFE](https://developer.mozilla.org/en-US/docs/Glossary/IIFE) with a [dynamic import](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/import) to load our module. If you look at `index.js`, you will see that the template is already loading our WASM module dynamically.

All we need to do is remove `index.js` from `choose_beast.html` (we previously added it) and add the following around our `choose_beast.js` file instead:

```js
(async () => {
const { beast_name_to_url } = await import("../pkg");
// ...
})();
```

This will dynamically import our module and get a handle for our function from WASM. Then we can update the original code to use our function instead of the JavaScript version. Our final version of the extension won't use this wrapper since everything will be written in Rust, but it is convenient for converting things slowly.
