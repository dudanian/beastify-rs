# Handling callbacks: `Function`s and `Closure`s

The next big hurdle is how to create a simple callback function. Which again is [covered in the manual](https://rustwasm.github.io/docs/wasm-bindgen/examples/closures.html). In general, if we want to create a callback like:

```js
document.addEventListener("click", (e) => {
    ...
});
```

then we have to create a `wasm_bindgen::closure::Closure` and reference it as a `js_sys::Function` when we pass it to the API function:

```rust ,ignore
let cb = Closure::wrap(Box::new(move |e| {
    ...
}) as Box<dyn FnMut(JsValue)>);

document
    .add_event_listener_with_callback("click", cb.into_js_value().as_ref().unchecked_ref())
    .unwrap();
```

Most of the code above is fairly boiler-plate, and the documentation on all of the parts involved will do a much better job of explaining the details than I can. But let's go over some important things to note:

* the Rust closure (the `|| {}`, not the `Closure`) needs to be `move` because it needs to own everything it references
* the `Closure` needs to wrap a `Box`
* the argument types can be further specified using the `as Box<dyn FnMut(String, Element, Document)>`
* `as_ref()` returns a reference to `JsValue`
* `unchecked_ref::<Function>()` returns a reference to a `Function`
* dropping the `Closure` will invalidate the callback
  * easiest way to avoid this is to pass ownership to JS using `forget()` or `into_js_value()`
  * however, these leak memory and should be used with caution

Probably the most annoying thing of all of the above is that there is no way to go straight from a `Closure` to a `Function`. For some reason, you _have_ to go through `JsValue`. This isn't a big deal, but seeing as I'm pretty sure the only reason to use `Closure` is to pass it as a `Function`, it just seems like an oversight.
