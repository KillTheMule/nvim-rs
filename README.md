# nvim-rs [![Travis Status](https://travis-ci.org/KillTheMule/nvim-rs.svg?branch=master)](https://travis-ci.org/KillTheMule/nvim-rs) [![Appveyor status](https://ci.appveyor.com/api/projects/status/9altsynjuqavb8jw/branch/master?svg=true)](https://ci.appveyor.com/project/KillTheMule/nvim-rs/branch/master)
Rust library for Neovim msgpack-rpc clients. Utilizes async to allow for arbitrary nesting of requests.

## Status

Useable, see the `examples/` and `tests/` folders for examples. The `nvim_rs::examples` submodule contains documentation of the examples.

The **API** is unstable, see the [Roadmap](https://github.com/KillTheMule/nvim-rs/issues/1) for things being planned.

## Contributing

I'd love contributions, comments, praise, criticism... You could open an [issue](https://github.com/KillTheMule/nvim-rs/issues) or a [pull request](https://github.com/KillTheMule/nvim-rs/pulls), or if you want a direct contact, meet me in the [neovim gitter channel](https://gitter.im/neovim/neovim). I also read the subreddits for [rust](https://www.reddit.com/r/rust/) and [neovim](https://www.reddit.com/r/neovim/), if that suits you better.

## Running tests

For some of the tests, the neovim source included as a submodule needs to be
compiled. 

- On Linux and OSX, after checking out the source, it should be as
simple as `cd neovim && make`.

- On Windows, follow the build instructions [from the neovim
  wiki](https://github.com/neovim/neovim/wiki/Building-Neovim).

After building neovim, you can simply run `cargo test`. Also run `cargo build
--examples` as well as `cargo bench -- --test` to make sure everything still
compiles.

## License

As this is a fork of [neovim-lib](https://github.com/daa84/neovim-lib), it is licensed under the GNU Lesser General Public License v3.0.

## CoC

Wherever applicable, this project follows the [rust code of
conduct](https://www.rust-lang.org/en-US/conduct.html).
