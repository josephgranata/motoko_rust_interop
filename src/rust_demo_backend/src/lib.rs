use ic_cdk::api::management_canister::main::{ canister_status, CanisterIdRecord, deposit_cycles };
use ic_cdk::{ api, storage };
use candid::{ Nat, Principal };
use ic_cdk_macros::{ init, query, update, pre_upgrade, post_upgrade };
use std::cell::RefCell;

// https://medium.com/encode-club/encode-x-internet-computer-intro-to-building-on-the-ic-in-rust-video-slides-b496d6baad08
// https://github.com/hpeebles/rust-canister-demo/blob/master/todo/src/lib.rs

thread_local! {
    static STATE: RefCell<State> = RefCell::default();
}

// TODO: https://forum.dfinity.org/t/init-arg-mandatory-in-state/16009/ ?
// I would rather like to have a mandatory { owner: Principal } without having to assign a default value.

#[derive(Default)]
struct State {
    owner: Option<Principal>,
}

#[init]
fn init(user: Principal) {
    ic_cdk::print(format!("Initializing bucket., {}", user.to_text()));
    STATE.with(|state| {
        *state.borrow_mut() = State {
            owner: Some(user),
        };
    });
}

#[pre_upgrade]
fn pre_upgrade() {
    STATE.with(|state| storage::stable_save((&state.borrow().owner,)).unwrap());
}

#[post_upgrade]
fn post_upgrade() {
    let (owner,): (Option<Principal>,) = storage::stable_restore().unwrap();
    let new_state: State = State { owner };
    STATE.with(|state| *state.borrow_mut() = new_state);
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

#[update]
async fn transfer_cycles() {
    // TODO: is caller === manager

    let _caller = api::caller();

    let arg = CanisterIdRecord { canister_id: ic_cdk::id() };

    let response = canister_status(arg).await.unwrap().0;

    let cycles: Nat = response.cycles - Nat::from(100_000_000_000u128);

    ic_cdk::print(format!("{}", cycles));

    // TODO: convert candid:Nat to u128
    // if cycles > 0 {
    deposit_cycles(arg, 500_000_000_000u128).await.unwrap();
    // }
}