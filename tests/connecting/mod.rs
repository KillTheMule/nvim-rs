#[cfg(feature = "use_tokio")]
pub mod conns;
#[cfg(feature = "use_smol")]
pub mod conns;

#[cfg(feature = "use_tokio")]
pub mod handshake;
#[cfg(feature = "use_tokio")]
macro_rules! atest { // taken from smol::lib.rs
    // Special case to get around bug in macro engine.
    (
        $(#[$post_attr:meta])*
        async fn $name:ident ($exname:ident : & $exty:ty)
        $(-> $ret:ty)? $bl:block
    ) => {
        #[tokio::test]
        $(#[$post_attr])*
        async fn $name($exname: &$exty) $(-> $ret)? $bl
    };

    (
        $(#[$post_attr:meta])*
        async fn $name:ident ($($pname:ident : $pty:ty),* $(,)?)
        $(-> $ret:ty)? $bl:block
    ) => {
        #[tokio::test] 
        $(#[$post_attr])*
        async fn $name($($pname: $pty),*) $(-> $ret)? $bl
    };
}
#[cfg(feature = "use_tokio")]
pub(crate) use atest;


#[cfg(feature = "use_smol")]
use smol_macros::test as atest;
