/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

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
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticatorAttestationResponse {
    /// Base64url of [crate::fido2::models::CollectedClientData] in JSON form.
    #[serde(rename = "clientDataJSON")]
    pub client_data_json: Base64<UrlSafe>,
    /// Values obtained from `AuthenticatorAttestationResponse.getTransports()`.
    /// Webauthn spec recommends RP to store it and user them along with
    /// `allowCredentials` while authentication ceremony.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_transports: Option<Vec<AuthenticatorTransport>>,
    /// Base64url of the attestation object.
    ///
    /// See in order:
    /// <https://www.w3.org/TR/webauthn-2/#dom-authenticatorattestationresponse-attestationobject>
    /// <https://www.w3.org/TR/webauthn-2/#sctn-attestation>
    /// <https://www.w3.org/TR/webauthn-2/#sctn-defined-attestation-formats>
    ///
    /// Currently, only U2F is supported, others will be rejected.
    pub attestation_object: Base64<UrlSafe>
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ConfirmEmailRequest {
    pub confirm_token: ZeroizedString
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ConfirmEmailResponse {
    pub user_email: String
}

/// This contains the request for adding a FIDO device
/// to user's data.
/// Initially, `POST /sys/v1/session/config_2fa/new_challenge` needs
/// to be called with protocol set to `fido2` and using that data,
/// `navigator.credentials.create()` is called in the frontend.
/// The data returned by `create` is sent in this request. The data
/// sent back here creates a new FIDO2 device for the user after
/// the payload is verified as per the rules stated in webauthn doc.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FidoAddDeviceRequest {
    /// A user friendly name for the device.
    pub name: String,
    /// Result of calling `navigator.credentials.create()` with the
    /// data obtained from `new_challenge` API.
    pub attestation_result: PublicKeyCredential<AuthenticatorAttestationResponse>
}

/// Initiate password reset sequence.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForgotPasswordRequest {
    pub user_email: String
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GetUserParams {
    /// Controls whether groups and explicit_groups fields are present in the response.
    /// Not including the groups field in the response can reduce the latency of the API,
    /// especially when the account contains a large number of groups and a large number
    /// of users with account-wide roles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_groups: Option<WithGroups>
}

impl UrlEncode for GetUserParams {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        if let Some(ref v) = self.with_groups {
            m.insert("with_groups".to_string(), v.to_string());
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetUserPermissionsParams {
    /// If `true`, implied permissions are added in the output. For example, if
    /// permission A implies permission B, and the user has permission A, the
    /// output will include both A and B if this is set to `true`. If this is
    /// set to `false`, B will only be returned if it was assigned to the user
    /// directly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_implied: Option<bool>
}

impl UrlEncode for GetUserPermissionsParams {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        if let Some(ref v) = self.with_implied {
            m.insert("with_implied".to_string(), v.to_string());
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetUserPermissionsResponse {
    /// User's permissions in the account.
    pub account: AccountPermissions,
    /// User's permissions in all groups. Note that this will only be returned
    /// if the user has one or more all-groups roles.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_groups: Option<GroupPermissions>,
    /// User's permissions in groups.
    pub groups: HashMap<Uuid,GroupPermissions>
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ListUsersParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acct_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    #[serde(flatten)]
    pub sort: UserSort,
    /// Controls whether groups and explicit_groups fields are present in the response.
    /// Not including the groups field in the response can reduce the latency of the API,
    /// especially when the account contains a large number of groups and a large number
    /// of users with account-wide roles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_groups: Option<WithGroups>
}

impl UrlEncode for ListUsersParams {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        if let Some(ref v) = self.group_id {
            m.insert("group_id".to_string(), v.to_string());
        }
        if let Some(ref v) = self.acct_id {
            m.insert("acct_id".to_string(), v.to_string());
        }
        if let Some(ref v) = self.limit {
            m.insert("limit".to_string(), v.to_string());
        }
        if let Some(ref v) = self.offset {
            m.insert("offset".to_string(), v.to_string());
        }
        self.sort.url_encode(m);
        if let Some(ref v) = self.with_groups {
            m.insert("with_groups".to_string(), v.to_string());
        }
    }
}

/// Request to delete a FIDO device.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct MfaDelDeviceRequest {
    /// Name of the FIDO device to delete.
    pub name: String
}

/// Request to rename a FIDO device.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct MfaRenameDeviceRequest {
    /// Old name of FIDO device.
    pub old_name: String,
    /// New name of FIDO device.
    pub new_name: String
}

/// Request to change user's password.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PasswordChangeRequest {
    pub current_password: ZeroizedString,
    pub new_password: ZeroizedString
}

/// Request to perform a password reset.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PasswordResetRequest {
    pub reset_token: ZeroizedString,
    pub new_password: ZeroizedString
}

