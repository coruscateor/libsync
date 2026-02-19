<div align="center">

# LibSync

[![Crates.io](https://img.shields.io/crates/v/libsync)](https://crates.io/crates/libsync)
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue)](#license)
[![Downloads](https://img.shields.io/crates/d/libsync)](https://crates.io/crates/libsync)
[![Docs](https://docs.rs/libsync/badge.svg)](https://docs.rs/libsync/latest/libsync)
[![Twitch Status](https://img.shields.io/twitch/status/coruscateor)](https://www.twitch.tv/coruscateor)

[X](https://twitter.com/Coruscateor) | 
[Twitch](https://www.twitch.tv/coruscateor) | 
[Youtube](https://www.youtube.com/@coruscateor) | 
[Mastodon](https://mastodon.social/@Coruscateor) | 
[GitHub](https://github.com/coruscateor) | 
[GitHub Sponsors](https://github.com/sponsors/coruscateor) 

</div>

</br>

LibSync is a library which contains channel implementations and objects used to produce channel implementations.

Use the channel implementations in the crossbeam_queue and scc modules as the crossbeam module is probably going to be removed. 

</br>

## Valid Features

| Feature                   | Description |
| -------                   | ----------- |
| crossbeam                 | Enable the crossbeam sub-module. |
| crossbeam-queue           | Enable the crossbeam-queue sub-module. |
| tokio                     | Enable the tokio-helpers sub-module and relevant tests. |
| std                       | Enable the std sub-module. |
| use_std_sync              | Use std synchronisation objects. |
| use_parking_lot_sync      | Use parking_lot synchronisation objects. |
| use_parking_lot_fair_sync | Use fair parking_lot synchronisation objects where possible. |

</br>

## Todo

- Add more documentation
- Add more tests
- Decide on what is staying in the library.
- Add async-runtime specific functionality to the crossbeam_queue and scc oriented channels (e.g. timeout methods). 
- Add an std VecDeque oriented channel implementation.

</br>

## Maybe

- Add more channel implementations using queue implementations of other crates that this crate doesn’t already conditionally depend on.

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
