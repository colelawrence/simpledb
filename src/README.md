### SimpleDB

SimpleDB is a Rust-based exercise to get some hands on Rust experience. It is probably not great, because I wrote it, and I don't know Rust.

#### What's the goal?

Under the `tests/` directory, you will find a set of `integration` (as Rust calls them, but I say they are `unit` tests) that exercise the behaviour of an `InMemoryDB`, that satisfies the `SimpleDB` trait. You should make all of these tests pass. I'd suggest commenting them all out and going one by one.

Implementations of the `SimpleDB` trait are expected to handle:
 - Setting/Unsetting key/values (simple Strings and u32s)
 - Getting values by key
 - Starting transactions (which can be nested)
 - Rolling back (most recent) and committing (all) transactions.

You can run the tests using `cargo test`, and I'd suggest you run them with `RUST_BACKTRACE=1` set in your environment, because there are some helper functions which result in annoying traces.

Again, I'd suggest commenting out all the tests and going one by one.

#### Anything else?

If you have all the tests passing, here's some other things to consider:
 - Could `SimpleDB` handle generic types rather than strings/u32s?
 - What other storages might be interesting other than `InMemory`?
 - How would you handle concurrency?