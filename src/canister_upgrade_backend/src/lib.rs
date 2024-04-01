use candid::Principal; 
use ic_cdk::caller;
use ic_cdk::{
    api::call::ManualReply, init, post_upgrade, pre_upgrade, query, storage, update
}; 
use std::cell::RefCell; 
use std::collections::{BTreeMap, BTreeSet}; 

type Users = BTreeSet<Principal>; 
type Store = BTreeMap<String, Vec<u8>>; 

thread_local! {
    static USERS: RefCell<Users> = RefCell::default(); 
    static STORE: RefCell<Store> = RefCell::default(); 
}

// This init function adds the user who instantiated the canister to the USERS set.
#[init]
fn init() {
    USERS.with(|users| users.borrow_mut().insert(ic_cdk::api::caller())); 
}

fn is_user() -> Result<(), String> {
    if USERS.with(|users| users.borrow().contains(&ic_cdk::api::caller())) {
        Ok(())
    } else {
        Err("Store can only be set by the owner of the canister".to_string())
    }
}

// The guard = "is_user" part specifies a guard function, is_user, which is called before the store function
// The store function is an update call that allows a user to store a value (a Vec<u8>) at a given path (a String)
#[update(guard = "is_user")] 
fn store(path: String, contents: Vec<u8>) {
    STORE.with(|store| store.borrow_mut().insert(path, contents)); 
}

// The retrieve function is a query call that allows anyone to retrieve the value at a given path from the STORE.
// The manual_reply = true part specifies that the function will manually construct the reply to the query.
#[query(manual_reply = true )] 
fn retrieve(path: String) -> ManualReply<Vec<u8>> {
    STORE.with(|store| match store.borrow().get(&path) {
        Some(content) => ManualReply::one(content), 
        None => panic!("Path {} not found", path)
    })
}

// The add_user function is an update call that allows a user to add another Principal to the USERS set.
#[update(guard = "is_user")] 
fn add_user(principal: Principal) {
    USERS.with(|users| users.borrow_mut().insert(principal)); 
}

// This function is called before canister upgrade. The users are saved in the stable storage before a canister upgrade (Thus allowing preservation of the canisters state) 
#[pre_upgrade] 
fn pre_upgrade() {
    USERS.with(|users| storage::stable_save((users,)).unwrap()); 
}

// This function restores users from stable storage: allowing the state of the canister to be preserved accross upgrades
#[post_upgrade] 
fn post_upgrade() {
    let (old_users,): (BTreeSet<Principal>,) = storage::stable_restore().unwrap(); // This function is called to restore users from the stable storage 
    USERS.with(|users| *users.borrow_mut() = old_users); 
}

ic_cdk::export_candid!();