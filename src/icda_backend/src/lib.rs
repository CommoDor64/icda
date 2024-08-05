use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::cell::RefCell;
use hex::encode;

type ObjectHash = String;

thread_local! {
    static STORAGE: RefCell<HashMap<ObjectHash, Vec<u8>>> = RefCell::new(HashMap::new());
}

#[derive(CandidType, Deserialize, Serialize, Clone)]
struct Object {
    data: Vec<u8>,
}

#[ic_cdk::init]
fn init() {}

#[ic_cdk::update]
fn store(obj: Object) -> ObjectHash {
    let serialized_obj = serde_cbor::to_vec(&obj).expect("Failed to serialize object");
    let mut hasher = Sha256::new();
    hasher.update(&serialized_obj);
    let hash = encode(hasher.finalize());
    
    STORAGE.with(|storage| {
        storage.borrow_mut().insert(hash.clone(), serialized_obj);
    });
    
    hash
}

#[ic_cdk::query]
fn fetch(hash: ObjectHash) -> Option<Object> {
    STORAGE.with(|storage| {
        storage.borrow().get(&hash).map(|bytes| {
            serde_cbor::from_slice(bytes).expect("Failed to deserialize object")
        })
    })
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

// Enable Candid export
ic_cdk::export_candid!();