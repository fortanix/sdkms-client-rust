extern crate sdkms;
extern crate uuid;

use hyper::Client as HyperClient;
use hyper::client::Pool;
use hyper::net::HttpsConnector;
use mbedtls::rng::Rdseed;
use mbedtls::ssl::mbed::MbedSSLConfig;
use mbedtls::ssl::mbed::{Endpoint, Preset, Transport, AuthMode, Version};
use sdkms::api_model::*;
use sdkms::{Error as SdkmsError, SdkmsClient};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;
use mbedtls_hyper::MbedSSLNetworkConnector;

const MY_USERNAME: &'static str = "...";
const MY_PASSWORD: &'static str = "...";


fn main() -> Result<(), SdkmsError> {
    let mut entropy = Rdseed;
    let mut rng = mbedtls::rng::CtrDrbg::new(&mut entropy, None).unwrap();

    let mut config = MbedSSLConfig::new(Endpoint::Client, Transport::Stream, Preset::Default);
    
    config.set_authmode(AuthMode::None);
    config.set_rng(Some(&mut rng));
    config.set_min_version(Version::Tls1_2).unwrap();
    
    // Immutable from this point on
    let rc_config = Arc::new(config);
    let connector = MbedSSLNetworkConnector::new(rc_config, None);
    let client = hyper::Client::with_connector(Pool::with_connector(Default::default(), connector));
    let ssl_client = Arc::new(client);
    
    let mut client = SdkmsClient::builder()
        .with_api_endpoint("https://apps.sdkms.test.fortanix.com")
        .with_hyper_client(ssl_client)
        .build()?
        .authenticate_user(MY_USERNAME, MY_PASSWORD)?;

    let result = client.list_accounts(None).unwrap();
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
    
    client.terminate()?;
    Ok(())
}
