/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

/// A challenge used for multi-factor authentication.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct MfaChallengeResponse {
    pub u2f_challenge: String,
    pub u2f_keys: Vec<U2fRegisteredKey>,
}

/// Description of a registered U2F device.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct U2fRegisteredKey {
    pub key_handle: String,
    pub version: String,
}

/// Request to select an account.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SelectAccountRequest {
    pub acct_id: Uuid,
}

/// Response to select account request.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SelectAccountResponse {
    #[serde(default)]
    pub cookie: Option<String>,
}

/// Request to start configuring U2F.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Config2faAuthRequest {
    pub password: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Config2faAuthResponse {}

/// Request to authenticate using U2F recovery code.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct RecoveryCodeAuthRequest {
    pub recovery_code: String,
}

pub struct OperationRefresh;
#[allow(unused)]
impl Operation for OperationRefresh {
    type PathParams = ();
    type QueryParams = ();
    type Body = ();
    type Output = ();

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/sys/v1/session/refresh")
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

impl SdkmsClient {
    pub fn refresh(&self) -> Result<()> {
        self.execute::<OperationRefresh>(&(), (), &())
    }
}

pub struct OperationSelectAccount;
#[allow(unused)]
impl Operation for OperationSelectAccount {
    type PathParams = ();
    type QueryParams = ();
    type Body = SelectAccountRequest;
    type Output = SelectAccountResponse;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/sys/v1/session/select_account")
    }
}

impl SdkmsClient {
    pub fn select_account(&self, req: &SelectAccountRequest) -> Result<SelectAccountResponse> {
        self.execute::<OperationSelectAccount>(req, (), &())
    }
}

pub struct OperationU2fAuth;
#[allow(unused)]
impl Operation for OperationU2fAuth {
    type PathParams = ();
    type QueryParams = ();
    type Body = U2fAuthRequest;
    type Output = ();

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/sys/v1/session/auth/2fa/u2f")
    }
}

impl SdkmsClient {
    pub fn u2f_auth(&self, req: &U2fAuthRequest) -> Result<()> {
        self.execute::<OperationU2fAuth>(req, (), &())
    }
}

pub struct OperationRecoveryCodeAuth;
#[allow(unused)]
impl Operation for OperationRecoveryCodeAuth {
    type PathParams = ();
    type QueryParams = ();
    type Body = RecoveryCodeAuthRequest;
    type Output = ();

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/sys/v1/session/auth/2fa/recovery_code")
    }
}

impl SdkmsClient {
    pub fn recovery_code_auth(&self, req: &RecoveryCodeAuthRequest) -> Result<()> {
        self.execute::<OperationRecoveryCodeAuth>(req, (), &())
    }
}

pub struct OperationConfig2faAuth;
#[allow(unused)]
impl Operation for OperationConfig2faAuth {
    type PathParams = ();
    type QueryParams = ();
    type Body = Config2faAuthRequest;
    type Output = Config2faAuthResponse;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/sys/v1/session/config_2fa/auth")
    }
}

impl SdkmsClient {
    pub fn config_2fa_auth(&self, req: &Config2faAuthRequest) -> Result<Config2faAuthResponse> {
        self.execute::<OperationConfig2faAuth>(req, (), &())
    }
}

pub struct OperationConfig2faTerminate;
#[allow(unused)]
impl Operation for OperationConfig2faTerminate {
    type PathParams = ();
    type QueryParams = ();
    type Body = ();
    type Output = ();

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/sys/v1/session/config_2fa/terminate")
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

impl SdkmsClient {
    pub fn config_2fa_terminate(&self) -> Result<()> {
        self.execute::<OperationConfig2faTerminate>(&(), (), &())
    }
}

pub struct OperationU2fNewChallenge;
#[allow(unused)]
impl Operation for OperationU2fNewChallenge {
    type PathParams = ();
    type QueryParams = ();
    type Body = ();
    type Output = MfaChallengeResponse;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/sys/v1/session/config_2fa/new_challenge")
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

impl SdkmsClient {
    pub fn u2f_new_challenge(&self) -> Result<MfaChallengeResponse> {
        self.execute::<OperationU2fNewChallenge>(&(), (), &())
    }
}
