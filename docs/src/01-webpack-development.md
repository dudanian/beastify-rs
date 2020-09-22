# Using `webpack --mode development`

By default, running webpack with `--mode development` causes errors with our extension when trying to load our WASM module. This is because the unminified JavaScript contains an `eval()` which is prevented by the default [content security policy](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/content_security_policy) for extensions. We can get around this by explicitly allowing `eval()` in remote scripts by adding the following policy to our `manifest.json`:

```json
{
    // ...
    "content_security_policy":
      "script-src 'self' 'unsafe-eval'; object-src 'self'"
}
```

Of course, this should only be added when developing and be removed before release.

Alternatively, we can just always build in `--mode release` since that doesn't insert the problematic `eval()` call.
