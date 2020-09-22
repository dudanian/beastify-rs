# Handling `Promise`s with `Future`s

Finally, let's talk about handling `Promise`s in Rust. This should be the last topic before rewriting our extension. At first, if you look at `js_sys::Promise`'s documentation, you notice that it has a `then` and a `catch` function which both take a `&Closure` as an argument. So I originally thought we would handle them similarly to how you handle function callbacks. For the given JavaScript:

```js
function_returning_promise()
.then((arg) => {
    // ... some anonymous function
})
.catch(error_handler);
```

I thought the Rust would look like:

```rust ,ignore
let cb = Closure::wrap(Box::new(move |arg| {
    // ... some anonymous function
}) as Box<dyn FnMut(JsValue)>);

let err = Closure::wrap(Box::new(move |e| {
    error_handler(e)
}) as Box<dyn FnMut(JsValue)>);

function_returning_promise().then(&cb).catch(&err);
cb.forget();
err.forget();
```

The above _does_ in fact work in the sens that the `Closure` will get called by `then` on success. But oh boy is this wrong. So so wrong. Why you might ask? Well to start out, it is _guaranteed_ to leak memory. Using `forget()` is essentially telling Rust that it is ok to leak this memory! I have read [the documentation](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/closure/struct.Closure.html#method.forget) a few times now, and it seems like it will _not_ get GC'd by JavaScript automatically like you might expect.

I think this function is basically like telling Rust, "Hey, please don't free this memory. I know we can't reclaim it, but I really _really_ want this function callback to work...". This can be okay for functions that don't get dynamically created like our initial callback, but is definitely not something we want to do on every click.

## `once` and `once_into_js`

You also might be thinking that using `Closure::once` or `Closure::once_into_js` instead of `Closure::wrap` will solve this problem since they _will_ free the underlying memory after the function is called. But the problem here is that a `Promise` will only call _one_ of those two functions: on `resolve`, the `then` callback gets called and on `reject`, the `catch` callback gets called. Which means while one of the `Closure`s does get freed, the other is still guaranteed to leak. So `Closure::once` also does not seem to be the answer.

## Rust `JsFuture`s

Instead, Rust provides an alternative way for handling `Promise`s: `Future`s, which are really just Rust's way of handling any asynchronous processing.

Also note that `Promise`s in JavaScript are _eager_ while `Promise`s/`Future`s in Rust are _lazy_. What this means is that creating a `Promise` in JavaScript is enough to begin execution. However, we need to explicitly start `Promise`s in Rust, the simplest of which is probably to use `wasm_bindgen_futures::spawn_local` with an `async` block:

```rust ,ignore
let f: JsFuture = // get future somehow
spawn_local(async move {
    f.await.unwrap();
});
```

Note again that we need a `move` in most cases like with `Closure`s. I don't think the move is mandatory in _all_ cases, but it geneally seems like we need it.

So the actual Rust code for handling `Promises` will generally look like:

```rust ,ignore
let promise = function_returning_promise();
spawn_local(async move {
    match JsFuture::from(promise).await {
        Ok(arg) => {
            // ... some anonymous function
        },
        Err(arg) => error_handler(arg),
    }
});
```

With this, we are no longer creating unnecessary `Closure`s and leaking all that memory.
