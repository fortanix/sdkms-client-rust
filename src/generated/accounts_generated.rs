/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

/// Type of subscription.
#[derive(Copy, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionType {
    Trial {
        expires_at: Option<Time>,
    },
    Standard,
    Enterprise,
    Custom {
        max_plugin: Option<u32>,
        max_operation: Option<u64>,
    },
    OnPrem,
    Reseller {
        max_plugin: Option<u32>,
        max_operation: Option<u64>,
        max_tenant: Option<u32>,
        max_tenant_plugin: Option<u32>,
        max_tenant_operation: Option<u64>,
    },
}

/// A request to update subscription type.
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct SubscriptionChangeRequest {
    pub subscription: SubscriptionType,
    #[serde(default)]
    pub contact: Option<String>,
    #[serde(default)]
    pub comment: Option<String>,
}

/// Notification preferences.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub enum NotificationPref {
    None,
    Email,
    Phone,
    Both,
}

/// Password authentication settings.
#[derive(PartialEq, Eq, Debug, Default, Serialize, Deserialize, Clone)]
pub struct AuthConfigPassword {
    pub require_2fa: bool,
    pub administrators_only: bool,
}

/// OAuth single sign-on authentication settings.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct AuthConfigOauth {
    pub idp_name: String,
    pub idp_icon_url: String,
    pub idp_authorization_endpoint: String,
    pub idp_token_endpoint: String,
    #[serde(default)]
    pub idp_userinfo_endpoint: Option<String>,
    pub idp_requires_basic_auth: bool,
    pub tls: TlsConfig,
    pub client_id: String,
    pub client_secret: String,
}

/// Credentials used by the service to authenticate itself to an LDAP server.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct LdapServiceAccount {
    pub dn: String,
    pub password: String,
}

/// Distinguished Name (DN) resolution method. Given a user's email address, a DN resolution method
/// is used to find the user's DN in an LDAP directory.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case", tag = "method")]
pub enum LdapDnResolution {
    /// Transform the user email through a pattern to derive the DN.
    Construct {
        /// For example: "example.com" => "uid={},ou=users,dc=example,dc=com".
        domain_format: HashMap<String, String>,
    },
    /// Search the directory using the LDAP `mail` attribute matching user's email.
    SearchByMail,
    /// Use email in place of DN. This method works with Active Directory if the userPrincipalName
    /// attribute is set for the user. https://docs.microsoft.com/en-us/windows/desktop/ad/naming-properties
    #[serde(rename = "upn")]
    UserPrincipalName,
}

/// LDAP authorization settings.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct LdapAuthorizationConfig {
    /// Number of seconds after which the authorization should be checked again.
    pub valid_for: u64,
    /// Distinguished name of an LDAP group. If specified, account members must be a member of this
    /// LDAP group to be able to select the accout.
    pub require_role: Option<String>,
}

/// LDAP authentication settings.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct AuthConfigLdap {
    pub name: String,
    pub icon_url: String,
    pub ldap_url: String,
    pub dn_resolution: LdapDnResolution,
    pub tls: TlsConfig,
    #[serde(default)]
    pub base_dn: Option<String>,
    #[serde(default)]
    pub user_object_class: Option<String>,
    #[serde(default)]
    pub service_account: Option<LdapServiceAccount>,
    #[serde(default)]
    pub authorization: Option<LdapAuthorizationConfig>,
}

/// Signed JWT authentication settings.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct AuthConfigSignedJwt {
    pub valid_issuers: HashSet<String>,
    pub signing_keys: JwtSigningKeys,
}

/// Counts of objects of various types in an account.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct ObjectCounts {
    pub groups: u64,
    pub apps: u64,
    pub users: u64,
    pub plugins: u64,
    pub sobjects: u64,
}

/// CA settings.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum CaConfig {
    CaSet(CaSet),
    Pinned(Vec<Blob>),
}

/// Predefined CA sets.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum CaSet {
    GlobalRoots,
}

/// TLS settings.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", tag = "mode")]
pub enum TlsConfig {
    Disabled,
    Opportunistic,
    Required {
        validate_hostname: bool,
        ca: CaConfig,
    },
}

/// Account approval policy.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AccountApprovalPolicy {
    pub policy: ApprovalPolicy,
    pub manage_groups: bool,
}

