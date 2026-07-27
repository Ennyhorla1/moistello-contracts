use soroban_sdk::{contractevent, contracterror, symbol_short, Address, Env};

#[contracterror]
#[derive(Debug)]
pub enum UpgradeError {
    NotAuthorized = 1,
}

/// Emitted when the contract implementation is upgraded.
#[contractevent]
#[derive(Clone, Debug)]
pub struct Upgraded {
    pub by: Address,
    pub new_impl: Address,
}

pub fn get_implementation(env: &Env) -> Option<Address> {
    env.storage().instance().get(&symbol_short!("impl"))
}

pub fn set_implementation(
    env: &Env,
    admin: &Address,
    new_impl: &Address,
) -> Result<(), UpgradeError> {
    admin.require_auth();
    env.storage()
        .instance()
        .set(&symbol_short!("impl"), new_impl);
    Upgraded {
        by: admin.clone(),
        new_impl: new_impl.clone(),
    }
    .publish(env);
    Ok(())
}
