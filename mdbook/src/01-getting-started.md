# Getting started

I decided the best place to start with is probably to just rewrite a small, working extension. This way, we know that the extension works and any problems can be directly attributed to my own mistakes and not to the extension.

So I tried to rewrite the [Your first extension](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Your_first_WebExtension) example from Mozilla in Rust. I say "try" because I ended up giving up on this idea. Despite there being only 1 line of JavaScript, trivially rewritable in Rust, it was _not_ so easy to get the resulting WASM module loaded on the client page.

Basically, the project changed from rewriting some JavaScript in Rust to trying to figure out how `fetch` works with [content scripts](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Content_scripts). A problem you can probably tell from my disclaimer that I am woefully unequipped to handle, although it does seem possible. I might revisit this in a "Part 2" down the road, but for now I'm okay with just giving up on content scripts and sending them as JavaScript.

Instead, let's look at [Your second extension](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Your_second_WebExtension) instead. While this also has a content script, it also has a [browser action](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Browser_actions) which is easier to convert to Rust (this seems to be because `html` pages like browser actions have an easier time loading multiple resources).

So let's get started!
