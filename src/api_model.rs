/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use serde::de::Error as DeserializeError;
use serde::ser::Error as SerializeError;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use simple_hyper_client::StatusCode;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use std::time::SystemTime;
use std::{error, fmt, io};
use time::format_description::FormatItem;
use time::macros::format_description;
use time::{OffsetDateTime, PrimitiveDateTime};
#[cfg(feature = "native-tls")]
use tokio_native_tls::native_tls;
use uuid::Uuid;

pub use crate::generated::*;

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
        serializer.serialize_str(&base64::encode(&self.0))
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
                Ok(Blob(base64::decode(string).map_err(|_| {
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

/// `Time` stores the number of seconds since Unix epoch.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct Time(pub u64);

static ISO_8601_FORMAT: &[FormatItem<'_>] =
    format_description!("[year][month][day]T[hour][minute][second]Z");

impl Serialize for Time {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let utc = self
            .to_utc_datetime()
            .map_err(|e| S::Error::custom(e.to_string()))?;
        let s = utc
            .format(ISO_8601_FORMAT)
            .map_err(|e| S::Error::custom(e.to_string()))?;
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for Time {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s: String = Deserialize::deserialize(deserializer)?;
        let t = PrimitiveDateTime::parse(&s, ISO_8601_FORMAT).map_err(|e| {
            D::Error::custom(format!("expected date/time in ISO-8601 format: {}", e))
        })?;

        Time::try_from(t.assume_utc()).map_err(|e| D::Error::custom(e))
    }
}

impl Time {
    pub fn now() -> Self {
        let t = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        Self(t.as_secs())
    }

    pub fn to_utc_datetime(&self) -> Result<OffsetDateTime, TimeOutOfRange> {
        if self.0 > i64::MAX as u64 {
            return Err(TimeOutOfRange::TooLarge);
        }
        OffsetDateTime::from_unix_timestamp(self.0 as i64).map_err(|_| TimeOutOfRange::TooLarge)
    }
}

impl TryFrom<OffsetDateTime> for Time {
    type Error = TimeOutOfRange;

    fn try_from(t: OffsetDateTime) -> Result<Self, Self::Error> {
        if t.unix_timestamp() < 0 {
            return Err(TimeOutOfRange::BeforeUnixEpoch);
        }
        Ok(Time(t.unix_timestamp() as u64))
    }
}

#[derive(Debug)]
pub enum TimeOutOfRange {
    BeforeUnixEpoch,
    TooLarge,
}

impl fmt::Display for TimeOutOfRange {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use TimeOutOfRange::*;
        match *self {
            BeforeUnixEpoch => write!(fmt, "date/times before Unix epoch (Jan. 1, 1970 00:00:00 UTC) cannot be stored as `Time`"),
            TooLarge => write!(fmt, "`Time` value is out of range for `OffsetDateTime`"),
        }
    }
}

impl error::Error for TimeOutOfRange {}

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
    NetworkError(simple_hyper_client::Error),
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
            StatusCode::UNAUTHORIZED => Error::Unauthorized(msg),
            StatusCode::FORBIDDEN => Error::Forbidden(msg),
            StatusCode::BAD_REQUEST => Error::BadRequest(msg),
            StatusCode::CONFLICT => Error::Conflict(msg),
            StatusCode::LOCKED => Error::Locked(msg),
            StatusCode::NOT_FOUND => Error::NotFound(msg),
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

impl From<simple_hyper_client::Error> for Error {
    fn from(error: simple_hyper_client::Error) -> Error {
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

    #[test]
    fn time() {
        let t = Time::now();
        t.to_utc_datetime().expect("in bounds");
        serde_json::to_string(&t).expect("in bounds and correct format");

        let t: Time = serde_json::from_str(r#""20200315T012345Z""#).expect("valid date/time");
        assert_eq!(t.0, 1584235425);

        let t: Time = serde_json::from_str(r#""19700101T000000Z""#).expect("valid date/time");
        assert_eq!(t.0, 0);

        let err = serde_json::from_str::<Time>(r#""20220119T024257""#).unwrap_err();
        assert_eq!(
            err.to_string(),
            "expected date/time in ISO-8601 format: a character literal was not valid"
        );

        let err = serde_json::from_str::<Time>(r#""19670120T012345Z""#).unwrap_err();
        assert_eq!(
            err.to_string(),
            "date/times before Unix epoch (Jan. 1, 1970 00:00:00 UTC) cannot be stored as `Time`"
        );

        let err = Time(i64::MAX as u64 + 10).to_utc_datetime().unwrap_err();
        assert_eq!(
            err.to_string(),
            "`Time` value is out of range for `OffsetDateTime`"
        );

        let err = Time::try_from(OffsetDateTime::from_unix_timestamp(-1).unwrap()).unwrap_err();
        assert_eq!(
            err.to_string(),
            "date/times before Unix epoch (Jan. 1, 1970 00:00:00 UTC) cannot be stored as `Time`"
        );
    }
}
