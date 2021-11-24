use sdkms::api_model::*;
use sdkms::{Error as SdkmsError, SdkmsClient};

const MY_API_KEY: &'static str = "MDczMjNlNmUtYzliZC...";
const KEY_NAME: &'static str = "RSA Key 1";

fn main() -> Result<(), SdkmsError> {
    env_logger::init();

    let mut client = SdkmsClient::builder()
        .with_api_endpoint("https://sdkms.fortanix.com")
        .build()?
        .authenticate_with_api_key(MY_API_KEY)?;

    let encrypt_req = EncryptRequest {
        plain: "hello, world!".as_bytes().to_owned().into(),
        alg: Algorithm::Rsa,
        key: Some(SobjectDescriptor::Name(KEY_NAME.to_owned())),
        mode: Some(CryptMode::rsa_oaep(DigestAlgorithm::Sha1)),
        iv: None,
        ad: None,
        tag_len: None,
    };
    let encrypt_resp = client.encrypt(&encrypt_req)?;

    let decrypt_req = DecryptRequest {
        cipher: encrypt_resp.cipher,
        iv: encrypt_resp.iv,
        key: Some(SobjectDescriptor::Name(KEY_NAME.to_owned())),
        mode: Some(CryptMode::rsa_oaep(DigestAlgorithm::Sha1)),
        alg: None,
        ad: None,
        tag: None,
    };
    let decrypt_resp = client.decrypt(&decrypt_req)?;
    let plain = String::from_utf8(decrypt_resp.plain.into()).expect("valid utf8");
    println!("{}", plain);

    client.terminate()?;
    Ok(())
}
