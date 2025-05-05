/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

/// Server execution mode.
#[derive(Copy, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub enum ServerMode {
    Software,
    Sgx
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct VersionParams {
    /// Include FIPS-relevant info in the response. Currently that is `plugins_digest`.
    ///
    /// Only applicable to FIPS builds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_fips_info: Option<bool>
}

impl UrlEncode for VersionParams {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        if let Some(ref v) = self.detailed_fips_info {
            m.insert("detailed_fips_info".to_string(), v.to_string());
        }
    }
}

/// Information about the service version.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct VersionResponse {
    /// Server version. This is encoded as "major.minor.build".
    pub version: String,
    /// The API version implemented by the server.
    pub api_version: String,
    pub server_mode: ServerMode,
    /// FIPS level at which the service in running. If this field is absent, then the service is
    /// not running in FIPS compliant mode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fips_level: Option<u8>,
    /// An opaque digest of all current plugins.
    ///
    /// Only present when the server is running in FIPS mode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugins_digest: Option<Blob>
}

pub struct OperationVersion;
#[allow(unused)]
impl Operation for OperationVersion {
    type PathParams = ();
    type QueryParams = VersionParams;
    type Body = ();
    type Output = VersionResponse;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/version?{q}", q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn version(&self, query_params: Option<&VersionParams>) -> Result<VersionResponse> {
        self.execute::<OperationVersion>(&(), (), query_params)
    }
}

