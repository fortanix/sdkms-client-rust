/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use b64_ct::{Config, FromBase64, ToBase64, URL_SAFE};
use http_v1::Method;
use serde::{
    de, de::Error as DeserializeError, de::Visitor, de::Unexpected, Deserialize, Deserializer, Serialize,
    Serializer,
};
use serde::{ser, ser::Error as SerializeError, ser::SerializeSeq, ser::SerializeStruct};
use simple_hyper_client::StatusCode;
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::fmt::Debug;
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

impl ToString for Blob {
    fn to_string(&self) -> String {
        base64::encode(&self.0)
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

#[derive(Debug, Default, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct ZeroizedString(String);

impl Clone for ZeroizedString {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }

    #[inline(always)]
    fn clone_from(&mut self, source: &Self) {
        self.0.clone_from(&source.0);
    }
}

impl Deref for ZeroizedString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ZeroizedString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<[u8]> for ZeroizedString {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsRef<str> for ZeroizedString {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl AsMut<str> for ZeroizedString {
    fn as_mut(&mut self) -> &mut str {
        self.0.as_mut()
    }
}

impl Borrow<str> for ZeroizedString {
    fn borrow(&self) -> &str {
        self.0.borrow()
    }
}

impl From<String> for ZeroizedString {
    fn from(string: String) -> Self {
        ZeroizedString(string)
    }
}

impl<'a> From<&'a str> for ZeroizedString {
    fn from(s: &'a str) -> Self {
        ZeroizedString(s.into())
    }
}

impl fmt::Display for ZeroizedString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ZeroizedBlob(pub(crate) Vec<u8>);

impl Default for ZeroizedBlob {
    fn default() -> Self {
        ZeroizedBlob(Default::default())
    }
}

impl Clone for ZeroizedBlob {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }

    #[inline(always)]
    fn clone_from(&mut self, source: &Self) {
        self.0.clone_from(&source.0);
    }
}

impl Deref for ZeroizedBlob {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ZeroizedBlob {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<[u8]> for ZeroizedBlob {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for ZeroizedBlob {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl From<Vec<u8>> for ZeroizedBlob {
    fn from(vec: Vec<u8>) -> Self {
        ZeroizedBlob(vec)
    }
}

impl From<ZeroizedBlob> for Vec<u8> {
    fn from(d: ZeroizedBlob) -> Self {
        d.0
    }
}

impl From<Blob> for ZeroizedBlob {
    fn from(blob: Blob) -> Self {
        ZeroizedBlob(blob.0)
    }
}

fn serialize_blob<S: Serializer>(
    serializer: S,
    bytes: &[u8],
) -> ::std::result::Result<S::Ok, S::Error> {
    serializer.serialize_bytes(bytes)
}

impl Serialize for ZeroizedBlob {
    fn serialize<S: Serializer>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error> {
        serialize_blob(serializer, &self.0)
    }
}

// NOTE: should deserialize just like Blob
impl<'de> Deserialize<'de> for ZeroizedBlob {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> ::std::result::Result<Self, D::Error> {
        // Note on reallocations: rustc_serialize base64 decoding allocates the Vec exactly once
        // (see <https://docs.rs/rustc-serialize/0.3.25/src/rustc_serialize/base64.rs.html#276>),
        // therefore we are safe w.r.t reallocations during deserialization. We should verify that
        // the same property holds once we migrate to a different base64 implementation.
        //
        // For more context on the issue with reallocations see:
        // <https://docs.rs/zeroize/latest/zeroize/#stackheap-zeroing-notes>
        Ok(Blob::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UrlSafe;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Base64<B64: Base64VariantMarker, V: Base64Able = Vec<u8>>(
    V,
    std::marker::PhantomData<B64>,
);
pub trait Base64VariantMarker {
    fn variant() -> Config;
}

impl Base64VariantMarker for UrlSafe {
    fn variant() -> Config {
        URL_SAFE
    }
}

pub trait Base64Able: Deref<Target = [u8]> + From<Vec<u8>> {
    /// The preferred string-like type to use for base64 strings.
    type StringLike: Deref<Target = str> + From<String>;
}

impl Base64Able for Vec<u8> {
    type StringLike = String;
}

impl Base64Able for ZeroizedBlob {
    type StringLike = ZeroizedString;
}

impl<B64: Base64VariantMarker, V: Base64Able> std::ops::Deref for Base64<B64, V> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.0
    }
}

impl<B64: Base64VariantMarker, V: Base64Able> From<Vec<u8>> for Base64<B64, V> {
    fn from(data: Vec<u8>) -> Self {
        Base64(data.into(), std::marker::PhantomData)
    }
}

impl<B64: Base64VariantMarker> From<ZeroizedBlob> for Base64<B64, ZeroizedBlob> {
    fn from(data: ZeroizedBlob) -> Self {
        Base64(data, std::marker::PhantomData)
    }
}

impl ToString for Base64<UrlSafe> {
    fn to_string(&self) -> String {
        self.0.to_base64(URL_SAFE)
    }
}

impl<B64: Base64VariantMarker, V: Base64Able> std::str::FromStr for Base64<B64, V> {
    type Err = b64_ct::FromBase64Error;

