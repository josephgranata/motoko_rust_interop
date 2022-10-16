use ic_cdk::api::management_canister::main::*;
use ic_cdk::{ api };
use candid::{Nat};

#[ic_cdk_macros::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

// https://github.com/dfinity/cdk-rs/tree/main/examples/management_canister

pub const WASM: &[u8] = include_bytes!(
    "../../../target/wasm32-unknown-unknown/release/rust_demo_backend.wasm"
);

#[ic_cdk_macros::update]
async fn create_bucket(user_id: ic_cdk::export::Principal) -> ic_cdk::export::Principal {
    let caller = api::caller();

    let canister_id = create_canister_with_extra_cycles(
        CreateCanisterArgument {
            settings: Some(CanisterSettings {
                controllers: Some(vec![ic_cdk::id()]),
                compute_allocation: None,
                memory_allocation: None,
                freezing_threshold: None,
            }),
        },
        1_000_000_000_000u128
    ).await.unwrap().0.canister_id;

    let arg = UpdateSettingsArgument {
        canister_id,
        settings: CanisterSettings {
            controllers: Some(vec![canister_id, ic_cdk::id(), caller, user_id]),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
        },
    };

    update_settings(arg).await.unwrap();

    // Install code of this canister
    let arg = InstallCodeArgument {
        mode: CanisterInstallMode::Install,
        canister_id,
        wasm_module: WASM.into(),
        // wasm_module: b"\x00asm\x01\x00\x00\x00".to_vec(),
        arg: vec![],
    };
    install_code(arg).await.unwrap();

    return canister_id;
}

#[ic_cdk_macros::update]
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