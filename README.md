# axum template

#### an opinionated template for [axum](https://github.com/tokio-rs/axum). aiming for mid to big scale projects.

content:

- [Architecture](#Architecture)
- [Dependencies](#Dependencies)

## Architecture

this template contains the following crates:

* `app`: main application entry point
* `lib-api`: main components of the REST layer, contains specific modules for http interactions, replaceable by
  alternative implementations (grpc ...etc.)
* `lib-core`: the core business logic of the applications. this is the where all the ~~pain~~ fun exists.
* `lib-db`: data access layer, containing connection pools, queries, wrappers and db-specific logics.
* `lib-shared`: shared objects, functions and utilities across the upper layers.

this design is somewhat similar to "onion architecture", with modifications.

### app layer

this crate should only contain initial steps that are required for the service to run, right before the main "loop".
initializations such as logs, tracing, metric collectors, event dispatchers ...etc.

it does **not** contain any api specific code, thats the responsibility of the next crate.

### api layer

this layer contains code that is mainly related to the nature of REST apis and HTTP communications.

it does not handle any logic directly, but rather calling them through the 3rd layer.

authentication & authorizations are partially handled by this layer, for example catching `Authorization` header, is
handled by api layer, but parsing its contents and validations are left in the core layer.

#### structure

`components`:

this directory contains routes and endpoints for the api, with their respective subdirectory.
for example in the template you have an `auth` dir, all the auth related endpoints and their models live here, if you
had endpoints for other matters, they would have their own dir.

`middlewars`:

this contains all of your middlewares and interceptors, because they are usually one or to functions they dont need
separate directory.
often placing them in `mod.rs` is okay.

`models`:
this contains every model that is shared across all directories, so they are not specific to some endpoints or
middleware.

note that you have to respect dependency inversion and not making depending your `lib-core` on the `lib-api` models.

`utils`:

this is where all the random crappy codes lives, usually stuff that cant be placed directly in components or they are
used in multiple places.

in the template there are some useful(or not) macros for returning response from the endpoints.

### core layer

as the name implies, this is the core of the app, state management, caching, tasks, services, "managers", are all inside
this crate.

this layer does not directly depend on objects, types and models specific to the api, so the api layer can be replaced
with alternatives while keeping the core logic unchanged.

#### structure

`managers`:

this is a sensitive place, it can easily turn into an antipattern or break SRP.

code inside this directory should focus on shared state across the core and potentially the api.
for example the `cache-manager` in the sample can be used for storing data related to any `component`, but it should not
do more than just holding the `cache`, otherwise it would turn into `*-manager , *-service` hell.

`services`:

this is the core logic of your app, it breaks into individual components, each related to **one** task.

for example `auth_service` is responsible for *AAA* and nothing more.

this design allows the developer to latter break this core into a microservice architecture with minimal modifications.
each directory inside the `services` is a standalone microservice, alongside with its models ...etc.

since this layer depends on `lib-db` and `lib-shared`, all microservices inherit this dependency without any issues.

`app_state`:

this is the entry point for all shared states across the `lib-api`, this is how you access most of your state-full codes
from the api. state-less codes are obviously called directly from the api endpoint.

### db layer

this layer is basically a "wrapper" over the db queries, making the organized in related units, while providing some
useful tools like metric collections dynamically.

in this design, the intention of this crate was not portability, so the database might not be easy to replace.

#### structure

`lib.rs`

there is one important macro placed in this file called `impl_psql_driver`. this macro write the boilerplate for a new
db "driver". e.x:

```rust
impl_psql_driver!(UserDriver,ProductDriver,TestDriver);
```

this creates a `PsqlDriver` and structs called `UserDriver`, `ProductDriver`, `TestDriver` and implements the default
functions.

`PsqlDriver`:

this object contains all the drivers, so in your core you would use `state.psql.user_driver` or other drivers to access
the queries.

a db driver is a wrapper over methods specific to an area or "table". so `UserDriver`, `ProductDriver`,
`TransactionsDriver` are some of the potential drivers for the codebase.

`psql_connection`:

this file contains a wrapper around the connection pool.
main purpose if this wrapper is to collect metrics dynamically;

when the pool guard drops, it sends the related metrics in a oneshot channel, and on the other side the metrics are sent
to open-telemetry.
this guard is necessary to collect the name of the method and its execution time, so do not access the pool directly
otherwise metrics won't be collected.

`drivers`:

each driver lives in its own file, it only contains the queries and db calls, models are separated in another
directory (manually or automatically generated).

the db connection is automatically passed to every driver, so you would use it from self: `*self.connection.db()` or
`.orm()` for seaorm.

## Dependencies

these templates use the following main dependencies:

- `axum` the web framework
- `tokio`: the async runtime
- `opentelemetry`: metric collector
- `ts-rs`: utility for converting types into typescript objects for the frontend
- `sqlx`: for writing direct SQL queries.
- `sea-orm`: an orm included for some use cases.

