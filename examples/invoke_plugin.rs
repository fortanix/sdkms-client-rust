extern crate sdkms;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

use sdkms::{Error as SdkmsError, SdkmsClient};
use std::{env, str::FromStr};
use uuid::Uuid;

pub const DEFAULT_API_ENDPOINT: &str = "https://sdkms.fortanix.com";

fn main() -> Result<(), SdkmsError> {
    let api_key = env::args().nth(1).expect("api key");
    let plugin_id = env::args().nth(2).expect("plugin id");

    let client = SdkmsClient::builder()
        .with_api_endpoint(DEFAULT_API_ENDPOINT)
        .with_api_key(&api_key)
        .build()?;

    let input = PluginInput { x: 10, y: 20 };
    let plugin_id = Uuid::from_str(&plugin_id).expect("valid uuid");
    let output: PluginOutput = client.invoke_plugin_nice(&plugin_id, &input)?;
    println!("{} + {} = {}", input.x, input.y, output.sum);
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct PluginInput {
    x: i32,
    y: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct PluginOutput {
    sum: i32,
}
