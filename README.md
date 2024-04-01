# CANISTER UPGRADE 

This is a simple practice on how to do a canister upgrade in order to maintain the canister's state whenever you're doing upgrades 

These are the main functions: 
```
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
```

## Commands to run: 
Execute the following commands:

1. Start the local canister environment: 
```
dfx start --clean --background
``` 

2. Obtain the canister's ID: 
```
dfx canister id canister_upgrade_backend
```

3. Command for upgrading a canister: 
```
dfx canister install --all --mode upgrade 
```