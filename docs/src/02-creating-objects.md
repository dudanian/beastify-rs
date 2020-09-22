# Creating `Object`s: the first hurdle

A lot of the web extension APIs use generic objects to pass parameters. For example:

```js
let details = {active: true, currentWindow: true}
browser.tabs.query(details)
```

So we obviously need to be able to create generic objects in Rust to pass to these API functions. Originally, I thought you would be able to write something like:

```rust ,ignore
let details = Object::new();
details.set("active", true);
details.set("currentWindow", true);
```

But as it turns out, `Object` doesn't have a `set` method. And neither does `JsValue`. They only have getters. So how do we create objects? Or alternatively set properties? Well, this is actually [covered in the manual](https://rustwasm.github.io/docs/wasm-bindgen/reference/accessing-properties-of-untyped-js-values.html), even if the title of the page is somewhat misleading in my opinion since I wanted to _set_ properties rather than access them.

## Setting properties

Basically, my approach above wasn't too far off. Instead of using a `set` method, you have to use the _much_ more verbose [`js_sys::Reflect::set`](https://docs.rs/js-sys/0.3.45/js_sys/Reflect/fn.set.html) API:

```rust ,ignore
let details = Object::new();
js_sys::Reflect::set(&details, &JsValue::from_str("active"), &JsValue::from_bool(true)).unwrap();
js_sys::Reflect::set(&details, &JsValue::from_str("currentWindow"), &JsValue::from_bool(true)).unwrap();
```

Alternatively, we could have created a `serde`-able `struct` and used `JsValue::from_serde`, but from what I have read, using `JsValue::from_serde` is currently _very_ expensive.

I'm still not completely satisfied with this approach since we aren't preallocating space and each call to `js_sys::Reflect::set` calls back to JS-land. It almost seems like it would be better to just create some `const`s in JavaScript and import them into Rust. But my goal for this extension is to be completely contained in Rust. So while importing from JS might be better, I will hold off for now.
