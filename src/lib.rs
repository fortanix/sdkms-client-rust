/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! API bindings for [Fortanix SDKMS].
//!
//! The [`SdkmsClient`] type provides access to [REST APIs] exposed by SDKMS through method calls.
//! Input/output types are defined in [`api_model`].
//!
//! ## Example
//! Here is a quick example for how to use this crate. A more extensive set of examples can be found
//! [in the source repository](https://github.com/fortanix/sdkms-client-rust/blob/master/examples)
//!
//! ```no_run
//! use sdkms::api_model::*;
//! use sdkms::{Error, SdkmsClient};
//!
//! fn main() -> Result<(), Error> {
//!     let client = SdkmsClient::builder()
//!         .with_api_endpoint("https://sdkms.fortanix.com")
//!         .with_api_key("MDczMjNlNmUtYzliZC...") // replace with an actual API key!
//!         .build()?;
//!
//!     let encrypt_resp = client.encrypt(&EncryptRequest {
//!         plain: "hello, world!".into(),
//!         alg: Algorithm::Aes,
//!         key: Some(SobjectDescriptor::Name("my AES key".to_owned())),
//!         mode: Some(CryptMode::Symmetric(CipherMode::Cbc)),
//!         iv: None,
//!         ad: None,
//!         tag_len: None,
//!     })?;
//!
//!     let decrypt_resp = client.decrypt(&DecryptRequest {
//!         cipher: encrypt_resp.cipher,
//!         iv: encrypt_resp.iv,
//!         key: Some(SobjectDescriptor::Name("my AES key".to_owned())),
//!         mode: Some(CryptMode::Symmetric(CipherMode::Cbc)),
//!         alg: None,
//!         ad: None,
//!         tag: None,
//!     })?;
//!
//!     let plain = String::from_utf8(decrypt_resp.plain.into()).expect("valid utf8");
//!     println!("{}", plain);
//!     Ok(())
//! }
//! ```
//!
//! [`SdkmsClient`]: ./struct.SdkmsClient.html
//! [`api_model`]: ./api_model/index.html
//! [REST APIs]: https://www.fortanix.com/api/sdkms/
//! [Fortanix SDKMS]: https://fortanix.com/products/sdkms/

#[macro_use]
extern crate serde_derive;
extern crate rustc_serialize;
extern crate serde;

extern crate chrono;
extern crate serde_json;
extern crate uuid;

#[macro_use]
extern crate bitflags;

extern crate hyper;

#[cfg(feature = "hyper-native-tls")]
extern crate hyper_native_tls;

extern crate url;

#[macro_use]
extern crate log;

#[macro_use]
mod macros;
pub mod api_model;
mod client;
mod generated;
pub mod operations;

pub use api_model::Error;
pub use client::*;
