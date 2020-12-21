extern crate sdkms;

use sdkms::{api_model::*, Error as SdkmsError, SdkmsClient};
use std::env;

pub const DEFAULT_API_ENDPOINT: &str = "https://sdkms.fortanix.com";

fn main() -> Result<(), SdkmsError> {
    let api_key = env::args().nth(1).expect("api key required");
    let key_name = env::args().nth(2).expect("sdkms key-name required");

    let client = SdkmsClient::builder()
        .with_api_endpoint(DEFAULT_API_ENDPOINT)
        .with_api_key(&api_key)
        .build()?;

    let encrypt_req = EncryptRequest {
        plain: "hello, world!".as_bytes().to_owned().into(),
        alg: Algorithm::Aes,
        key: Some(SobjectDescriptor::Name(key_name.clone())),
        mode: Some(CryptMode::Symmetric(CipherMode::Cbc)),
        iv: None,
        ad: None,
        tag_len: None,
    };
    let encrypt_resp = client.encrypt(&encrypt_req)?;

    let decrypt_req = DecryptRequest {
        cipher: encrypt_resp.cipher,
        iv: encrypt_resp.iv,
        key: Some(SobjectDescriptor::Name(key_name)),
        mode: Some(CryptMode::Symmetric(CipherMode::Cbc)),
        alg: None,
        ad: None,
        tag: None,
    };
    let decrypt_resp = client.decrypt(&decrypt_req)?;
    let plain = String::from_utf8(decrypt_resp.plain.into()).expect("valid utf8");

    println!("{}", plain);
    Ok(())
}
