use crate::blockstore::Blockstore;

use cid::multihash::Code;
use cid::Cid;
use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};
use fvm_ipld_encoding::{to_vec, CborStore, DAG_CBOR};
use fvm_sdk as sdk;
use fvm_shared::econ::TokenAmount;

macro_rules! abort {
    ($code:ident, $msg:literal $(, $ex:expr)*) => {
        fvm_sdk::vm::abort(
            fvm_shared::error::ExitCode::$code.value(),
            Some(format!($msg, $($ex,)*).as_str()),
        )
    };
}

/// The state object.
#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, Default)]
pub struct State {
    /// Active transfers (not yet fulfilled or expired)
    pub transfers: Vec<Transfer>,
}

/// Part of [`State`].
#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, Default)]
pub struct Transfer {
    /// Epoch when transfer was started
    pub start_epoch: i64,
    /// Number of epochs to hold transfer funds
    pub hold_time: u32,
    /// Amount of FIL to transfer
    pub amount: TokenAmount,
    /// Destination address of transfer
    pub destination: String,
}

impl State {
    pub fn load() -> Self {
        let root = match sdk::sself::root() {
            Ok(root) => root,
            Err(err) => abort!(USR_ILLEGAL_STATE, "Failed to get root: {:?}", err),
        };
        match Blockstore.get_cbor::<Self>(&root) {
            Ok(Some(state)) => state,
            Ok(None) => abort!(USR_ILLEGAL_STATE, "State does not exist"),
            Err(err) => abort!(USR_ILLEGAL_STATE, "Failed to get state: {}", err),
        }
    }

    pub fn save(&self) -> Cid {
        let serialized = match to_vec(self) {
            Ok(s) => s,
            Err(err) => abort!(USR_SERIALIZATION, "Failed to serialize state: {:?}", err),
        };
        let cid = match sdk::ipld::put(Code::Blake2b256.into(), 32, DAG_CBOR, serialized.as_slice())
        {
            Ok(cid) => cid,
            Err(err) => abort!(USR_SERIALIZATION, "Failed to store initial state: {:}", err),
        };
        if let Err(err) = sdk::sself::set_root(&cid) {
            abort!(USR_ILLEGAL_STATE, "Failed to set root ciid: {:}", err);
        }
        cid
    }
}