/// Syslog facility.
#[derive(Copy, Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub enum SyslogFacility {
    User,
    Local0,
    Local1,
    Local2,
    Local3,
    Local4,
    Local5,
    Local6,
    Local7,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub acct_id: Uuid,
    #[serde(default)]
    pub approval_policy: Option<AccountApprovalPolicy>,
    #[serde(default)]
    pub approval_request_expiry: Option<u64>,
    #[serde(default)]
    pub auth_config: Option<AuthConfig>,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub created_at: Option<Time>,
    #[serde(default)]
    pub custom_logo: Option<Blob>,
    #[serde(default)]
    pub custom_metadata: Option<HashMap<String, String>>,
    #[serde(default)]
    pub description: Option<String>,
    pub enabled: bool,
    #[serde(default)]
    pub initial_purchase_at: Option<Time>,
    pub logging_configs: HashMap<Uuid, LoggingConfig>,
    #[serde(default)]
    pub max_app: Option<u32>,
    #[serde(default)]
    pub max_group: Option<u32>,
    #[serde(default)]
    pub max_operation: Option<u64>,
    #[serde(default)]
    pub max_plugin: Option<u32>,
    #[serde(default)]
    pub max_sobj: Option<u32>,
    #[serde(default)]
    pub max_user: Option<u32>,
    pub name: String,
    #[serde(default)]
    pub notification_pref: Option<NotificationPref>,
    #[serde(default)]
    pub organization: Option<String>,
    #[serde(default)]
    pub parent_acct_id: Option<Uuid>,
    #[serde(default)]
    pub pending_subscription_change_request: Option<SubscriptionChangeRequest>,
    #[serde(default)]
    pub phone: Option<String>,
    #[serde(default)]
    pub plugin_enabled: Option<bool>,
    pub subscription: SubscriptionType,
    #[serde(default)]
    pub totals: Option<ObjectCounts>,
    #[serde(default)]
    pub trial_expires_at: Option<Time>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct AccountRequest {
    #[serde(default)]
    pub add_ldap: Option<Vec<AuthConfigLdap>>,
    #[serde(default)]
    pub add_logging_configs: Option<Vec<LoggingConfigRequest>>,
    #[serde(default)]
    pub approval_policy: Option<AccountApprovalPolicy>,
    #[serde(default)]
    pub approval_request_expiry: Option<u64>,
    #[serde(default)]
    pub auth_config: Option<AuthConfig>,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub custom_logo: Option<Blob>,
    #[serde(default)]
    pub custom_metadata: Option<HashMap<String, String>>,
    #[serde(default)]
    pub del_ldap: Option<HashSet<Uuid>>,
    #[serde(default)]
    pub del_logging_configs: Option<HashSet<Uuid>>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub mod_ldap: Option<HashMap<Uuid, AuthConfigLdap>>,
    #[serde(default)]
    pub mod_logging_configs: Option<HashMap<Uuid, LoggingConfigRequest>>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub notification_pref: Option<NotificationPref>,
    #[serde(default)]
    pub organization: Option<String>,
    #[serde(default)]
    pub parent_acct_id: Option<Uuid>,
    #[serde(default)]
    pub pending_subscription_change_request: Option<SubscriptionChangeRequest>,
    #[serde(default)]
    pub phone: Option<String>,
    #[serde(default)]
    pub plugin_enabled: Option<bool>,
    #[serde(default)]
    pub subscription: Option<SubscriptionType>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GetAccountParams {
    pub with_totals: bool,
}

impl UrlEncode for GetAccountParams {
    fn url_encode(&self, m: &mut HashMap<&'static str, String>) {
        m.insert("with_totals", self.with_totals.to_string());
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct CountParams {
    pub range_from: Option<u64>,
    pub range_to: Option<u64>,
}

impl UrlEncode for CountParams {
    fn url_encode(&self, m: &mut HashMap<&'static str, String>) {
        if let Some(ref v) = self.range_from {
            m.insert("range_from", v.to_string());
        }
        if let Some(ref v) = self.range_to {
            m.insert("range_to", v.to_string());
        }
    }
}

/// Splunk logging configuration.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct SplunkLoggingConfig {
    pub enabled: bool,
    pub host: String,
    pub port: u16,
    pub index: String,
    pub tls: TlsConfig,
}

/// Stackdriver logging configuration.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct StackdriverLoggingConfig {
    pub enabled: bool,
    /// The log ID that will recieve the log items (see https://cloud.google.com/logging/docs/reference/v2/rest/v2/LogEntry).
    pub log_id: String,
    pub service_account_key: GoogleServiceAccountKey,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct StackdriverLoggingConfigRequest {
    pub enabled: Option<bool>,
    /// The log ID that will recieve the log items (see https://cloud.google.com/logging/docs/reference/v2/rest/v2/LogEntry).
    pub log_id: Option<String>,
    pub service_account_key: Option<GoogleServiceAccountKey>,
}

/// A Google service account key object. See https://cloud.google.com/video-intelligence/docs/common/auth.
#[derive(Default, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct GoogleServiceAccountKey {
    #[serde(rename = "type")]
    pub type_: String,
    pub project_id: String,
    pub private_key_id: String,
    #[serde(default)]
    pub private_key: Option<String>,
    pub client_email: String,
}

#[derive(Default, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct SplunkLoggingConfigRequest {
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub host: Option<String>,
    #[serde(default)]
    pub port: Option<u16>,
    /// The Splunk index that will receive log items.
    #[serde(default)]
    pub index: Option<String>,
    /// The Splunk authentication token.
    #[serde(default)]
    pub token: Option<String>,
    #[serde(default)]
    pub tls: Option<TlsConfig>,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct SyslogLoggingConfig {
    pub enabled: bool,
    pub host: String,
    pub port: u16,
    pub tls: TlsConfig,
    pub facility: SyslogFacility,
}

#[derive(Default, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct SyslogLoggingConfigRequest {
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub host: Option<String>,
    #[serde(default)]
    pub port: Option<u16>,
    #[serde(default)]
    pub tls: Option<TlsConfig>,
    #[serde(default)]
    pub facility: Option<SyslogFacility>,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum LoggingConfig {
    Splunk(SplunkLoggingConfig),
    Stackdriver(StackdriverLoggingConfig),
    Syslog(SyslogLoggingConfig),
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum LoggingConfigRequest {
    Splunk(SplunkLoggingConfigRequest),
    Stackdriver(StackdriverLoggingConfigRequest),
    Syslog(SyslogLoggingConfigRequest),
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct GetUsageResponse {
    pub num_operations: u64,
}

/// Account authentication settings.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct AuthConfig {
    #[serde(default)]
    pub password: Option<AuthConfigPassword>,
    #[serde(default)]
    pub saml: Option<String>,
    #[serde(default)]
    pub oauth: Option<AuthConfigOauth>,
    #[serde(default)]
    pub ldap: HashMap<Uuid, AuthConfigLdap>,
    #[serde(default)]
    pub signed_jwt: Option<AuthConfigSignedJwt>,
}

pub struct OperationListAccounts;
#[allow(unused)]
impl Operation for OperationListAccounts {
    type PathParams = ();
    type QueryParams = GetAccountParams;
    type Body = ();
    type Output = Vec<Account>;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/accounts?{q}", q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

impl SdkmsClient {
    pub fn list_accounts(&self, query_params: Option<&GetAccountParams>) -> Result<Vec<Account>> {
        self.execute::<OperationListAccounts>(&(), (), query_params)
    }
}

pub struct OperationGetAccount;
#[allow(unused)]
impl Operation for OperationGetAccount {
    type PathParams = (Uuid,);
    type QueryParams = GetAccountParams;
    type Body = ();
    type Output = Account;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/accounts/{id}?{q}", id = p.0, q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

impl SdkmsClient {
    pub fn get_account(
        &self,
        id: &Uuid,
        query_params: Option<&GetAccountParams>,
    ) -> Result<Account> {
        self.execute::<OperationGetAccount>(&(), (id,), query_params)
    }
}

pub struct OperationCreateAccount;
#[allow(unused)]
impl Operation for OperationCreateAccount {
    type PathParams = ();
    type QueryParams = ();
    type Body = AccountRequest;
    type Output = Account;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/accounts")
    }
}

impl SdkmsClient {
    pub fn create_account(&self, req: &AccountRequest) -> Result<Account> {
        self.execute::<OperationCreateAccount>(req, (), None)
    }
    pub fn request_approval_to_create_account(
        &self,
        req: &AccountRequest,
        description: Option<String>,
    ) -> Result<PendingApproval<OperationCreateAccount>> {
        self.request_approval::<OperationCreateAccount>(req, (), None, description)
    }
}

pub struct OperationUpdateAccount;
#[allow(unused)]
impl Operation for OperationUpdateAccount {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = AccountRequest;
    type Output = Account;

    fn method() -> Method {
        Method::Patch
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/accounts/{id}", id = p.0)
    }
}

impl SdkmsClient {
    pub fn update_account(&self, id: &Uuid, req: &AccountRequest) -> Result<Account> {
        self.execute::<OperationUpdateAccount>(req, (id,), None)
    }
    pub fn request_approval_to_update_account(
        &self,
        id: &Uuid,
        req: &AccountRequest,
        description: Option<String>,
    ) -> Result<PendingApproval<OperationUpdateAccount>> {
        self.request_approval::<OperationUpdateAccount>(req, (id,), None, description)
    }
}

pub struct OperationDeleteAccount;
#[allow(unused)]
impl Operation for OperationDeleteAccount {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = ();

    fn method() -> Method {
        Method::Delete
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/accounts/{id}", id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

impl SdkmsClient {
    pub fn delete_account(&self, id: &Uuid) -> Result<()> {
        self.execute::<OperationDeleteAccount>(&(), (id,), None)
    }
}

pub struct OperationAccountUsage;
#[allow(unused)]
impl Operation for OperationAccountUsage {
    type PathParams = (Uuid,);
    type QueryParams = CountParams;
    type Body = ();
    type Output = GetUsageResponse;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/accounts/{id}/usage?{q}", id = p.0, q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

impl SdkmsClient {
    pub fn account_usage(
        &self,
        id: &Uuid,
        query_params: Option<&CountParams>,
    ) -> Result<GetUsageResponse> {
        self.execute::<OperationAccountUsage>(&(), (id,), query_params)
    }
}