/// Accept/reject invitations to join account.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessInviteRequest {
    /// Optional list of account IDs to accept.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accepts: Option<HashSet<Uuid>>,
    /// Optional list of account IDs to reject.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rejects: Option<HashSet<Uuid>>
}

/// U2F recovery codes.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct RecoveryCodes {
    pub recovery_codes: Vec<String>
}

/// Request to signup a new user.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignupRequest {
    pub user_email: String,
    pub user_password: ZeroizedString,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recaptcha_response: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>
}

/// Description of a U2F device to add for two factor authentication.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct U2fAddDeviceRequest {
    pub name: String,
    pub registration_data: Blob,
    pub client_data: Blob,
    pub version: String
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct User {
    pub account_role: UserAccountFlags,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Time>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependent_services: Option<BTreeSet<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,
    /// Explicit group assignments.
    /// 
    /// This is similar to `groups` field except that it does not include groups due to
    /// all-groups roles. Use this field to find out which group assignments can be
    /// changed using `mod_groups` and `del_groups` fields in user update API.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explicit_groups: Option<HashMap<Uuid,UserGroupRole>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<HashMap<Uuid,UserGroupRole>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_account: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_password: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_logged_in_at: Option<Time>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Mfa devices registered with the user
    pub mfa_devices: Vec<MfaDevice>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub self_provisioned: Option<bool>,
    pub u2f_devices: Vec<MfaDevice>,
    pub user_email: String,
    pub user_id: Uuid
}

#[derive(Default, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct UserRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_role: Option<UserAccountFlags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_groups: Option<HashMap<Uuid,UserGroupRole>>,
    /// FIDO devices to add. Only one device can be added at present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_mfa_devices: Option<Vec<FidoAddDeviceRequest>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_u2f_devices: Option<Vec<U2fAddDeviceRequest>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub del_groups: Option<HashMap<Uuid,UserGroupRole>>,
    /// Mfa devices to delete
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub del_mfa_devices: Option<Vec<MfaDelDeviceRequest>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub del_u2f_devices: Option<Vec<MfaDelDeviceRequest>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mod_groups: Option<HashMap<Uuid,UserGroupRole>>,
    /// Mfa devices to rename
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rename_mfa_devices: Option<Vec<MfaRenameDeviceRequest>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rename_u2f_devices: Option<Vec<MfaRenameDeviceRequest>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_password: Option<ZeroizedString>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserSort {
    ByUserId {
        order: Order,
        #[serde(skip_serializing_if = "Option::is_none")]
        start: Option<Uuid>
    }
}

impl UrlEncode for UserSort {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        match *self {
            UserSort::ByUserId{ ref order, ref start } => {
                m.insert("sort".to_string(), format!("user_id:{}", order));
                if let Some(v) = start {
                    m.insert("start".to_string(), v.to_string());
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValidateTokenRequest {
    pub reset_token: ZeroizedString
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValidateTokenResponse {
    pub user_email: String
}

#[derive(Copy, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum WithGroups {
    /// Both the `groups` and the `explicit_groups` fields are present in the response.
    All,
    /// The `groups` field is omitted from the response.
    ExplicitOnly,
    /// Both the `groups` and the `explicit_groups` fields are omitted from the response.
    None
}

pub struct OperationChangePassword;
#[allow(unused)]
impl Operation for OperationChangePassword {
    type PathParams = ();
    type QueryParams = ();
    type Body = PasswordChangeRequest;
    type Output = ();

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/sys/v1/users/change_password".to_string()
    }
}

impl SdkmsClient {
    pub fn change_password(&self, req: &PasswordChangeRequest) -> Result<()> {
        self.execute::<OperationChangePassword>(req, (), None)
    }
}

pub struct OperationConfirmEmail;
#[allow(unused)]
impl Operation for OperationConfirmEmail {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ConfirmEmailRequest;
    type Output = ConfirmEmailResponse;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/users/{user_id}/confirm_email", user_id = p.0)
    }
}

impl SdkmsClient {
    pub fn confirm_email(&self, user_id: &Uuid, req: &ConfirmEmailRequest) -> Result<ConfirmEmailResponse> {
        self.execute::<OperationConfirmEmail>(req, (user_id,), None)
    }
}

pub struct OperationDeleteStale;
#[allow(unused)]
impl Operation for OperationDeleteStale {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = ();

