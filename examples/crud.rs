extern crate chrono;
extern crate env_logger;
extern crate rand;
extern crate sdkms;

use chrono::Local;
use rand::prelude::*;
use sdkms::api_model::*;
use sdkms::{Error as SdkmsError, SdkmsClient};

const MY_API_KEY: &'static str = "MDczMjNlNmUtYzliZC...";

fn main() -> Result<(), SdkmsError> {
    env_logger::init();

    let client = SdkmsClient::builder()
        .with_api_endpoint("https://sdkms.fortanix.com")
        .with_api_key(MY_API_KEY)
        .build()?;

    let sobject_req = SobjectRequest {
        name: Some(format!("TestKey-{}", random_name(8))),
        obj_type: Some(ObjectType::Aes),
        key_ops: Some(
            KeyOperations::ENCRYPT | KeyOperations::DECRYPT | KeyOperations::APPMANAGEABLE,
        ),
        key_size: Some(256),
        ..Default::default()
    };
    let sobject = client.create_sobject(&sobject_req)?;
    println!("Created sobject: {}", sobject_to_string(&sobject));

    let query_params = ListSobjectsParams {
        sort: SobjectSort::ByName {
            order: Order::Ascending,
            start: None,
        },
        ..Default::default()
    };
    let keys = client.list_sobjects(&query_params)?;
    println!("\n\nListing all sobjects ({}):", keys.len());
    for key in keys {
        println!("{}", sobject_to_string(&key));
    }

    let kid = sobject.kid.as_ref().expect("not transient");
    client.delete_sobject(kid)?;
    println!("\n\nSobject {} deleted", kid);
    Ok(())
}

fn sobject_to_string(s: &Sobject) -> String {
    format!(
        "{{ {} {} group({}) enabled: {} created: {} }}",
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
