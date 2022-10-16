use ic_cdk::api::management_canister::main::*;
use ic_cdk::{ api };

#[ic_cdk_macros::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

// https://github.com/dfinity/cdk-rs/tree/main/examples/management_canister

#[ic_cdk_macros::update]
async fn create_bucket(user_id: ic_cdk::export::Principal) -> ic_cdk::export::Principal {
    let caller = api::caller();

    let arg = CreateCanisterArgument {
        settings: Some(CanisterSettings {
            controllers: Some(vec![ic_cdk::id()]),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
        }),
    };

    let canister_id = create_canister(arg).await.unwrap().0.canister_id;

    let arg = UpdateSettingsArgument {
        canister_id,
        settings: CanisterSettings {
            controllers: Some(vec![ic_cdk::id(), caller, user_id]),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
        },
    };    

    update_settings(arg).await.unwrap();

    return canister_id;
}