// Non snake case for backwards compatibility
#![allow(non_snake_case)]

pub mod storage {
    use std::collections::HashMap;
    use candid::{ Principal, CandidType };
    use serde::Deserialize;
    use std::clone::Clone;

    // Internal types

    pub type Batches = HashMap<u8, Batch>;
    pub type Chunks = HashMap<u8, Chunk>;
    pub type Assets = HashMap<String, Asset>;

    #[derive(Default, CandidType, Deserialize, Clone)]
    pub struct State {
        pub user: Option<Principal>,
        pub batches: Batches,
        pub chunks: Chunks,
        pub assets: Assets,
    }

    // Exposed types

    #[derive(CandidType, Deserialize, Clone)]
    pub struct Chunk {
        pub batchId: u8,
        pub content: Vec<u8>,
    }

    #[derive(CandidType, Deserialize, Clone)]
    pub struct AssetEncoding {
        pub modified: u32,
        pub contentChunks: Vec<Vec<u8>>,
        pub totalLength: u8,
    }

    #[derive(CandidType, Deserialize, Clone)]
    pub struct AssetKey {
        // myimage.jpg
        pub name: String,
        // images
        pub folder: String,
        // /images/myimage.jpg
        pub fullPath: String,
        // ?token=1223-3345-5564-3333
        pub token: Option<String>,
        // The sha256 representation of the content
        pub sha256: Option<Vec<u8>>,
    }

    #[derive(CandidType, Deserialize, Clone)]
    pub struct Asset {
        pub key: AssetKey,
        pub headers: Vec<(String, String)>,
        pub encoding: AssetEncoding,
    }

    #[derive(CandidType, Deserialize, Clone)]
    pub struct Batch {
        pub key: AssetKey,
        pub expiresAt: u64,
    }
}

pub mod http {
    use candid::{ CandidType };

    #[derive(CandidType)]
    pub struct HeaderField (String, String);
}