    fn from_str(string: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(string.from_base64()?.into())
    }
}

impl<B64: Base64VariantMarker, V: Base64Able> Serialize for Base64<B64, V> {
    fn serialize<S: Serializer>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error> {
        let string: V::StringLike = self.0.to_base64(B64::variant()).into();
        string.serialize(serializer)
    }
}

impl<'de, B64: Base64VariantMarker, V: Base64Able> Deserialize<'de> for Base64<B64, V> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> ::std::result::Result<Self, D::Error> {
        let string: V::StringLike = String::deserialize(deserializer)?.into();
        Ok(Self::from_str(&string).map_err(|_| {
            serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(&string),
                &"base64 encoded string",
            )
        })?)
    }
}

#[derive(Clone, Debug)]
pub enum Removable<T: Described> {
    Keep(T),
    Remove,
}

pub trait Described {
    fn type_name() -> &'static str;
}

impl<T: Described> Described for &T {
    fn type_name() -> &'static str {
        T::type_name()
    }
}

mod removable_serde_impl {
    use super::*;
    use serde_value::Value;

    fn generate_deserialization_err_msg(
        original_json: impl fmt::Display,
        t_name: impl fmt::Display,
        serde_err: impl fmt::Display,
    ) -> String {
        format!(
            "unknown value {original_json}. Expected either the string `\"remove\"`, or a \
            value of type {t_name}. Error from trying to deserialize into {t_name}: {serde_err}"
        )
    }

    impl<'de, T: Described + for<'a> Deserialize<'a>> Deserialize<'de> for Removable<T> {
        // NOTE: if you change this, don't forget to update the removable_json
        // test!
        fn deserialize<D: Deserializer<'de>>(
            deserializer: D,
        ) -> ::std::result::Result<Self, D::Error> {
            let value = Value::deserialize(deserializer)?;
            // Hacky way to retrieve original JSON passed in. Note that this should
            // not fail
            let Ok(value_str) = serde_json::to_string(&value) else {
                return Err(D::Error::custom("failed deserialization of Removable type"));
            };
            if let Value::String(ref v) = value {
                if v == "remove" {
                    return Ok(Removable::Remove);
                }
            }
            match value.deserialize_into() {
                Ok(val) => Ok(Removable::Keep(val)),
                Err(e) => Err(D::Error::custom(generate_deserialization_err_msg(
                    value_str,
                    T::type_name(),
                    e,
                ))),
            }
        }
    }

