/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct AuthDiscoverParams {
    /// The account for which the user wishes to discover authentication mechanisms.
    /// Note that if specified, the user_email field in an AuthDiscoverRequest will
    /// be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acct_id: Option<Uuid>
}

impl UrlEncode for AuthDiscoverParams {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        if let Some(ref v) = self.acct_id {
            m.insert("acct_id".to_string(), v.to_string());
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthDiscoverRequest {
    /// The email of the user wishing to log in. If the auth discovery
    /// request also includes an acct_id query parameter, this field
    /// will be effectively ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_email: Option<String>
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case", tag = "method")]
pub enum AuthMethod {
    Password,
    SamlPost {
        name: String,
        icon_url: String,
        id: String,
        binding_url: String,
        authn_request: String,
        idp_id: Blob
    },
    OauthAuthCodeGrant {
        name: String,
        icon_url: String,
        authorization_url: String,
        client_id: String,
        redirect_uri: String,
        state: String,
        idp_id: Blob,
        auth_params: OauthAuthenticationParameters
    },
    LdapPassword {
        name: String,
        icon_url: String,
        idp_id: Blob
    },
    Vcd {
        name: String,
        authorization_url: String,
        idp_id: Blob
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case", tag = "method")]
pub enum AuthRequest {
    SamlResponse {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        response: String
    },
    OauthAuthCode (
        OauthCodeData
    ),
    LdapBasicAuth {
        idp_id: Blob,
        email: String,
        password: ZeroizedString,
        /// The account where the IdP is configured. This should
        /// only be used if attempting to self-provision into the
        /// account. (Self-provisioning may not be possible for
        /// existing users; they may need to be manually invited
        /// into the account.)
        #[serde(skip_serializing_if = "Option::is_none")]
        acct_id: Option<Uuid>
    },
    AuthByAppName {
        acct_id: Uuid,
        name: String,
        password: ZeroizedString
    },
    AwsIam {
        acct_id: Uuid,
        region: String,
        headers: HashMap<String,String>
    },
    /// Login using a DSM user password and, optionally, a captcha. This is useful
    /// for situations where a user is locked out of their DSM account, and wants to
    /// become unlocked.
    Password {
        /// The user's email.
        email: String,
        /// The user's password.
        password: ZeroizedString,
        /// The response token after solving a reCAPTCHA successfully.
        #[serde(skip_serializing_if = "Option::is_none")]
        recaptcha_response: Option<String>
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthResponse {
    pub token_type: String,
    pub expires_in: u32,
    pub access_token: ZeroizedString,
    pub entity_id: Uuid,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub challenge: Option<U2fMfaChallengeResponse>,
    /// Its presence indicates that 2FA is required for this
    /// session and contains response that should be used with
    /// `navigator.credentials.get()`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fido2_assertion_options: Option<PublicKeyCredentialRequestOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mfa_methods: Option<Vec<MfaAuthMethod>>
}

/// Temporary credentials to be used for AWS KMS.
#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone)]
pub struct AwsTemporaryCredentials {
    pub access_key: String,
    pub secret_key: ZeroizedString,
    pub session_token: ZeroizedString
}

/// Request to start configuring U2F.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Config2faAuthRequest {
    pub password: ZeroizedString
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Config2faAuthResponse {

}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub enum MfaAuthMethod {
    Fido2 {
        challenge: PublicKeyCredentialRequestOptions,
        challenge_token: Blob,
        mfa_devices: Vec<MfaDevice>
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OauthCodeData {
    pub idp_id: Blob,
    pub code: String,
    pub email: String
}

/// Request to authenticate using U2F recovery code.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct RecoveryCodeAuthRequest {
    pub recovery_code: ZeroizedString
}

/// Request to select an account.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SelectAccountRequest {
    pub acct_id: Uuid
}

/// Response to select account request.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SelectAccountResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cookie: Option<String>
}

pub struct OperationAuthDiscover;
#[allow(unused)]
impl Operation for OperationAuthDiscover {
    type PathParams = ();
    type QueryParams = AuthDiscoverParams;
    type Body = AuthDiscoverRequest;
    type Output = Vec<AuthMethod>;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/session/auth/discover?{q}", q = q.encode())
    }
}

impl SdkmsClient {
    pub fn auth_discover(&self, query_params: Option<&AuthDiscoverParams>, req: &AuthDiscoverRequest) -> Result<Vec<AuthMethod>> {
        self.execute::<OperationAuthDiscover>(req, (), query_params)
    }
}

pub struct OperationAuthenticate;
#[allow(unused)]
impl Operation for OperationAuthenticate {
    type PathParams = ();
    type QueryParams = ();
    type Body = AuthRequest;
    type Output = AuthResponse;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/sys/v1/session/auth".to_string()
    }
}

impl SdkmsClient {
    pub fn authenticate(&self, req: &AuthRequest) -> Result<AuthResponse> {
        self.execute::<OperationAuthenticate>(req, (), None)
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
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/sys/v1/session/config_2fa/auth".to_string()
    }
}

impl SdkmsClient {
    pub fn config_2fa_auth(&self, req: &Config2faAuthRequest) -> Result<Config2faAuthResponse> {
        self.execute::<OperationConfig2faAuth>(req, (), None)
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
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/sys/v1/session/config_2fa/terminate".to_string()
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn config_2fa_terminate(&self) -> Result<()> {
        self.execute::<OperationConfig2faTerminate>(&(), (), None)
    }
}

pub struct OperationReauthenticate;
#[allow(unused)]
impl Operation for OperationReauthenticate {
    type PathParams = ();
    type QueryParams = ();
    type Body = ();
    type Output = AuthResponse;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/sys/v1/session/reauth".to_string()
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn reauthenticate(&self) -> Result<AuthResponse> {
        self.execute::<OperationReauthenticate>(&(), (), None)
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
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/sys/v1/session/auth/2fa/recovery_code".to_string()
    }
}

impl SdkmsClient {
    pub fn recovery_code_auth(&self, req: &RecoveryCodeAuthRequest) -> Result<()> {
        self.execute::<OperationRecoveryCodeAuth>(req, (), None)
    }
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
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/sys/v1/session/refresh".to_string()
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn refresh(&self) -> Result<()> {
        self.execute::<OperationRefresh>(&(), (), None)
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
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/sys/v1/session/select_account".to_string()
    }
}

impl SdkmsClient {
    pub fn select_account(&self, req: &SelectAccountRequest) -> Result<SelectAccountResponse> {
        self.execute::<OperationSelectAccount>(req, (), None)
    }
}

pub struct OperationSetAwsTemporaryCredentials;
#[allow(unused)]
impl Operation for OperationSetAwsTemporaryCredentials {
    type PathParams = ();
    type QueryParams = ();
    type Body = AwsTemporaryCredentials;
    type Output = ();

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/sys/v1/session/aws_temporary_credentials".to_string()
    }
}

impl SdkmsClient {
    pub fn set_aws_temporary_credentials(&self, req: &AwsTemporaryCredentials) -> Result<()> {
        self.execute::<OperationSetAwsTemporaryCredentials>(req, (), None)
    }
}

pub struct OperationTerminate;
#[allow(unused)]
impl Operation for OperationTerminate {
    type PathParams = ();
    type QueryParams = ();
    type Body = ();
    type Output = ();

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/sys/v1/session/terminate".to_string()
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn terminate(&self) -> Result<()> {
        self.execute::<OperationTerminate>(&(), (), None)
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
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/sys/v1/session/auth/2fa/u2f".to_string()
    }
}

impl SdkmsClient {
    pub fn u2f_auth(&self, req: &U2fAuthRequest) -> Result<()> {
        self.execute::<OperationU2fAuth>(req, (), None)
    }
}

