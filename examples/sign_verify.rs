use sdkms::api_model::*;
use sdkms::{Error as SdkmsError, SdkmsClient};

const MY_API_KEY: &'static str = "MDczMjNlNmUtYzliZC...";
const KEY_NAME: &'static str = "RSA Key 1";

fn main() -> Result<(), SdkmsError> {
    env_logger::init();

    let client = SdkmsClient::builder()
        .with_api_endpoint("https://sdkms.fortanix.com")
        .with_api_key(MY_API_KEY)
        .build()?;

    let sign_req = SignRequest {
        data: Some("hello, world!".as_bytes().to_owned().into()),
        hash_alg: DigestAlgorithm::Sha256,
        key: Some(SobjectDescriptor::Name(KEY_NAME.to_owned())),
        mode: Some(SignatureMode::rsa_pss(DigestAlgorithm::Sha1)),
        hash: None,
        deterministic_signature: None,
    };
    let sign_resp = client.sign(&sign_req)?;

    let verify_req = VerifyRequest {
        signature: sign_resp.signature,
        key: Some(SobjectDescriptor::Name(KEY_NAME.to_owned())),
        hash_alg: DigestAlgorithm::Sha256,
        data: Some("hello, world!".as_bytes().to_owned().into()),
        mode: Some(SignatureMode::rsa_pss(DigestAlgorithm::Sha1)),
        hash: None,
    };
    let verify_resp = client.verify(&verify_req)?;
    println!("Verify result: {}", verify_resp.result);
    Ok(())
}