    impl<T: Described + Serialize + Clone> Serialize for Removable<T> {
        fn serialize<S: Serializer>(
            &self,
            serializer: S,
        ) -> ::std::result::Result<S::Ok, S::Error> {
            match self {
                Removable::Remove => "remove".serialize(serializer),
                Removable::Keep(t) => t.serialize(serializer),
            }
        }
    }
}

impl<T: Described> From<Option<T>> for Removable<T> {
    fn from(val: Option<T>) -> Self {
        match val {
            Some(val) => Removable::Keep(val),
            None => Removable::Remove,
        }
    }
}

pub trait RemovableExt {
    type T;
    fn into_option(self) -> Option<Self::T>;
}

impl<T: Described> RemovableExt for Removable<T> {
    type T = T;
    fn into_option(self) -> Option<T> {
        match self {
            Removable::Keep(val) => Some(val),
            Removable::Remove => None,
        }
    }
}

impl<T: Described> RemovableExt for Option<Removable<T>> {
    type T = T;
    fn into_option(self) -> Option<T> {
        match self {
            None => None,
            Some(Removable::Remove) => None,
            Some(Removable::Keep(val)) => Some(val),
        }
    }
}

impl<'a, T: Described> RemovableExt for Option<&'a Removable<T>> {
    type T = &'a T;
    fn into_option(self) -> Option<&'a T> {
        match self {
            None => None,
            Some(Removable::Remove) => None,
            Some(Removable::Keep(val)) => Some(val),
        }
    }
}

impl<T: Eq + Described> Eq for Removable<T> {}

impl<T: PartialEq + Described> PartialEq for Removable<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Removable::Keep(x), Removable::Keep(y)) => x == y,
            (Removable::Remove, Removable::Remove) => true,
            _ => false,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct PluginVersion {
    pub major: u32,
    pub minor: u32,
}

impl Serialize for PluginVersion {
    fn serialize<S: Serializer>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error> {
        let version = format!("{}.{}", self.major, self.minor);
        serializer.serialize_str(&version)
    }
}

impl<'de> Deserialize<'de> for PluginVersion {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> ::std::result::Result<Self, D::Error> {
        let version: String = Deserialize::deserialize(deserializer)?;
        let mut components = version.split(".");
        let major = components
            .next()
            .ok_or_else(|| D::Error::custom("no major version found"))?
            .parse::<u32>()
            .map_err(D::Error::custom)?;
        let minor = components
            .next()
            .ok_or_else(|| D::Error::custom("no minor version found"))?
            .parse::<u32>()
            .map_err(D::Error::custom)?;
        Ok(PluginVersion { major, minor })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
/// Contains the attributes that are returned to the caller when a new
/// credential is created, or a new assertion is requested.
pub struct PublicKeyCredential<T>
where
    T: Debug + Clone + Eq + PartialEq,
{
    /// Credential's identifier.
    pub id: Base64<UrlSafe>,

    /// Type of credential.
    pub r#type: PublicKeyCredentialType,

    /// Can be [`AuthenticatorAttestationResponse`] or [`AuthenticatorAssertionResponse`].
    pub response: T,

    /// This field contains client extension output entries produced
    /// by the extension’s client extension processing.
    pub get_client_extension_results: Option<AuthenticationExtensionsClientOutputs>,
}

// #[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
// #[serde(rename_all = "camelCase")]
// /// <https://www.w3.org/TR/webauthn-2/#iface-authenticatorassertionresponse>
// pub struct AuthenticatorAssertionResponse {
//     // We need to preserve it as JSON and not deserialize it
//     // to [CollectedClientData] because u2f signature contains
//     // it in hashed form and we need to verify that.
//     /// Base64url of client_data in JSON format.
//     #[serde(rename = "clientDataJSON")]
//     pub client_data_json: Base64<UrlSafe>,

//     /// Data returned by authenticator.
//     /// <https://www.w3.org/TR/webauthn-2/#sctn-authenticator-data>
//     pub authenticator_data: Base64<UrlSafe>,

//     /// Raw signature returned by authenticator.
//     /// <https://www.w3.org/TR/webauthn-2/#sctn-op-get-assertion>
//     pub signature: Base64<UrlSafe>,

//     /// Corresponds to [PublicKeyCredentialUserEntity::id] sent during
//     /// credential creation.
//     pub user_handle: Option<Base64<UrlSafe>>,
// }

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
/// This represents the authenticator's response to a client’s request
/// for the creation of a new public key credential.
///
/// It contains
/// information about the new credential that can be used to identify
/// it for later use, and metadata that can be used by the WebAuthn
/// Relying Party to assess the characteristics of the credential during
/// registration.
///
/// <https://www.w3.org/TR/webauthn-2/#iface-authenticatorattestationresponse>
pub struct AuthenticatorAttestationResponse {
    // We need to preserve it as JSON and not deserialize it
    // to [crate::fido2::models::CollectedClientData] because u2f signature contains
    // it in hashed form and we need to verify that.
    /// Base64url of [crate::fido2::models::CollectedClientData] in JSON form.
    #[serde(rename = "clientDataJSON")]
    pub client_data_json: Base64<UrlSafe>,

