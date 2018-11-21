extern crate env_logger;
extern crate sdkms;
extern crate uuid;

use sdkms::api_model::*;
use sdkms::{Error as SdkmsError, SdkmsClient};
use std::str::FromStr;
use uuid::Uuid;

const MY_USERNAME: &'static str = "name@example.com";
const MY_PASSWORD: &'static str = "password";
const MY_ACCT_ID: &'static str = "b6080ec0-df2e-...";

fn main() -> Result<(), SdkmsError> {
    env_logger::init();

    let mut client = SdkmsClient::builder()
        .with_api_endpoint("https://sdkms.fortanix.com")
        .build()?
        .authenticate_user(MY_USERNAME, MY_PASSWORD)?;

    let acct_id = Uuid::from_str(MY_ACCT_ID).expect("valid uuid");
    client.select_account(&SelectAccountRequest { acct_id })?;
    let user_id = client.entity_id().unwrap();
    let user = client.get_user(&user_id)?;
    println!("User: {:#?}", user);

    client.terminate()?;
    Ok(())
}
