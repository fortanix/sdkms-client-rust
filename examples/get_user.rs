extern crate sdkms;
extern crate uuid;

use sdkms::{api_model::*, Error as SdkmsError, SdkmsClient};
use std::{env, str::FromStr};
use uuid::Uuid;

pub const DEFAULT_API_ENDPOINT: &str = "https://sdkms.fortanix.com";

fn main() -> Result<(), SdkmsError> {
    let username = env::args().nth(1).expect("username");
    let password = env::args().nth(2).expect("password");
    let acct_id = env::args().nth(3).expect("account name");

    let mut client = SdkmsClient::builder()
        .with_api_endpoint(DEFAULT_API_ENDPOINT)
        .build()?
        .authenticate_user(&username, &password)?;

    let acct_id = Uuid::from_str(&acct_id).expect("valid uuid");
    client.select_account(&SelectAccountRequest { acct_id })?;
    let user_id = client.entity_id().unwrap();
    let user = client.get_user(&user_id)?;
    println!("User: {:?}", user);

    client.terminate()?;
    Ok(())
}
