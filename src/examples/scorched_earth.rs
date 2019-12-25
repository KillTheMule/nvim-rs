//! # Scorched earth
//!
//! A minimal port of [scorched earth by boxofrox](https://github.com/boxofrox/neovim-scorched-earth) to
//! nvim-rs. Works the same, but foregoes any error handling, removes the
//! customisation of color, and removes some abstractions that aren't helpfull
//! in an example.
//!
//! ## Usage
//!
//! First, build this example via
//!
//! ```sh
//! cargo build --example scorched_earth
//! ```
//!
//! The binary will be located in `target/debug/examples/scorched_earth`.
//!
//! Follow the
//! [steps](https://github.com/boxofrox/neovim-scorched-earth#try-it-out)
//! described for the original plugin. Of course, you don't need to build it.
//! Before step 4, put
//!
//! ```viml
//! let g:scorched_earth_program = '<path to nvim-rs>/target/debug/examples/scorched_earth'
//! ```
//!
//! into `init.vim`. That's it, fire it up and enjoy.
//!
//! ## Description
//!
//! Some overview over the code:
//!
//! * The handler struct `NeovimHandler` needs to contain some plugin state,
//!   namely two
//! cursor positions `start` and `end`. It needs to be `Send` and `Sync`, and we
//! need mutable access, so we wrap it in a `Arc<Mutex<_>>`. Note that we're
//! using the [`Mutex`](crate::runtime::Mutex) from `nvim-rs`, which is a
//! re-export from Tokio.
//!
//! * Implementing the [`Handler`](crate::Handler) trait requires some magic
//! because of the async functions, we we use the
//! [`async_trait`](https://docs.rs/async-trait/0.1.21/async_trait/) macro.
//!
//! * We use `Stdout` as the type for the `Writer` because neovim acts as our
//! parent, so it reads from our stdout. Note that this is the [async
//! version](crate::runtime::Stdout) re-exported from Tokio.
//!
//! * We only implement `handle_notify` since we don't want to serve requests.
//! It gets a [`Neovim`](crate::Neovim) passed that we can use to send
//! requests to neovim. All requests are async methods, so we need to `await`
//! them.
//!
//! * The main function is denoted `#[tokio::main]` to use async notation, but
//! it would be perfectly feasible to explicitely create a runtime and use that.
//!
//! * After creation of the handler, we connect to neovim via one of the
//! [`create`](crate::create) functions. It gives back a
//! [`Neovim`](crate::Neovim) instance which we could use for requests, and a
//! `Future` which implements the IO, so we need to run it.
//!
//! * The plugin quits by ending the IO task when neovim closes the channel, so
//!   we don't need to do anything special. Any cleanup-logic can happen after
//!   the IO task has finished. Note that we're loosing access to our
//!   [`Handler`](crate::Handler), which isn't optimal and will need to be
//!   fixed.
//!
//! * After the IO task has finished, we're inspecting the errors to see why it
//! went. First, if we did not see a general reader error, we try to send some
//! last notification to the neovim user. Secondly, we quietly ignore the
//! channel being closed, because this usually means that it was closed by
//! neovim, which isn't always an error.
//!
//!   *Note*: A closed channel could still mean an error, so the plugin has the
//!   option to react to this.
