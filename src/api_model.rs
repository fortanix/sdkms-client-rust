/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use chrono::{DateTime, Local, TimeZone, Utc};
use hyper;
use hyper::status::StatusCode;
#[cfg(feature = "native-tls")]
use hyper_native_tls::native_tls;
use rustc_serialize::base64::{FromBase64, ToBase64, STANDARD};
use serde::de::Error as DeserializeError;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::collections::{HashMap, HashSet};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use std::{error, fmt, io};
use uuid::Uuid;

pub use generated::*;

/// Arbitrary binary data that is serialized/deserialized to/from base 64 string.
#[derive(Default, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Blob(Vec<u8>);

impl From<Vec<u8>> for Blob {
    fn from(d: Vec<u8>) -> Self {
        Blob(d)
    }
}

impl From<String> for Blob {
    fn from(s: String) -> Self {
        Blob(s.into_bytes())
    }
}

impl<'a> From<&'a str> for Blob {
    fn from(s: &str) -> Self {
        Blob(s.as_bytes().to_owned())
    }
}

impl From<Blob> for Vec<u8> {
    fn from(d: Blob) -> Self {
        d.0
    }
}

impl Deref for Blob {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Blob {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Serialize for Blob {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0.to_base64(STANDARD))
    }
}

impl<'de> Deserialize<'de> for Blob {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;

        impl<'a> serde::de::Visitor<'a> for Visitor {
            type Value = Blob;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "base64-encoded string")
            }

            fn visit_str<E: de::Error>(self, string: &str) -> Result<Blob, E> {
                Ok(Blob(string.from_base64().map_err(|_| {
                    de::Error::invalid_value(de::Unexpected::Str(string), &"base64 encoded string")
                })?))
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

impl AsRef<[u8]> for Blob {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

pub type Name = String;
pub type Email = String;

// Data structure defitions for time wrapper structure - Store the number of seconds since EPOCH
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct Time(pub u64);
static ISO_8601_FORMAT: &'static str = "%Y%m%dT%H%M%SZ";

impl Serialize for Time {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_datetime().format(ISO_8601_FORMAT).to_string())
    }
}

impl<'de> Deserialize<'de> for Time {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s: String = Deserialize::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, ISO_8601_FORMAT)
            .map(|t| Time(t.timestamp() as u64))
            .map_err(|_| D::Error::invalid_value(
                serde::de::Unexpected::Str(&s),
                &"Date/time in ISO 8601 format",
            ))
    }
}

impl Time {
    pub fn now() -> Self {
        Self::from(Local::now())
    }
    pub fn to_datetime(&self) -> DateTime<Utc> {
        Utc.timestamp(self.0 as i64, 0)
    }
}

impl<Tz: TimeZone> From<DateTime<Tz>> for Time {
    fn from(t: DateTime<Tz>) -> Self {
        assert!(t.timestamp() >= 0);
        Time(t.timestamp() as u64)
    }
}

#[derive(Debug)]
pub enum Error {
    Unauthorized(String),
    Forbidden(String),
    BadRequest(String),
    Conflict(String),
    Locked(String),
    NotFound(String),
    StatusCode(String),
    EncoderError(serde_json::error::Error),
    IoError(io::Error),
    NetworkError(hyper::Error),
#[cfg(feature = "native-tls")]
    TlsError(native_tls::Error),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "sdkms-client error"
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NotFound(ref msg) => write!(fmt, "{}", msg),
            Error::Unauthorized(ref msg) => write!(fmt, "{}", msg),
            Error::Forbidden(ref msg) => write!(fmt, "{}", msg),
            Error::BadRequest(ref msg) => write!(fmt, "{}", msg),
            Error::Conflict(ref msg) => write!(fmt, "{}", msg),
            Error::Locked(ref msg) => write!(fmt, "{}", msg),
            Error::EncoderError(ref err) => write!(fmt, "{}", err),
            Error::IoError(ref err) => write!(fmt, "{}", err),
            Error::NetworkError(ref err) => write!(fmt, "{}", err),
            #[cfg(feature = "native-tls")]
            Error::TlsError(ref err) => write!(fmt, "{}", err),
            Error::StatusCode(ref msg) => write!(fmt, "unexpected status code: {}", msg),
        }
    }
}

