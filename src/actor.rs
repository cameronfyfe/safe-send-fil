use crate::{abort, error::Error, state, state::State, Result};
use fvm_ipld_encoding::{to_vec, RawBytes, DAG_CBOR};
use fvm_sdk::NO_DATA_BLOCK_ID;
use fvm_shared::{address::Address, econ::TokenAmount, error::ExitCode, METHOD_SEND};
use serde::{Deserialize, Serialize};

// TODO: replace all .unwrap() and .expect() with proper error reporting

/// Entrypoint
#[no_mangle]
pub fn invoke(params: u32) -> u32 {
    let result = match fvm_sdk::message::method_number() {
        1 => constructor(),
        2 => new_transfer(params),
        3 => accept_transfers(),
        4 => list_transfers(),
        _ => abort!(USR_UNHANDLED_MESSAGE, "Unrecognized method number."),
    };

    let result = match result {
        Ok(result) => result,
        Err(err) => err.abort(),
    };

    let return_block_id = match result {
        Some(v) => match fvm_sdk::ipld::put_block(DAG_CBOR, v.bytes()) {
            Ok(id) => id,
            Err(err) => abort!(USR_SERIALIZATION, "Failed to store return value: {}", err),
        },
        None => NO_DATA_BLOCK_ID,
    };

    return_block_id
}

// ***
// *** Method 1 - Constructor
// ***

/// Constructor
pub fn constructor() -> Result<Option<RawBytes>> {
    let state = State::default();
    state.save();

    Ok(None)
}

// ***
// *** Method 2 - New Transfer
// ***

/// Parameters for [`new_transfer`].
#[derive(Deserialize, Debug)]
pub struct NewTransferParams {
    pub hold_time: u32,
    /// Amount of nanoFIL to transfer
    pub amount: u64,
    /// Destination address of transfer
    pub destination: String,
}

/// Request new transfer
pub fn new_transfer(params: u32) -> Result<Option<RawBytes>> {
    let (_, params) = fvm_sdk::message::params_raw(params).map_err(|e| {
        Error::new(
            Some(ExitCode::USR_SERIALIZATION),
            format!("Failed to retrieve params block: error number = {e}."),
        )
    })?;
    let params = String::from_utf8(params).map_err(|e| {
        Error::new(
            Some(ExitCode::USR_SERIALIZATION),
            format!("Failed to deserialize params block to string: {e}."),
        )
    })?;
    let params = serde_json::from_str(&params).map_err(|e| {
        Error::new(
            Some(ExitCode::USR_SERIALIZATION),
            format!("Failed to deserialize params string to json. String:{params}; Error:{e}"),
        )
    })?;

    // let start_epoch = fvm_sdk::network::curr_epoch();
    let start_epoch = 5; // TODO: fvm_sdk::network::curr_epoch() fails.

    let NewTransferParams {
        hold_time,
        amount,
        destination,
    } = params;
    let amount = TokenAmount::from_nano(amount);
    let _ = destination;
    let destination = TEST_ADDR.to_owned(); // TODO: remove and use actual caller based on message info

    let mut state = State::load();
    state.transfers.push(state::Transfer {
        start_epoch,
        hold_time,
        amount,
        destination,
    });
    state.save();

    Ok(None)
}

// ***
// *** Method 3 - Accept Transfers
// ***

/// Accept all active transfers destined for caller.
pub fn accept_transfers() -> Result<Option<RawBytes>> {
    // let caller_address = Address::new_id(fvm_sdk::message::caller());
    // TODO: get caller_address from caller id
    let caller_address = Address::from_bytes(&TEST_ADDR.as_bytes()[1..]).unwrap();

    let mut state = State::load();

    let mut received_amount = TokenAmount::from_atto(0);
    state.transfers.retain(|transfer| {
        let destination_address =
            Address::from_bytes(&transfer.destination.as_bytes()[1..]).unwrap();
        if destination_address == caller_address {
            received_amount += transfer.amount.clone();
            false
        } else {
            true
        }
    });

    // Send FIL to caller
    let _ = fvm_sdk::send::send(
        &caller_address,
        METHOD_SEND,
        RawBytes::default(),
        received_amount,
    )
    .unwrap();

    Ok(None)
}

// ***
// *** Method 4 - List Transfers
// ***

/// Part of response to [`list_transfers`].
#[derive(Debug, Serialize)]
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

impl From<state::Transfer> for Transfer {
    fn from(t: state::Transfer) -> Self {
        let state::Transfer {
            start_epoch,
            hold_time,
            amount,
            destination,
        } = t;
        Self {
            start_epoch,
            hold_time,
            amount,
            destination,
        }
    }
}

/// List active transfers.
pub fn list_transfers() -> Result<Option<RawBytes>> {
    let state = State::load();

    let transfers = state
        .transfers
        .into_iter()
        .map(Into::into)
        .collect::<Vec<Transfer>>();

    response(&transfers)
}

fn response<T>(value: &T) -> Result<Option<RawBytes>>
where
    T: serde::Serialize + ?Sized,
{
    let s = serde_json::to_string(value).unwrap();
    match to_vec(&s) {
        Ok(response) => Ok(Some(RawBytes::new(response))),
        Err(err) => Err(Error::new(
            Some(ExitCode::USR_ILLEGAL_STATE),
            format!("Failed to serialize return value: {:?}", err),
        )),
    }
}

// TODO: remove
const TEST_ADDR: &str =
    "t3rpdccjp6bn35s4lp7inacy3bi2kok4gefdh6np7u5kfwbux3omis6dehnmgswqnxrmzucal22jyn7ozgk7wq";
