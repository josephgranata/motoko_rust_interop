use ic_cdk::{ api };

use crate::{ STATE, types::storage::{ AssetKey, Batch, State, Chunk } };

// Upload batch and chunks

const BATCH_EXPIRY_NANOS: u64 = 300_000_000_000;

static mut NEXT_BACK_ID: u8 = 0;
static mut NEXT_CHUNK_ID: u8 = 0;

pub fn create_batch(key: AssetKey) -> u8 {
    STATE.with(|state| create_batch_impl(key, &mut state.borrow_mut()))
}

pub fn create_chunk(chunk: Chunk) -> Result<u8, &'static str> {
    STATE.with(|state| create_chunk_impl(chunk, &mut state.borrow_mut()))
}

fn create_batch_impl(key: AssetKey, state: &mut State) -> u8 {
    let now = api::time();

    unsafe {
        NEXT_BACK_ID = NEXT_BACK_ID + 1;

        state.batches.insert(NEXT_BACK_ID, Batch {
            key,
            expiresAt: now + BATCH_EXPIRY_NANOS,
        });

        NEXT_BACK_ID
    }
}

fn create_chunk_impl(Chunk {batchId, content}: Chunk, state: &mut State) -> Result<u8, &'static str> {
    let batch = state.batches.get(&batchId);

    match batch {
        None => Err("Batch not found."),
        Some(b) => {
            let now = api::time();

            state.batches.insert(
                batchId,
                Batch {
                    key: b.key.clone(),
                    expiresAt: now + BATCH_EXPIRY_NANOS
                }
            );

            unsafe {
                NEXT_CHUNK_ID = NEXT_CHUNK_ID + 1;

                state.chunks.insert(
                    NEXT_CHUNK_ID,
                    Chunk {
                        batchId,
                        content
                    }
                );

                Ok(NEXT_CHUNK_ID)
            }
        }
    }
}