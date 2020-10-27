extern crate sdkms;
extern crate uuid;

use hyper::client::Pool;
use mbedtls::arc::rng::Rdseed;
use mbedtls::arc::ssl::Config;
use mbedtls::arc::ssl::config::{Endpoint, Preset, Transport, AuthMode, Version};
use mbedtls_hyper::MbedSSLNetworkConnector;
use sdkms::{Error as SdkmsError, SdkmsClient};
use std::sync::Arc;
use mbedtls::arc::rng::CtrDrbg;

const MY_USERNAME: &'static str = "...";
const MY_PASSWORD: &'static str = "...";


fn main() -> Result<(), SdkmsError> {
    let entropy = Arc::new(Rdseed);
    let rng = Arc::new(CtrDrbg::new(entropy, None).unwrap());

    let mut config = Config::new(Endpoint::Client, Transport::Stream, Preset::Default);
    
    config.set_authmode(AuthMode::None);
    config.set_rng(rng);
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