    fn method() -> Method {
        Method::Delete
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/users/{user_id}", user_id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn delete_stale(&self, user_id: &Uuid) -> Result<()> {
        self.execute::<OperationDeleteStale>(&(), (user_id,), None)
    }
}

pub struct OperationDeleteUser;
#[allow(unused)]
impl Operation for OperationDeleteUser {
    type PathParams = ();
    type QueryParams = ();
    type Body = ();
    type Output = ();

    fn method() -> Method {
        Method::Delete
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/sys/v1/users".to_string()
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn delete_user(&self) -> Result<()> {
        self.execute::<OperationDeleteUser>(&(), (), None)
    }
}

pub struct OperationDeleteUserAccount;
#[allow(unused)]
impl Operation for OperationDeleteUserAccount {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = ();

    fn method() -> Method {
        Method::Delete
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/users/{user_id}/accounts", user_id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn delete_user_account(&self, user_id: &Uuid) -> Result<()> {
        self.execute::<OperationDeleteUserAccount>(&(), (user_id,), None)
    }
}

pub struct OperationForgotPassword;
#[allow(unused)]
impl Operation for OperationForgotPassword {
    type PathParams = ();
    type QueryParams = ();
    type Body = ForgotPasswordRequest;
    type Output = ();

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/sys/v1/users/forgot_password".to_string()
    }
}

impl SdkmsClient {
    pub fn forgot_password(&self, req: &ForgotPasswordRequest) -> Result<()> {
        self.execute::<OperationForgotPassword>(req, (), None)
    }
}

pub struct OperationGenerateRecoveryCodes;
#[allow(unused)]
impl Operation for OperationGenerateRecoveryCodes {
    type PathParams = ();
    type QueryParams = ();
    type Body = ();
    type Output = RecoveryCodes;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/sys/v1/users/generate_recovery_codes".to_string()
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn generate_recovery_codes(&self) -> Result<RecoveryCodes> {
        self.execute::<OperationGenerateRecoveryCodes>(&(), (), None)
    }
}

pub struct OperationGetUser;
#[allow(unused)]
impl Operation for OperationGetUser {
    type PathParams = (Uuid,);
    type QueryParams = GetUserParams;
    type Body = ();
    type Output = User;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/users/{user_id}?{q}", user_id = p.0, q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_user(&self, user_id: &Uuid, query_params: Option<&GetUserParams>) -> Result<User> {
        self.execute::<OperationGetUser>(&(), (user_id,), query_params)
    }
}

pub struct OperationGetUserAccounts;
#[allow(unused)]
impl Operation for OperationGetUserAccounts {
    type PathParams = ();
    type QueryParams = ();
    type Body = ();
    type Output = HashMap<Uuid,UserAccountFlags>;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/sys/v1/users/accounts".to_string()
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_user_accounts(&self) -> Result<HashMap<Uuid,UserAccountFlags>> {
        self.execute::<OperationGetUserAccounts>(&(), (), None)
    }
}

pub struct OperationGetUserPermissions;
#[allow(unused)]
impl Operation for OperationGetUserPermissions {
    type PathParams = ();
    type QueryParams = GetUserPermissionsParams;
    type Body = ();
    type Output = GetUserPermissionsResponse;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/users/permissions?{q}", q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_user_permissions(&self, query_params: Option<&GetUserPermissionsParams>) -> Result<GetUserPermissionsResponse> {
        self.execute::<OperationGetUserPermissions>(&(), (), query_params)
    }
}

