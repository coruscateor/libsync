<div align="center">

# LibSync

[![Crates.io](https://img.shields.io/crates/v/gtk_estate)](https://crates.io/crates/libsync)
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue)](#license)
[![Downloads](https://img.shields.io/crates/d/gtk_estate)](https://crates.io/crates/libsync)
[![Docs](https://docs.rs/gtk_estate/badge.svg)](https://docs.rs/gtk_estate/latest/libsync)
[![Twitch Status](https://img.shields.io/twitch/status/coruscateor)](https://www.twitch.tv/coruscateor)

[X](https://twitter.com/Coruscateor) | 
[Twitch](https://www.twitch.tv/coruscateor) | 
[Youtube](https://www.youtube.com/@coruscateor) | 
[Mastodon](https://mastodon.social/@Coruscateor) | 
[GitHub](https://github.com/coruscateor) | 
[GitHub Sponsors](https://github.com/sponsors/coruscateor) 

</div>

</br>

LibSync is a library which contains objects which can be used in the synchronisation of application threads.

This this library combines the features of “lower level” library objects e.g. it combines crossbeam ArrayQueue objects with Tokio Semaphore objects to produce a multi-producer-multi-consumer channel that can be waited on asynchronously on both sides.

It also has objects that help you keep count of messages in a pipeline and "return stores" for "returning" values from one thread to another using a reusable location.

In addition to message counters, you may find IO channels useful when working with actors.

</br>

## Todo

- Add more documentation
- Add tests
- Decide on what is staying in the library.

</br>

## Coding Style

This project uses a coding style that emphasises the use of white space over keeping the line and column counts as low as possible.

So this:

```rust
fn bar() {}

fn foo()
{

    bar();

}

```

Not this:

```rust
fn bar() {}

fn foo()
{
    bar();
}

```

<br/>

## License

Licensed under either of:

- Apache License, Version 2.0, ([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0 (see also: https://www.tldrlegal.com/license/apache-license-2-0-apache-2-0))
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT (see also: https://www.tldrlegal.com/license/mit-license))

at your discretion

<br/>

## Contributing

Please clone the repository and create an issue explaining what feature or features you'd like to add or bug or bugs you'd like to fix and perhaps how you intend to implement these additions or fixes. Try to include details though it doesn't need to be exhaustive and we'll take it from there (dependant on availability).

<br/>

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
