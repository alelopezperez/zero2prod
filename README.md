# zero2prod

## Cargo tips

- We can speed up build time via changer the linker, in the project.

  - `sudo apt-get install lld clang`
  <ul>

  ```
  # .cargo/config.toml

  [target.x86_64-unknown-linux-gnu]
  rustflags = ["-C", "linker=clang", "-C", "link-arg=-fuse-ld=lld"]

  ```

  </ul>

## TODO

- [ ] CI Pipleline

## Devtools for this project

### cargo-watch

It automatically trigger a build and run when a file changes. We can also add the test phase so it stops if any test fails.

`cargo watch -x check -x test -x run`

### rust-analyzer

The default LSP for rust.

## The Stack

### Actix-Web

#### **Server - HttpServer**

Handles all TCP Port, numbers and limits of concurrent connections, TLS, etc.

#### **Aplication - App**

Here were we do our app logic like routing (get, post, etc..), middleware,reques handler.

The **App** component takes an incoming request and passes it down to our `fn` handler where it's proccesed.

```
App::new()
.route("/", web::get().to(greet))
.route("/{name}", web::get().to(greet))
```

<ul>

#### Endpoint Route

1. path, a string that, that can be templated e.g. "/{name}"
2. route instance of route struct.

**Route**
Route struct uses guards from app, shorthand using web::get()

`.route("/", web::get().to(greet))` same as `Route::new().guard(guard::Get())` this means the request will be passed down **if and only if** it is a GET method.

our handler `async fn greet(req: HttpRequest) -> impl Responder` is an aync function that takes the request and returns something that implements the Responder `trait`.

The Responder `trait` is return like an HttpResponse

</ul>

#### **Runtime - Tokio**

We need to make main an async function, but the main needs to be syncronous, so we use the macro `#[toikio:main]` because who is in charge to call poll on it? if it's the main function.

<ul>

`cargo expand` to expand any macro, the `main` function is still syncronous. the macro just adds boilerplate to handle the async body of main, via the tokio-runtime.

</ul>

`async` is for value that needs to be polled to confirm its existance.

## CI - Continuos Integration

Using GitHub Actions

1. Tests `cargo test`
2. Code Coverrage `tarpaulin`
3. Lintin `clippy`
4. Formatting `rustfmt`
5. Security Vulnerabilities `cargo-audit`
