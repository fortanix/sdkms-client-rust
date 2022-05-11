/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ConfirmEmailRequest {
    pub confirm_token: String,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ConfirmEmailResponse {
    pub user_email: String,
}

/// Initiate password reset sequence.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForgotPasswordRequest {
    pub user_email: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ListUsersParams {
    pub group_id: Option<Uuid>,
    pub acct_id: Option<Uuid>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    #[serde(flatten)]
    pub sort: UserSort,
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
    }
}

/// Request to change user's password.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PasswordChangeRequest {
    pub current_password: String,
    pub new_password: String,
}

/// Request to perform a password reset.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PasswordResetRequest {
    pub reset_token: String,
    pub new_password: String,
}

/// Accept/reject invitations to join account.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessInviteRequest {
    /// Optional list of account IDs to accept.
    #[serde(default)]
    pub accepts: Option<HashSet<Uuid>>,
    /// Optional list of account IDs to reject.
    #[serde(default)]
    pub rejects: Option<HashSet<Uuid>>,
}

/// U2F recovery codes.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct RecoveryCodes {
    pub recovery_codes: Vec<String>,
}

/// Request to signup a new user.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignupRequest {
    pub user_email: String,
    pub user_password: String,
    #[serde(default)]
    pub recaptcha_response: Option<String>,
    #[serde(default)]
    pub first_name: Option<String>,
    #[serde(default)]
    pub last_name: Option<String>,
}

/// Description of a U2F device to add for two factor authentication.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct U2fAddDeviceRequest {
    pub name: String,
    pub registration_data: Blob,
    pub client_data: Blob,
    pub version: String,
}

/// Request to delete a U2F device.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct U2fDelDeviceRequest {
    pub name: String,
}

/// A U2f device that may be used for second factor authentication.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct U2fDevice {
    pub name: String,
}

/// Request to rename a U2F device.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct U2fRenameDeviceRequest {
    pub old_name: String,
    pub new_name: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct User {
    pub account_role: UserAccountFlags,
    #[serde(default)]
    pub created_at: Option<Time>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub email_verified: Option<bool>,
    #[serde(default)]
    pub first_name: Option<String>,
    pub groups: HashMap<Uuid, UserGroupRole>,
    #[serde(default)]
    pub has_account: Option<bool>,
    #[serde(default)]
    pub has_password: Option<bool>,
    #[serde(default)]
    pub last_logged_in_at: Option<Time>,
    #[serde(default)]
    pub last_name: Option<String>,
    #[serde(default)]
    pub new_email: Option<String>,
    pub u2f_devices: Vec<U2fDevice>,
    #[serde(default)]
    pub user_email: Option<String>,
    pub user_id: Uuid,
}

