/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

/// Server mode.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub enum ServerMode {
    Software,
    Sgx,
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
    #[serde(default)]
    pub fips_level: Option<u8>,
}

pub struct OperationVersion;
#[allow(unused)]
impl Operation for OperationVersion {
    type PathParams = ();
    type QueryParams = ();
    type Body = ();
    type Output = VersionResponse;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/version")
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

impl SdkmsClient {
    pub fn version(&self) -> Result<VersionResponse> {
        self.execute::<OperationVersion>(&(), (), None)
    }
}
