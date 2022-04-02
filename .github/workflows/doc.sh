mkdir doc
RUSTDOCFLAGS="--cfg docsrs" cargo doc --features use_tokio --no-deps
mv target/doc/ doc/use_tokio
RUSTDOCFLAGS="--cfg docsrs" cargo doc --features use_async-std --no-deps
mv target/doc/ doc/use_async-std