/// User's role and state in an account.
pub use self::user_flags::UserAccountFlags;
pub mod user_flags {
    bitflags_set! {
        pub struct UserAccountFlags: u64 {
            const ACCOUNTADMINISTRATOR = 0x0000000000000001;
            const ACCOUNTMEMBER = 0x0000000000000002;
            const ACCOUNTAUDITOR = 0x0000000000000004;
            const STATEENABLED = 0x0000000000000008;
            const PENDINGINVITE = 0x0000000000000010;
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct UserRequest {
    #[serde(default)]
    pub account_role: Option<UserAccountFlags>,
    #[serde(default)]
    pub add_groups: Option<HashMap<Uuid, UserGroupRole>>,
    #[serde(default)]
    pub add_u2f_devices: Option<Vec<U2fAddDeviceRequest>>,
    #[serde(default)]
    pub del_groups: Option<HashMap<Uuid, UserGroupRole>>,
    #[serde(default)]
    pub del_u2f_devices: Option<Vec<U2fDelDeviceRequest>>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub enable: Option<bool>,
    #[serde(default)]
    pub first_name: Option<String>,
    #[serde(default)]
    pub last_name: Option<String>,
    #[serde(default)]
    pub mod_groups: Option<HashMap<Uuid, UserGroupRole>>,
    #[serde(default)]
    pub rename_u2f_devices: Option<Vec<U2fRenameDeviceRequest>>,
    #[serde(default)]
    pub user_email: Option<String>,
    #[serde(default)]
    pub user_password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserSort {
    ByUserId { order: Order, start: Option<Uuid> },
}

impl UrlEncode for UserSort {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        match *self {
            UserSort::ByUserId {
                ref order,
                ref start,
            } => {
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
    pub reset_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValidateTokenResponse {
    pub user_email: String,
}

pub struct OperationChangePassword;
#[allow(unused)]
impl Operation for OperationChangePassword {
    type PathParams = ();
    type QueryParams = ();
    type Body = PasswordChangeRequest;
    type Output = ();

    fn method() -> Method {
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/users/change_password")
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
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/users/{id}/confirm_email", id = p.0)
    }
}

impl SdkmsClient {
    pub fn confirm_email(
        &self,
        id: &Uuid,
        req: &ConfirmEmailRequest,
    ) -> Result<ConfirmEmailResponse> {
        self.execute::<OperationConfirmEmail>(req, (id,), None)
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
        Method::DELETE
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/users/{id}", id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

impl SdkmsClient {
    pub fn delete_stale(&self, id: &Uuid) -> Result<()> {
        self.execute::<OperationDeleteStale>(&(), (id,), None)
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
        Method::DELETE
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/users")
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

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
        Method::DELETE
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/users/{id}/accounts", id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

impl SdkmsClient {
    pub fn delete_user_account(&self, id: &Uuid) -> Result<()> {
        self.execute::<OperationDeleteUserAccount>(&(), (id,), None)
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
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/users/forgot_password")
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
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/users/generate_recovery_codes")
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

impl SdkmsClient {
    pub fn generate_recovery_codes(&self) -> Result<RecoveryCodes> {
        self.execute::<OperationGenerateRecoveryCodes>(&(), (), None)
    }
}

pub struct OperationGetUser;
#[allow(unused)]
impl Operation for OperationGetUser {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = User;

    fn method() -> Method {
        Method::GET
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/users/{id}", id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

impl SdkmsClient {
    pub fn get_user(&self, id: &Uuid) -> Result<User> {
        self.execute::<OperationGetUser>(&(), (id,), None)
    }
}

pub struct OperationGetUserAccounts;
#[allow(unused)]
impl Operation for OperationGetUserAccounts {
    type PathParams = ();
    type QueryParams = ();
    type Body = ();
    type Output = HashMap<Uuid, UserAccountFlags>;

    fn method() -> Method {
        Method::GET
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/users/accounts")
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

impl SdkmsClient {
    pub fn get_user_accounts(&self) -> Result<HashMap<Uuid, UserAccountFlags>> {
        self.execute::<OperationGetUserAccounts>(&(), (), None)
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
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/users/invite")
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
        Method::GET
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/users?{q}", q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

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
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/users/process_invite")
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
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/users/resend_confirm_email")
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

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
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/users/{id}/resend_invite", id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

impl SdkmsClient {
    pub fn resend_invite(&self, id: &Uuid) -> Result<()> {
        self.execute::<OperationResendInvite>(&(), (id,), None)
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
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/users/{id}/reset_password", id = p.0)
    }
}

impl SdkmsClient {
    pub fn reset_password(&self, id: &Uuid, req: &PasswordResetRequest) -> Result<()> {
        self.execute::<OperationResetPassword>(req, (id,), None)
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
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/users")
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
        Method::PATCH
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/users/{id}", id = p.0)
    }
}

impl SdkmsClient {
    pub fn update_user(&self, id: &Uuid, req: &UserRequest) -> Result<User> {
        self.execute::<OperationUpdateUser>(req, (id,), None)
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
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/users/{id}/validate_token", id = p.0)
    }
}

impl SdkmsClient {
    pub fn validate_token(
        &self,
        id: &Uuid,
        req: &ValidateTokenRequest,
    ) -> Result<ValidateTokenResponse> {
        self.execute::<OperationValidateToken>(req, (id,), None)
    }
}