impl Error {
    pub fn from_status(status: StatusCode, msg: String) -> Self {
        match status {
            StatusCode::Unauthorized => Error::Unauthorized(msg),
            StatusCode::Forbidden => Error::Forbidden(msg),
            StatusCode::BadRequest => Error::BadRequest(msg),
            StatusCode::Conflict => Error::Conflict(msg),
            StatusCode::Locked => Error::Locked(msg),
            StatusCode::NotFound => Error::NotFound(msg),
            _ => Error::StatusCode(format!("{}\n{}", status.to_string(), msg)),
        }
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(error: serde_json::error::Error) -> Error {
        Error::EncoderError(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::IoError(error)
    }
}

impl From<hyper::Error> for Error {
    fn from(error: hyper::Error) -> Error {
        Error::NetworkError(error)
    }
}

#[cfg(feature = "native-tls")]
impl From<native_tls::Error> for Error {
    fn from(error: native_tls::Error) -> Error {
        Error::TlsError(error)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BatchEncryptRequestItem {
    pub kid: Uuid,
    pub request: EncryptRequest,
}

pub type BatchEncryptRequest = Vec<BatchEncryptRequestItem>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BatchDecryptRequestItem {
    pub kid: Uuid,
    pub request: DecryptRequest,
}

pub type BatchDecryptRequest = Vec<BatchDecryptRequestItem>;

pub type BatchSignRequest = Vec<SignRequest>;

pub type BatchVerifyRequest = Vec<VerifyRequest>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum BatchResponseItem<T> {
    Success { status: u16, body: T },
    Error { status: u16, error: String },
}

impl<T> BatchResponseItem<T> {
    pub fn status(&self) -> u16 {
        match *self {
            BatchResponseItem::Success { status, .. } | BatchResponseItem::Error { status, .. } => {
                status
            }
        }
    }
}

pub type BatchResponse<T> = Vec<BatchResponseItem<T>>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthResponse {
    pub token_type: String,
    pub expires_in: u32,
    pub access_token: String,
    pub entity_id: Uuid,
    pub challenge: Option<MfaChallengeResponse>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ApprovableResult {
    pub status: u16,
    pub body: serde_json::Value,
}

impl ApprovableResult {
    pub fn is_ok(&self) -> bool {
        200 <= self.status && self.status < 300
    }
}

pub type PluginOutput = serde_json::Value;

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
pub enum Order {
    Ascending,
    Descending,
}

impl FromStr for Order {
    type Err = ();

    fn from_str(order: &str) -> Result<Self, ()> {
        match order {
            "asc" => Ok(Order::Ascending),
            "desc" => Ok(Order::Descending),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Order::Ascending => write!(f, "asc"),
            Order::Descending => write!(f, "desc"),
        }
    }
}

impl Default for Order {
    fn default() -> Self {
        Order::Ascending
    }
}

// AppGroups contains a list of groups and optionally permissions granted to an app in each group.
// In order to get information about the app permissions in each group, you should set
// `group_permissions` to true in GetAppParams/ListAppsParams when making app-related requests.
// When creating a new app, you should always specify desired permissions for each group.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppGroups(HashMap<Uuid, Option<AppPermissions>>);

impl Deref for AppGroups {
    type Target = HashMap<Uuid, Option<AppPermissions>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AppGroups {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<HashMap<Uuid, Option<AppPermissions>>> for AppGroups {
    fn from(d: HashMap<Uuid, Option<AppPermissions>>) -> Self {
        AppGroups(d)
    }
}

impl From<AppGroups> for HashMap<Uuid, Option<AppPermissions>> {
    fn from(d: AppGroups) -> Self {
        d.0
    }
}

impl Serialize for AppGroups {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0
            .iter()
            .map(|(id, perm)| (id, perm.unwrap_or(AppPermissions::empty())))
            .collect::<HashMap<_, _>>()
            .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for AppGroups {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum S {
            Modern(HashMap<Uuid, AppPermissions>),
            Legacy(HashSet<Uuid>),
        }

        Ok(AppGroups(match S::deserialize(deserializer)? {
            S::Modern(map) => map.into_iter().map(|(id, perm)| (id, Some(perm))).collect(),
            S::Legacy(set) => set.into_iter().map(|id| (id, None)).collect(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_groups_modern() {
        let id = Uuid::parse_str("34e03147-9f71-4be9-9a54-3feda0843393").unwrap();
        let mut a = HashMap::new();
        a.insert(id, Some(AppPermissions::ENCRYPT | AppPermissions::DECRYPT));
        let a = AppGroups::from(a);
        let json = r#"{"34e03147-9f71-4be9-9a54-3feda0843393":["ENCRYPT","DECRYPT"]}"#;
        assert_eq!(serde_json::to_string(&a).unwrap(), json);
        assert_eq!(a, serde_json::from_str(&json).unwrap());

        let mut a = HashMap::new();
        a.insert(id, Some(AppPermissions::empty()));
        let a = AppGroups::from(a);
        let json = r#"{"34e03147-9f71-4be9-9a54-3feda0843393":[]}"#;
        assert_eq!(serde_json::to_string(&a).unwrap(), json);
        assert_eq!(a, serde_json::from_str(&json).unwrap());

        let a = AppGroups::from(HashMap::new());
        let json = r#"{}"#;
        assert_eq!(serde_json::to_string(&a).unwrap(), json);
        assert_eq!(a, serde_json::from_str(&json).unwrap());
    }

    #[test]
    fn app_groups_legacy() {
        let id = Uuid::parse_str("34e03147-9f71-4be9-9a54-3feda0843393").unwrap();
        let mut a = HashMap::new();
        a.insert(id, None);
        let a = AppGroups::from(a);
        let json = r#"["34e03147-9f71-4be9-9a54-3feda0843393"]"#;
        assert_eq!(a, serde_json::from_str(&json).unwrap());

        let a = AppGroups::from(HashMap::new());
        let json = r#"[]"#;
        assert_eq!(a, serde_json::from_str(&json).unwrap());
    }
}