    /// Values obtained from `AuthenticatorAttestationResponse.getTransports()`.
    /// Webauthn spec recommends RP to store it and user them along with
    /// `allowCredentials` while authentication ceremony.
    pub get_transports: Option<Vec<AuthenticatorTransport>>,

    /// Base64url of the attestation object.
    ///
    /// See in order:
    /// <https://www.w3.org/TR/webauthn-2/#dom-authenticatorattestationresponse-attestationobject>
    /// <https://www.w3.org/TR/webauthn-2/#sctn-attestation>
    /// <https://www.w3.org/TR/webauthn-2/#sctn-defined-attestation-formats>
    ///
    /// Currently, only U2F is supported, others will be rejected.
    pub attestation_object: Base64<UrlSafe, ZeroizedBlob>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
/// <https://www.w3.org/TR/webauthn-2/#dictionary-pkcredentialentity>
pub struct PublicKeyCredentialEntity<T>
where
    T: Debug + Clone + Eq + PartialEq,
{
    // Depending on T, which can be a User
    // or a relying party, webauthn recommends
    // certain naming ways.
    //   Relying Party:
    //     spec recommends nickname RFC checks for this - RFC 8266
    //   User:
    //     spec recommends username RFC checks for this - RFC 8265
    //
    // We don't follow these.
    /// Name of the entity.
    // For user it is their email
    pub name: String,

    #[serde(flatten)]
    /// Can be relying party or user.
    pub entity: T,
}

/// Response structure from list security objects query.
#[derive(Clone, Debug, Default)]
pub struct ListSobjectsResponse {
    /// Metadata indicating filtered and total count.
    pub metadata: Option<Metadata>,
    /// List of security objects matching the filtering parameters.
    pub items: Vec<Sobject>,
}

impl ser::Serialize for ListSobjectsResponse {
    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        if self.metadata.is_some() {
            let mut state = serializer.serialize_struct("ListSobjectsResponse", 2)?;
            state.serialize_field(
                "metadata",
                &self.metadata.as_ref().expect("expected metadta"),
            )?;
            state.serialize_field("items", &self.items)?;
            state.end()
        } else {
            let mut seq = serializer.serialize_seq(Some(self.items.len()))?;
            for item in self.items.iter() {
                seq.serialize_element(item)?;
            }
            seq.end()
        }
    }
}

// This is only required for provider/sdkms to deserialize data for clients
impl<'de> Deserialize<'de> for ListSobjectsResponse {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ListSobjectsResponseVisitor;

        impl<'de> Visitor<'de> for ListSobjectsResponseVisitor {
            type Value = ListSobjectsResponse;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("ListSobjectsResponseItem")
            }

            fn visit_seq<V>(
                self,
                mut seq: V,
            ) -> ::std::result::Result<ListSobjectsResponse, V::Error>
            where
                V: serde::de::SeqAccess<'de>,
            {
                let mut items: Vec<Sobject> = Vec::new();
                loop {
                    let data = seq.next_element()?;
                    if let Some(item) = data {
                        items.push(item);
                    } else {
                        break;
                    }
                }
                Ok(ListSobjectsResponse {
                    metadata: None,
                    items,
                })
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> ::std::result::Result<ListSobjectsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut metadata = None;
                let mut items = Vec::new();
                loop {
                    if let Some(key) = map.next_key::<String>()? {
                        match key.as_str() {
                            "metadata" => metadata = map.next_value()?,
                            "items" => items = map.next_value()?,
                            other => {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&format!(
                                        "unexpected key {}",
                                        other
                                    )),
                                    &self,
                                ))
                            }
                        }
                    } else {
                        break;
                    }
                }

