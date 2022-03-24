mkdir doc
RUSTDOCFLAGS="--cfg docsrs" cargo doc --features use_tokio --no-deps
mv target/doc/ doc/use_tokio
RUSTDOCFLAGS="--cfg docsrs" cargo doc --features use_async-std --no-deps
mv target/doc/ doc/use_async-std
RUSTDOCFLAGS="--cfg docsrs" cargo doc --features use_neovim_lib --no-deps
mv target/doc/ doc/use_neovim_lib
