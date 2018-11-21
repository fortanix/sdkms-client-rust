extern crate env_logger;
extern crate rustc_serialize;
extern crate sdkms;

use rustc_serialize::base64::{ToBase64, STANDARD};
use sdkms::api_model::*;
use sdkms::{Error as SdkmsError, SdkmsClient};
use std::{thread, time};

const MY_API_KEY: &'static str = "ODZmODJhNTAtYmJjNy...";
const KEY_NAME: &'static str = "RSA Key 2";

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
        mode: Some(SignatureMode::Rsa(RsaSignaturePadding::Pkcs1V15 {})),
        hash: None,
        deterministic_signature: None,
    };
    let sign_resp = sign(&client, &sign_req)?;
    println!("Signature: {}", sign_resp.signature.to_base64(STANDARD));
    Ok(())
}

fn sign(client: &SdkmsClient, req: &SignRequest) -> Result<SignResponse, SdkmsError> {
    println!("trying direct call to Sign API first...");
    match client.sign(req) {
        Err(SdkmsError::Forbidden(ref msg)) if msg == "This operation requires approval" => {
            println!("trying approval request path...");
            sign_with_approval(client, req)
        }
        Err(err) => Err(err),
        Ok(resp) => Ok(resp),
    }
}

fn sign_with_approval(client: &SdkmsClient, req: &SignRequest) -> Result<SignResponse, SdkmsError> {
    let description = "Pretty please".to_owned();
    let pa = client.request_approval_to_sign(req, Some(description))?;
    while pa.status(client)? == ApprovalStatus::Pending {
        println!("Request is pending...");
        thread::sleep(time::Duration::from_secs(10));
    }
    pa.result(client)?
}
