//! # Quitting
//!
//! A very small example showing how to handle quitting an application.
//!
//! * Running as an rpc plugin, just have neovim close the channel corresponding
//! to our plugin
//! * When embedding neovim, do the same via a command, as shown in this
//! example. Note that this final request _will_ receive an error, since it will
//! not get an answer from neovim.
//!
//! Also note that all other pending requests will receive an EOF error as well.
//!
//! ## Usage
//!
//! First, build the neovim included as a submodule:
//!
//! ```sh
//! cd neovim
//! make
//! ```
//!
//! See https://github.com/neovim/neovim/wiki/Building-Neovim for build options.
//! Nothing is really needed to run the example.
//!
//! After that, run the example via
//!
//! ```sh
//! cargo run --example quitting
//! ```
//!
//! ## Description
//!
//! Some overview over the code:
//!
//! * Since we're not interested in handling anything, our `NeovimHandler` is an
//! empty struct. We need not implement any methods, but declare the associated
//! type `Handler::Writer`, which is `nvim_rs::runtime::ChildStdin` here since
//! we connect to a neovim subprocess.
//!
//! * We need to spawn the io future still, because it runs the loop that
//! notices the EOF and subsequently sends the errors to the pending requests.
//! We could not bother, and run the `command` with a timeout.
//!
//! * Any shutdown logic should be handled after the channel was closed. We
//! don't actually need to inspect the error, since the application will shut
//! down no matter what. We don't have access to our `Handler` anymore though,
//! so nvim-rs will need to be extended.
