use candid::{CandidType, Deserialize};
use ic_certified_map::{AsHashTree, RbTree};
use serde::Serialize;
use std::cell::RefCell;
use std::collections::HashMap;

type ObjectHash = String;

thread_local! {
    static STORAGE: RefCell<HashMap<Vec<u8>, Vec<u8>>> = RefCell::new(HashMap::new());
    static TREE: RefCell<RbTree<Vec<u8>, Vec<u8>>> = RefCell::new(RbTree::new());
}

#[derive(CandidType, Deserialize, Serialize, Clone)]
struct Object {
    data: Vec<u8>,
}

#[ic_cdk::init]
fn init() {}

#[derive(CandidType, Debug, Deserialize, Serialize, Clone)]
struct StorageReceipt {}

fn sanitize_hash(hash: &ObjectHash) -> String {
    hash.strip_prefix("0x").unwrap_or(&hash).to_string()
}

#[ic_cdk::update]
fn store(hash: ObjectHash, block: Object) -> Result<StorageReceipt, String> {
    let shash = sanitize_hash(&hash).as_bytes().to_vec();
    STORAGE.with(|storage| {
        let storage_receipt = TREE.with(|tree| {
            let mut tree = tree.borrow_mut();
            tree.insert(shash.to_owned(), shash.to_owned());
            let root_hash = tree.root_hash();
            ic_cdk::api::set_certified_data(&root_hash);

            StorageReceipt {}
        });
        storage.borrow_mut().insert(shash, block.data);

        ic_cdk::println!("successfully stored block with hash: {}", &hash);

        Ok(storage_receipt)
    })
}

#[derive(CandidType, Deserialize, Serialize, Clone)]
struct CertifiedBlock {
    certificate: Vec<u8>,
    witness: Vec<u8>,
    data: Vec<u8>,
}

#[ic_cdk::query]
fn fetch(hash: ObjectHash) -> Result<CertifiedBlock, String> {
    let shash = &sanitize_hash(&hash).as_bytes().to_vec();
    let certificate = ic_cdk::api::data_certificate()
        .ok_or_else(|| format!("No certificate for hash: {:?}", shash))?;

    let witness = TREE.with(|tree: &RefCell<RbTree<Vec<u8>, Vec<u8>>>| {
        let tree = tree.borrow();
        let mut witness = vec![];
        let mut witness_serializer = serde_cbor::Serializer::new(&mut witness);
        witness_serializer.self_describe().unwrap();
        tree.witness(shash)
            .serialize(&mut witness_serializer)
            .unwrap();
        witness
    });

    let data = STORAGE.with(|storage| {
        storage
            .borrow()
            .get(shash)
            .cloned()
            .ok_or_else(|| format!("No data for hash: {:?}", shash))
    })?;

    ic_cdk::println!("successfully fetched block with hash: {}", &hash);

    Ok(CertifiedBlock {
        certificate,
        witness,
        data,
    })
}

ic_cdk::export_candid!();
