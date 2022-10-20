use ic_cdk::api::management_canister::main::{ canister_status, CanisterIdRecord, deposit_cycles };
use ic_cdk::{ api };
use candid::{ Nat, Principal };
use ic_cdk_macros::{ init, query, update };

#[init]
fn init(user: Principal) {
    ic_cdk::print(format!("Initializing bucket., {}", user.to_text()));
}

#[query]
fn greet(name: String) -> String {
    format!("Hello super super, {}!", name)
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