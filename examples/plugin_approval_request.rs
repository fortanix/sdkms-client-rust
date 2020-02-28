extern crate env_logger;
extern crate rustc_serialize;
extern crate sdkms;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

use rustc_serialize::base64::{ToBase64, STANDARD};
use sdkms::api_model::*;
use sdkms::{Error as SdkmsError, SdkmsClient};
use std::str::FromStr;
use std::{thread, time};
use uuid::Uuid;

const MY_API_KEY: &'static str = "ODZmODJhNTAtYmJjNy...";
const PLUGIN_ID: &'static str = "121b272e-d963-...";

fn main() -> Result<(), SdkmsError> {
    env_logger::init();

    let client = SdkmsClient::builder()
        .with_api_endpoint("https://sdkms.fortanix.com")
        .with_api_key(MY_API_KEY)
        .build()?;

    let input = PluginInput {
        data: "hello, world!".as_bytes().to_owned().into(),
        hash_alg: DigestAlgorithm::Sha256,
    };
    let input = serde_json::to_value(&input)?;
    let plugin_id = Uuid::from_str(PLUGIN_ID).expect("valid uuid");
    let pa = client.request_approval_to_invoke_plugin(&plugin_id, &input, None)?;
    while pa.status(&client)? == ApprovalStatus::Pending {
        println!("Request is pending...");
        thread::sleep(time::Duration::from_secs(10));
    }
    let output = pa.result(&client)??;
    let output: SignResponse = serde_json::from_value(output)?;
    println!("Signature: {}", output.signature.to_base64(STANDARD));
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct PluginInput {
    data: Blob,
    hash_alg: DigestAlgorithm,
}