                Ok(ListSobjectsResponse { items, metadata })
            }
        }

        deserializer.deserialize_seq(ListSobjectsResponseVisitor)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HyperHttpMethod(pub Method);

impl Serialize for HyperHttpMethod {
    fn serialize<S: Serializer>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for HyperHttpMethod {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> ::std::result::Result<Self, D::Error> {
        let method = String::deserialize(deserializer)?
            .parse::<Method>()
            .map_err(D::Error::custom)?;

        Ok(HyperHttpMethod(method))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CustomMetadata(pub HashMap<String, String>);


impl crate::operations::UrlEncode for CustomMetadata {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        for (key, value) in self.0.clone().into_iter() {
            m.insert(key, value);
        }
    }
}


mod custom_metadata_params_de {
    use super::CustomMetadata;
    use serde::de::{self, Deserialize, Deserializer, IgnoredAny, MapAccess, Visitor};
    use std::collections::HashMap;
    use std::fmt;

    enum Key {
        Key(String),
        Ignored,
    }

    struct KeyVisitor;
    impl<'de> Visitor<'de> for KeyVisitor {
        type Value = Key;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("`custom_metadata.{key}` where {key} is an arbitrary identifier")
        }

        fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
            let prefix = "custom_metadata.";
            if value.starts_with(prefix) {
                return Ok(Key::Key(value[prefix.len()..].to_owned()));
            }
            Ok(Key::Ignored)
        }
    }

    impl<'de> Deserialize<'de> for Key {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            deserializer.deserialize_identifier(KeyVisitor)
        }
    }

    struct CMVisitor;
    impl<'de> Visitor<'de> for CMVisitor {
        type Value = CustomMetadata;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("map containing key/value pairs of form custom_metadata.K = V ")
        }

        fn visit_map<M: MapAccess<'de>>(self, mut access: M) -> Result<Self::Value, M::Error> {
            let mut map = HashMap::with_capacity(access.size_hint().unwrap_or(0));

            while let Some(key) = access.next_key::<Key>()? {
                match key {
                    Key::Key(key) => {
                        let value = access.next_value::<String>()?;
                        map.insert(key, value);
                    }
                    Key::Ignored => {
                        let _ = access.next_value::<IgnoredAny>()?;
                    }
                }
            }
            Ok(CustomMetadata(map))
        }
    }

    impl<'de> Deserialize<'de> for CustomMetadata {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            deserializer.deserialize_map(CMVisitor)
        }
    }
}

impl Serialize for CustomMetadata {
    fn serialize<S: Serializer>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error> {
        self.0
            .iter()
            .map(|(k, v)| (format!("custom_metadata.{}", k), v))
            .collect::<HashMap<_, _>>()
            .serialize(serializer)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// https://www.w3.org/TR/webauthn-2/#typedefdef-cosealgorithmidentifier
///
/// Signing algorithms from [IANA COSE Algorithms registry]
/// that are supported on DSM side for verifying signed messages
/// from authenticator.
///
/// [IANA COSE Algorithms registry]: https://www.iana.org/assignments/cose/cose.xhtml#algorithms
// COSE allows alg to be a tstr or int but [COSEAlgorithmIdentifier] is
// speicifically an int as per WebAuthn.
pub enum COSEAlgorithmIdentifier {
    /// ECDSA w/ SHA-256
    /// https://datatracker.ietf.org/doc/draft-ietf-cose-rfc8152bis-algs/12/
    Es256 = -7,
}

impl Serialize for COSEAlgorithmIdentifier {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(*self as i64)
    }
}

impl<'de> Deserialize<'de> for COSEAlgorithmIdentifier {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: i64 = Deserialize::deserialize(deserializer)?;

        Ok(match value {
            -7 => COSEAlgorithmIdentifier::Es256,
            value => return Err(D::Error::invalid_value(
                Unexpected::Signed(value),
                &"a signing algo value from IANA COSE Algorithms registry that is supported by DSM",
            )),
        })
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