pub struct OperationInviteUser;
#[allow(unused)]
impl Operation for OperationInviteUser {
    type PathParams = ();
    type QueryParams = ();
    type Body = UserRequest;
    type Output = User;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/sys/v1/users/invite".to_string()
    }
}

impl SdkmsClient {
    pub fn invite_user(&self, req: &UserRequest) -> Result<User> {
        self.execute::<OperationInviteUser>(req, (), None)
    }
}

pub struct OperationListUsers;
#[allow(unused)]
impl Operation for OperationListUsers {
    type PathParams = ();
    type QueryParams = ListUsersParams;
    type Body = ();
    type Output = Vec<User>;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/users?{q}", q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn list_users(&self, query_params: Option<&ListUsersParams>) -> Result<Vec<User>> {
        self.execute::<OperationListUsers>(&(), (), query_params)
    }
}

pub struct OperationProcessInvite;
#[allow(unused)]
impl Operation for OperationProcessInvite {
    type PathParams = ();
    type QueryParams = ();
    type Body = ProcessInviteRequest;
    type Output = ();

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/sys/v1/users/process_invite".to_string()
    }
}

impl SdkmsClient {
    pub fn process_invite(&self, req: &ProcessInviteRequest) -> Result<()> {
        self.execute::<OperationProcessInvite>(req, (), None)
    }
}

pub struct OperationResendConfirmEmail;
#[allow(unused)]
impl Operation for OperationResendConfirmEmail {
    type PathParams = ();
    type QueryParams = ();
    type Body = ();
    type Output = ();

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/sys/v1/users/resend_confirm_email".to_string()
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn resend_confirm_email(&self) -> Result<()> {
        self.execute::<OperationResendConfirmEmail>(&(), (), None)
    }
}

pub struct OperationResendInvite;
#[allow(unused)]
impl Operation for OperationResendInvite {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = ();

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/users/{user_id}/resend_invite", user_id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn resend_invite(&self, user_id: &Uuid) -> Result<()> {
        self.execute::<OperationResendInvite>(&(), (user_id,), None)
    }
}

pub struct OperationResetPassword;
#[allow(unused)]
impl Operation for OperationResetPassword {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = PasswordResetRequest;
    type Output = ();

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/users/{user_id}/reset_password", user_id = p.0)
    }
}

impl SdkmsClient {
    pub fn reset_password(&self, user_id: &Uuid, req: &PasswordResetRequest) -> Result<()> {
        self.execute::<OperationResetPassword>(req, (user_id,), None)
    }
}

pub struct OperationSignupUser;
#[allow(unused)]
impl Operation for OperationSignupUser {
    type PathParams = ();
    type QueryParams = ();
    type Body = SignupRequest;
    type Output = User;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/sys/v1/users".to_string()
    }
}

impl SdkmsClient {
    pub fn signup_user(&self, req: &SignupRequest) -> Result<User> {
        self.execute::<OperationSignupUser>(req, (), None)
    }
}

pub struct OperationUpdateUser;
#[allow(unused)]
impl Operation for OperationUpdateUser {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = UserRequest;
    type Output = User;

    fn method() -> Method {
        Method::Patch
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/users/{user_id}", user_id = p.0)
    }
}

impl SdkmsClient {
    pub fn update_user(&self, user_id: &Uuid, req: &UserRequest) -> Result<User> {
        self.execute::<OperationUpdateUser>(req, (user_id,), None)
    }
}

pub struct OperationValidateToken;
#[allow(unused)]
impl Operation for OperationValidateToken {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ValidateTokenRequest;
    type Output = ValidateTokenResponse;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/users/{user_id}/validate_token", user_id = p.0)
    }
}

impl SdkmsClient {
    pub fn validate_token(&self, user_id: &Uuid, req: &ValidateTokenRequest) -> Result<ValidateTokenResponse> {
        self.execute::<OperationValidateToken>(req, (user_id,), None)
    }
}

