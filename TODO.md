* Check what we're doing with outgoing request parameters, there are 2 allocations going on in call_args! and rpc_args!, and we're just reading it in the end.

* Can we use the non-generic `split` methods from tokio for unixstream, tcpstream? Supposedly better performance, but introduces lifetimes...

* Don't return an error on channel close, because the the regular way to shut down a plugin
  --> maybe? For a plugin, sure, what about GUIs?

* We don't use `model::encode` anymore, remove it. Do the tests with the sync version.

* Move Queue/SharedQueue from neovim.rs to their own module, make the latter it's
  own type and augment the lock method with a check for being closed. We should not
  make ourselves have to remember to call `ensure_closed` 
