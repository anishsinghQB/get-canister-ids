mod types;
use candid::Principal;
use ic_cdk::{api::{self, call::{call_with_payment128, CallResult}}, update, };
use types::{CanisterIdRecord, CanisterSettings, CreateCanisterArgument};
use ic_cdk::export_candid;

#[update]
async fn get_canister_id() -> Result<Principal, String> {
    prevent_anonymous()?;
    let controllers: Vec<Principal> = vec![api::caller(), ic_cdk::api::id()];

    let controller_settings = CanisterSettings {
        controllers: Some(controllers),
        ..Default::default()
    };

    let arg = CreateCanisterArgument {
        settings: Some(controller_settings),
    };
    
    match create_new_canister(arg).await {
        Ok((canister_id_record,)) => Ok(canister_id_record.canister_id),
        Err((_, err_string)) => Err(format!("Failed to create canister: {}", err_string)),
    }
}

pub async fn create_new_canister(
    arg: CreateCanisterArgument,
) -> CallResult<(CanisterIdRecord,)> {
    let cycles: u128 = 1_000_000_000_000;
    call_with_payment128(
        Principal::management_canister(),
        "create_canister",
        (arg,),
        cycles,
    )
    .await
}

pub fn prevent_anonymous() -> Result<(), String> {
    if api::caller() == Principal::anonymous() {
        return Err(String::from("unauthorized user"));
    }
    Ok(())
}


export_candid!();
