mod types;
mod store;

use ic_cdk::api::management_canister::main::{CanisterIdRecord, deposit_cycles};
use ic_cdk::api::{canister_balance128, caller, trap};
use ic_cdk::{storage, print};
use candid::{Principal};
use ic_cdk_macros::{init, query, update, pre_upgrade, post_upgrade};
use std::cell::RefCell;
use std::collections::HashMap;

use crate::store::{commit_batch, create_batch, create_chunk};
use crate::types::{interface::{InitUpload, UploadChunk,CommitBatch}, storage::{AssetKey, State, Chunk}};

// https://medium.com/encode-club/encode-x-internet-computer-intro-to-building-on-the-ic-in-rust-video-slides-b496d6baad08
// https://github.com/hpeebles/rust-canister-demo/blob/master/todo/src/lib.rs

thread_local! {
    static STATE: RefCell<State> = RefCell::default();
}

// TODO: https://forum.dfinity.org/t/init-arg-mandatory-in-state/16009/ ?
// I would rather like to have a mandatory { owner: Principal } without having to assign a default value.

#[init]
fn init(user: Principal) {
    print(format!("Initializing bucket., {}", user.to_text()));
    STATE.with(|state| {
        *state.borrow_mut() = State {
            user: Some(user),
            batches: HashMap::new(),
            chunks: HashMap::new(),
            assets: HashMap::new(),
        };
    });
}

#[pre_upgrade]
fn pre_upgrade() {
    STATE.with(|state| storage::stable_save((&state.borrow().user, )).unwrap());
}

#[post_upgrade]
fn post_upgrade() {
    // TODO: pre and post upgrade

    // let (owner,): (Option<Principal>,) = storage::stable_restore().unwrap();
    // let new_state: State = State { owner };
    // STATE.with(|state| {
    //     *state.borrow_mut() = new_state;
    // });
}

#[query]
fn greet(name: String) -> String {
    let owner: Option<Principal> = STATE.with(|state| get_owner(&state.borrow()));

    match owner {
        Some(owner) => format!("Hello super, {}, {o}!", name, o = owner.to_text()),
        None => String::from("No owner."),
    }
}

fn get_owner(state: &State) -> Option<Principal> {
    state.user
}

//
// Upload
//

#[allow(non_snake_case)]
#[update]
async fn initUpload(key: AssetKey) -> InitUpload {
    // TODO: is caller === user

    let batchId: u128 = create_batch(key);
    return InitUpload { batchId };
}

#[allow(non_snake_case)]
#[update]
async fn uploadChunk(chunk: Chunk) -> UploadChunk {
    // TODO: is caller === user

    let result = create_chunk(chunk);

    match result {
        Ok(chunk_id) => { UploadChunk { chunkId: chunk_id } }
        Err(error) => trap(error)
    }
}

#[allow(non_snake_case)]
#[update]
async fn commitUpload(commit: CommitBatch) {
    // TODO: is caller === user

    let result = commit_batch(commit);

    match result {
        Ok(_) => (),
        Err(error) => trap(error)
    }
}

//
// Canister mgmt
//

#[update]
async fn transfer_cycles() {
    let caller = caller();

    // TODO: is caller === manager

    // TODO: determine effective threshold - how few cycles should be retained before deleting the canister?
    // use freezing_threshold_in_cycles? - https://github.com/dfinity/interface-spec/pull/18/files
    // actually above PR was ultimately deleted? - https://forum.dfinity.org/t/minimal-cycles-to-delete-canister/15926

    // Source: https://forum.dfinity.org/t/candid-nat-to-u128/16016
    let balance: u128 = canister_balance128();
    let cycles: u128 = balance - 100_000_000_000u128;

    if cycles > 0 {
        let arg_deposit = CanisterIdRecord { canister_id: caller };
        deposit_cycles(arg_deposit, cycles).await.unwrap();
    }
}