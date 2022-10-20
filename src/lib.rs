mod actor;
mod blockstore;
mod error;
mod state;

pub const INIT_ACTOR_ADDR: fvm_shared::ActorID = 1;

pub type Result<T> = std::result::Result<T, crate::error::Error>;

// copied from https://github.com/raulk/fil-hello-world-actor/blob/master/src/lib.rs
macro_rules! abort {
    ($code:ident, $msg:literal $(, $ex:expr)*) => {
        fvm_sdk::vm::abort(
            fvm_shared::error::ExitCode::$code.value(),
            Some(format!($msg, $($ex,)*).as_str()),
        )
    };
}
pub(crate) use abort;
