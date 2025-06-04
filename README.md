# axum template

#### an opinionated template for [axum](https://github.com/tokio-rs/axum).

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

### core layer

as the name implies, this is the core of the app, state management, caching, tasks, services, "managers", are all inside
this crate.

this layer does not directly depend on objects, types and models specific to the api, so the api layer can be replaced
with alternatives while keeping the core logic unchanged.

### db layer

this layer is basically a "wrapper" over the db queries, making the organized in related units, while providing some
useful tools like metric collections dynamically.

in this design, the intention of this crate was not portability, so the database might not be easy to replace.


## Dependencies

this templates uses the following main dependencies:

- `axum` the web framework
- `tokio`: the async runtime
- `opentelemetry`: metric collector
- `ts-rs`: utility for converting types into typescript objects for the frontend
- `sqlx`: for writing direct SQL queries.
- `sea-orm`: an orm included for some use cases.

