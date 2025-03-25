use ic_cdk::export::candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(CandidType, Serialize, Deserialize, Clone)]
struct NFT {
    owner: Principal,
    metadata: Metadata,
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
struct Metadata {
    title: String,
    creator: Principal,
    image_url: String,  // IPFS/Arweave CID
    royalties: u8,      // Percentage (e.g., 5 for 5%)
}

thread_local! {
    static NFTS: std::cell::RefCell<HashMap<u64, NFT>> = std::cell::RefCell::new(HashMap::new());
    static COUNTER: std::cell::RefCell<u64> = std::cell::RefCell::new(0);
}

#[ic_cdk_macros::update]
fn mint_nft(title: String, image_url: String, royalties: u8) -> u64 {
    let caller = ic_cdk::caller();
    let id = COUNTER.with(|counter| {
        let mut c = counter.borrow_mut();
        *c += 1;
        *c
    });
    let nft = NFT {
        owner: caller,
        metadata: Metadata {
            title,
            creator: caller,
            image_url,
            royalties,
        },
    };
    NFTS.with(|nfts| nfts.borrow_mut().insert(id, nft));
    id
}

#[ic_cdk_macros::query]
fn get_nft(id: u64) -> Option<NFT> {
    NFTS.with(|nfts| nfts.borrow().get(&id).cloned())
}