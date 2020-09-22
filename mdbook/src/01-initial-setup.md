# Initial setup

I'm using the [rust-webpack-template](https://github.com/rustwasm/rust-webpack-template) as a starting point:

```sh
npm init rust-webpack beastify-rs
```

From there, I copied all of the example files from [the example extension](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Your_second_WebExtension) into the `static` directory. This should include:

* `manifest.json` - the extension's manifest
* `choose_beast.html` - the browser action page
* `choose_beast.css` - the browser action page's style
* `beastify.js` - the content script
* `icons` - directory containing all the icons

The last file, `choose_beast.js`, was put into the `js` folder because we are going to be messing with it. Also, make sure you either keep the same directory structure of the example or update all of the reference paths. I will be using slightly different paths than the original example.

## "Hello world!"

And finally we want to add our WASM module to the browser action's page. By default it should just log a `"Hello world!"` when it is loaded. All we need to do is add our `index.js` to the `choose_beast.html` file:

```html
<body>
  ...
  <script src="index.js"></script>
  <script src="choose_beast.js"></script>
</body>
```

With this, we should be able to compile our extension and confirm that it works. If we run `web-ext run -s dist` and visit a webpage, the extension should work properly. And if we go to `about:debugging` -> `This Firefox` -> `Beastify` -> `Inspect` -> `Console` and then click the extension, we should see `"Hello world!"` being printed by our WASM when it is loaded. And with that, we have successfully gotten some WASM to run in an extension!
