# RUST Actix-Web Login Microservice
RUST, Actix, Login, Argonautica, Diesel, SQLite

### Description

- Register with username and password
- Login with username and password ➡ Get verified and receive auth cookie
- Update password with auth cookie
- Argonautica for password hashing
- Sqlite3 embedded db for user database

### Crate Used

- [actix-web](https://crates.io/crates/actix-web) // Actix web is a simple, pragmatic and extremely fast web framework for Rust.
- [argonautica](https://docs.rs/argonautica) // crate for hashing passwords using the cryptographically-secure Argon2 hashing algorithm.
- [chrono](https://crates.io/crates/chrono) // Date and time library for Rust.
- [diesel](https://crates.io/crates/diesel) // A safe, extensible ORM and Query Builder for PostgreSQL, SQLite, and MySQL.
- [dotenv](https://crates.io/crates/dotenv) // A dotenv implementation for Rust.
- [derive_more](https://crates.io/crates/derive_more) // Convenience macros to derive tarits easily
- [env_logger](https://crates.io/crates/env_logger) // A logging implementation for log which is configured via an environment variable.
- [futures](https://crates.io/crates/futures) // An implementation of futures and streams featuring zero allocations, composability, and iterator-like interfaces.
- [lazy_static](https://docs.rs/lazy_static) // A macro for declaring lazily evaluated statics.
- [r2d2](https://crates.io/crates/r2d2) // A generic connection pool.
- [serde](https://crates.io/crates/serde) // A generic serialization/deserialization framework.
- [serde_json](https://crates.io/crates/serde_json) // A JSON serialization file format.
- [serde_derive](https://crates.io/crates/serde_derive) // Macros 1.1 implementation of #[derive(Serialize, Deserialize)].

## Build

```
$ echo "DATABASE_URL=user.db" > .env
$ diesel migration run

$ cargo check 
$ cargo run 

```
