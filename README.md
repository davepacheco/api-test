This example shows how it is possible using progenitor's `replace` to exchange types between a client and server that do not conform to the spec.

Here's a type:

```rust
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Thing {
    a: String,
    #[schemars(skip)]
    b: String,
}
```

The OpenAPI spec (in `spec.json`) shows:

```json
"Thing": {
  "type": "object",
  "properties": {
    "a": {
      "type": "string"
    }
  },
  "required": [
    "a"
  ]
}
```

i.e., there is no property "b" as we'd expect.  But the server does emit it:

```rust
#[endpoint {
    method = GET,
    path = "/thing",
}]
async fn get_thing(
    _rqctx: RequestContext<()>,
) -> Result<HttpResponseOk<Thing>, HttpError> {
    Ok(HttpResponseOk(Thing { a: "ok".to_string(), b: "oops".to_string() }))
}
```

and the client can see it too!  The program uses the progenitor-generated client with "replace" to use its `Thing`, makes a request, and prints it out:

```rust
client got: Ok(Thing { a: "ok", b: "oops" })
```

On the other hand, if you remove the `replace` directive from the program:

```
$ git diff
diff --git a/src/main.rs b/src/main.rs
index f732ada..c34c4bb 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -16,7 +16,7 @@ use serde::Serialize;

 progenitor::generate_api!(
     spec = "spec.json", // The OpenAPI document
-    replace = { Thing = Thing }
+    // replace = { Thing = Thing }
 );

 #[tokio::main]
```

the client prints:

```
client got: Ok(Thing { a: "ok" })
```
