use ic_cdk::{ api };

use crate::{ STATE, types::{ AssetKey, Batch, State } };

// Upload batch and chunks

const BATCH_EXPIRY_NANOS: u64 = 300_000_000_000;

static mut next_back_id: u8 = 0;

pub fn create_batch(key: AssetKey) -> u8 {
    STATE.with(|state| create_batch_impl(key, &mut state.borrow_mut()))
}

fn create_batch_impl(key: AssetKey, state: &mut State) -> u8 {
    let now = api::time();

    unsafe {
        next_back_id = next_back_id + 1;

        state.batches.insert(next_back_id, Batch {
            key,
            expiresAt: now + BATCH_EXPIRY_NANOS,
        });

        next_back_id
    }
}
