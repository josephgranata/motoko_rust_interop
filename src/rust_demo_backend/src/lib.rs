mod types;
mod store;

use ic_cdk::api::management_canister::main::{CanisterIdRecord, deposit_cycles};
use ic_cdk_macros::{init, update, pre_upgrade, post_upgrade, query};
use ic_cdk::api::{canister_balance128, caller, trap};
use ic_cdk::export::candid::{candid_method, export_service};
use ic_cdk::{storage, print};
use candid::{Principal};
use std::cell::RefCell;
use std::collections::HashMap;

use crate::store::{commit_batch, create_batch, create_chunk, get_asset_for_url};
use crate::types::{interface::{InitUpload, UploadChunk, CommitBatch}, storage::{AssetKey, State, Chunk, Asset}, http::{HttpRequest, HttpResponse}};

// https://medium.com/encode-club/encode-x-internet-computer-intro-to-building-on-the-ic-in-rust-video-slides-b496d6baad08
// https://github.com/hpeebles/rust-canister-demo/blob/master/todo/src/lib.rs

thread_local! {
    static STATE: RefCell<State> = RefCell::default();
}

// TODO: https://forum.dfinity.org/t/init-arg-mandatory-in-state/16009/ ?
// I would rather like to have a mandatory { owner: Principal } without having to assign a default value.

// TODO: test upgrade
// TODO: batches and chunks do not need to be in state

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

//
// Http
//

#[query]
#[candid_method(query)]
fn http_request(HttpRequest {method, url, headers: _, body: _}: HttpRequest) -> HttpResponse {
    if method != "GET" {
        return HttpResponse {
            body: "Method Not Allowed.".as_bytes().to_vec(),
            headers: Vec::new(),
            status_code: 405,
            streaming_strategy: None
        }
    }

    let result = get_asset_for_url(url);

    // TODO: stream strategy

    match result {
        Ok(Asset {key, headers, encoding}) => {
            return HttpResponse {
                body: encoding.contentChunks[0].clone(),
                headers,
                status_code: 200,
                streaming_strategy: None
            }
        }
        Err(_) => ()
    }

    return HttpResponse {
        body: "Permission denied. Could not perform this operation.".as_bytes().to_vec(),
        headers: Vec::new(),
        status_code: 405,
        streaming_strategy: None
    }
}

//
// Upload
//

#[allow(non_snake_case)]
#[candid_method(update)]
#[update]
async fn initUpload(key: AssetKey) -> InitUpload {
    // TODO: is caller === user

    let batchId: u128 = create_batch(key);
    return InitUpload { batchId };
}

#[allow(non_snake_case)]
#[candid_method(update)]
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
#[candid_method(update)]
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

#[allow(non_snake_case)]
#[candid_method(update)]
#[update]
async fn transferFreezingThresholdCycles() {
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

// Generate did files

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_candid() {
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;

        let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let dir = dir.parent().unwrap().parent().unwrap().join("src").join("rust_demo_backend");
        write(dir.join("rust_demo_backend.did"), export_candid()).expect("Write failed.");
    }
}