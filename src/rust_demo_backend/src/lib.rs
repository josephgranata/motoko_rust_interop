mod types;
mod store;

use ic_cdk::api::management_canister::main::{ canister_status, CanisterIdRecord, deposit_cycles };
use ic_cdk::{ api, storage, print };
use candid::{ Nat, Principal };
use ic_cdk_macros::{ init, query, update, pre_upgrade, post_upgrade };
use std::cell::RefCell;
use std::collections::HashMap;

use crate::types::State;

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
            assets: HashMap::new()
        };
    });
}

#[pre_upgrade]
fn pre_upgrade() {
    STATE.with(|state| storage::stable_save((&state.borrow().user,)).unwrap());
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
    state.owner
}

/**
* Upload
*/

#[allow(non_snake_case)]
async fn initUpload() {}

/**
 * Canister mgmt
 */

#[update]
async fn transfer_cycles() {
    let caller = api::caller();

    // TODO: determine effective threshold - get freezing_threshold_in_cycles via ic.canister_status()
    // use freezing_threshold_in_cycles - https://github.com/dfinity/interface-spec/pull/18/files
    // https://forum.dfinity.org/t/minimal-cycles-to-delete-canister/15926

    // TODO: is caller === manager

    let arg = CanisterIdRecord { canister_id: ic_cdk::id() };
    let response = canister_status(arg).await.unwrap().0;
    let cycles: Nat = response.cycles - Nat::from(100_000_000_000u128);

    print(format!("Current cycles {}", cycles));

    if cycles > Nat::from(0) {
        let arg_deposit = CanisterIdRecord { canister_id: caller };
        // Source: https://forum.dfinity.org/t/candid-nat-to-u128/16016
        // or cycles.0.to_u128()
        deposit_cycles(arg_deposit, u128::try_from(cycles.0).unwrap()).await.unwrap();
    }
}