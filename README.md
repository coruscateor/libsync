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

LibSync is a library which contains objects which can be used in the synchronisation of application threads.

This this library is structured to compound the features of other “lower level” libraries e.g. it combines crossbeam ArrayQueue objects with Tokio Notify objects to produce a multi-producer-multi-consumer channel that can be waited on asynchronously at both ends.

</div>

</br>

## Todo

- Add crossbeam::mpmc::tokio::seg_queue implementation.
- Complete std::Notifier
- Add documentation
- Add tests

</br>

## Coding Style

This project uses a coding style the emphasises the use of white space over keeping the line and column counts as low as possible.

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
