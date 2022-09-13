# diesel_sqlite

## Overview

Sample project to get the Rust SQL crate, [diesel](https://github.com/diesel-rs/diesel), working with the [time](https://github.com/time-rs/time) crate. Removing `Insertable` from derive allows the program to continue compiling but I'm under the impression that I need to use the same struct to insert since `DateTime` is just handled as `TEXT` in sqlite.

Using rustc 1.63.0 (4b91a6ea7 2022-08-08)

## Resolution

For all the datetime columns in my `src/schema.rs`, I switched the column type to `TimestamptzSqlite`. Although even though the underlying sqlite type is `TEXT` the diesel schema needs to know you'd like to treat it as a `OffsetDateTime`. To make the change permanent between schema runs I created the `schema.patch` file in the root directory.
