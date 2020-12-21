extern crate chrono;
extern crate rand;
extern crate sdkms;

use chrono::Local;
use rand::prelude::*;
use sdkms::{api_model::*, Error as SdkmsError, SdkmsClient};
use std::env;

pub const DEFAULT_API_ENDPOINT: &str = "https://sdkms.fortanix.com";

fn main() -> Result<(), SdkmsError> {
    let api_key = env::args().nth(1).expect("api key");

    let client = SdkmsClient::builder()
        .with_api_endpoint(DEFAULT_API_ENDPOINT)
        .with_api_key(&api_key)
        .build()?;

    // create key
    let key_name = format!("TestKey-{}", random_name(8));
    let sobject_req = SobjectRequest {
        name: Some(key_name.clone()),
        obj_type: Some(ObjectType::Aes),
        key_ops: Some(KeyOperations::ENCRYPT | KeyOperations::DECRYPT | KeyOperations::APPMANAGEABLE),
        key_size: Some(256),
        ..Default::default()
    };
    let sobject = client.create_sobject(&sobject_req)?;
    println!("Created sobject: {}", sobject_to_string(&sobject));

    // fetch all keys
    let query_params = ListSobjectsParams {
        sort: SobjectSort::ByName {
            order: Order::Ascending,
            start: None,
        },
        ..Default::default()
    };
    let keys = client.list_sobjects(Some(&query_params))?;
    println!("\n\nListing all sobjects ({}):", keys.len());
    for key in keys {
        println!("{}", sobject_to_string(&key));
    }

    // update key
    let kid = sobject.kid.as_ref().expect("not transient");
    let sobject_req = SobjectRequest {
        enabled: Some(false),
        ..Default::default()
    };
    let key = client.update_sobject(kid, &sobject_req)?;

    // fetch updated specific key
    let query_params = ListSobjectsParams {
        name: key.name,
        sort: SobjectSort::ByName {
            order: Order::Ascending,
            start: None,
        },
        ..Default::default()
    };
    let keys = client.list_sobjects(Some(&query_params))?;
    println!("\n\nListing updated key");
    for key in keys {
        println!("{}", sobject_to_string(&key));
    }

    // delete key
    client.delete_sobject(kid)?;
    println!("\n\nSobject {} deleted", kid);
    Ok(())
}

fn sobject_to_string(s: &Sobject) -> String {
    format!(
        "{{\n  kid = {}\n  name = {}\n  group = {}\n  enabled = {}\n  created = {} \n}}",
        s.kid.map_or("?".to_owned(), |kid| kid.to_string()),
        s.name.as_ref().map(String::as_str).unwrap_or_default(),
        s.group_id.map_or("?".to_owned(), |kid| kid.to_string()),
        s.enabled,
        s.created_at.to_datetime().with_timezone(&Local),
    )
}

fn random_name(size: usize) -> String {
    let char_set = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
    let mut s = String::with_capacity(size);
    let mut rng = thread_rng();
    for _ in 0..size {
        let r = rng.gen_range(0, char_set.len());
        s.push_str(&char_set[r..r + 1]);
    }
    s
}
