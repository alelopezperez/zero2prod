# zero2prod

## Curr Page: 88

## Cargo ans Rust tips

- We can speed up build time via changer the linker, in the project.

  - `sudo apt-get install lld clang`
  <ul>

  ```
  # .cargo/config.toml

  [target.x86_64-unknown-linux-gnu]
  rustflags = ["-C", "linker=clang", "-C", "link-arg=-fuse-ld=lld"]

  ```

  </ul>

- A binary proj it's different than a bin and lib project, since that way we can share code betweeen binaries using lib folder. Now we can use `use zero2prod::somthing`
- One lib file, multiple binaries, `[[bin]]` it's an array in TOML format since we can declare multiple binaries.

- `use crate` vs `use zero2prod`

  - `use crate` when inside a lib
  - `use zero2prod` when using main

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

### Loggin

- Using actix logger middleware we can use the wrap
- env_logger crate for log to the terminal
- #### Good Logging Practices
  - Have a request id associated the the logs to correalte or id which error pertains to which request

#### App Data

Actix works by default as statless, we can state to our with App Data, so the server need to return an app data struct so for DB connection it will be multiple instance of app data for each core. Since PG connection does not clone, we will use ARC for safe and atomic connection to the DB.

```
pub fn run(listener: TcpListener, connection: PgConnection) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}

```

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

#### **Serde**

SerDe: Serialize and Deserialize data structures in Rust generically. NO JSON(needs serde_json), serde defines a set of interfaces or, as they themselves call it, a data model

```
#[derive(serde::Deserialize)]
pub struct FormData {
email: String,
name: String,
}
// Let's start simple: we always return a 200 OK
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
HttpResponse::Ok().finish()
}
```

- before calling subscribe actix-web invokes the from_request method for all subscribe’s
  input arguments: in our case, Form::from_request;
- Form::from_request tries to deserialise the body into FormData according to the rules of URL-
  encoding leveraging serde_urlencoded and the Deserialize implementation of FormData,
  automatically generated for us by #[derive(serde::Deserialize)];
- if Form::from_request fails, a 400 BAD REQUEST is returned to the caller. If it succeeds,
  subscribe is invoked and we return a 200 OK

#### **Runtime - Tokio**

We need to make main an async function, but the main needs to be syncronous, so we use the macro `#[toikio:main]` because who is in charge to call poll on it? if it's the main function.

<ul>

`cargo expand` to expand any macro, the `main` function is still syncronous. the macro just adds boilerplate to handle the async body of main, via the tokio-runtime.

</ul>

`async` is for value that needs to be polled to confirm its existance.

We can run taks in background mode, so it will concurrent with any other task and futures. with `tokio::spawn`

### The Database (Postgres) - SQLX

Using `sqlx-cli` we can create our migrations from our db schema.

Postgres valid connections string `postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}`

## CI - Continuos Integration

Using GitHub Actions

1. Tests `cargo test`
2. Code Coverrage `tarpaulin`
3. Lintin `clippy`
4. Formatting `rustfmt`
5. Security Vulnerabilities `cargo-audit`

## Integration Tests

Integration Test is testing the API or any service/micro/program the same exact way an user would. In this case it'll be via HTTP Request since we are creating and API. **Black Box Testing** since we only pass input and expect output, _not testing any internals._

<ul>

### Caveats of Testing in Rust _Where to test_

We use the macro `#[cfg(test)]` for any test

1. Embedded Tests _in the same file as your code_

   Test in a folder or Doc, have a separate binary.

   Embedded tests visibilty has access to all struct, fn without the need for it to be `pub`

2. External Test Folder
   Ideal for integration testing since it generates a different binary
3. Doc Test
</ul>

### Table Driven Development

Instead of testing each bad possible input for we can use an array of kown incorrect input to test if we get the desired **400** status code.

```
let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to exec request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        )
    }
```

## SQLX - Rust DB ORM Crate

- `sqlx::query!()` macro; it takes a sql command e.g "SELECT \* from example..." and at compile time checks the connection, and checks sql correctness and existance; and also returns a anonymous record type of the fetch (i.e saved.email for the email column)
