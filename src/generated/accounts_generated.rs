/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

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
    pub client_configurations: Option<ClientConfigurations>,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub created_at: Option<Time>,
    #[serde(default)]
    pub cryptographic_policy: Option<CryptographicPolicy>,
    #[serde(default)]
    pub custom_logo: Option<Blob>,
    #[serde(default)]
    pub custom_metadata: Option<HashMap<String, String>>,
    #[serde(default)]
    pub custom_metadata_attributes: Option<HashMap<String, CustomAttributeSearchMetadata>>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub disabled_at: Option<Time>,
    pub enabled: bool,
    #[serde(default)]
    pub initial_purchase_at: Option<Time>,
    #[serde(default)]
    pub key_history_policy: Option<KeyHistoryPolicy>,
    #[serde(default)]
    pub key_metadata_policy: Option<KeyMetadataPolicy>,
    #[serde(default)]
    pub log_bad_requests: Option<bool>,
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
    pub subscription: Subscription,
    #[serde(default)]
    pub totals: Option<ObjectCounts>,
    #[serde(default)]
    pub trial_expires_at: Option<Time>,
    #[serde(default)]
    pub workspace_cse_config: Option<WorkspaceCseConfig>,
}

/// Account approval policy.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AccountApprovalPolicy {
    pub policy: QuorumPolicy,
    pub manage_groups: bool,
    /// When this is true, changes to the account authentication methods require approval.
    pub protect_authentication_methods: Option<bool>,
    /// When this is true, changes to the account cryptographic policy requires approval.
    pub protect_cryptographic_policy: Option<bool>,
    /// When this is true, changes to logging configuration require approval.
    pub protect_logging_config: Option<bool>,
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
    pub client_configurations: Option<ClientConfigurationsRequest>,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub cryptographic_policy: Option<Option<CryptographicPolicy>>,
    #[serde(default)]
    pub custom_logo: Option<Blob>,
    #[serde(default)]
    pub custom_metadata: Option<HashMap<String, String>>,
    #[serde(default)]
    pub custom_metadata_attributes: Option<HashMap<String, CustomAttributeSearchMetadata>>,
    #[serde(default)]
    pub del_ldap: Option<HashSet<Uuid>>,
    #[serde(default)]
    pub del_logging_configs: Option<HashSet<Uuid>>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub key_history_policy: Option<Option<KeyHistoryPolicy>>,
    #[serde(default)]
    pub key_metadata_policy: Option<Option<KeyMetadataPolicy>>,
    #[serde(default)]
    pub log_bad_requests: Option<bool>,
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
    pub subscription: Option<Subscription>,
    #[serde(default)]
    pub workspace_cse_config: Option<Option<WorkspaceCseConfig>>,
}

#[derive(PartialEq, Eq, Debug, Default, Serialize, Deserialize, Clone)]
pub struct AppCreditsUsage {
    pub generic: u32,
    pub tokenization: u32,
    pub tep: u32,
    pub accelerator: u32,
    pub secrets_management: u32,
    pub aws_cloud_accounts: u32,
    pub azure_cloud_accounts: u32,
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
    #[serde(default)]
    pub vcd: Option<AuthConfigVcd>,
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

/// Password authentication settings.
#[derive(PartialEq, Eq, Debug, Default, Serialize, Deserialize, Clone)]
pub struct AuthConfigPassword {
    pub require_2fa: bool,
    pub administrators_only: bool,
}

/// Signed JWT authentication settings.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct AuthConfigSignedJwt {
    pub valid_issuers: HashSet<String>,
    pub signing_keys: JwtSigningKeys,
}

/// Vcd single sign-on authentication settings.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct AuthConfigVcd {
    pub idp_name: String,
    pub idp_authorization_endpoint: String,
    pub org: String,
    pub tls: TlsConfig,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct CountParams {
    pub range_from: Option<u64>,
    pub range_to: Option<u64>,
    pub detailed_usage: Option<bool>,
    pub saas_full_usage: Option<bool>,
}

impl UrlEncode for CountParams {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        if let Some(ref v) = self.range_from {
            m.insert("range_from".to_string(), v.to_string());
        }
        if let Some(ref v) = self.range_to {
            m.insert("range_to".to_string(), v.to_string());
        }
        if let Some(ref v) = self.detailed_usage {
            m.insert("detailed_usage".to_string(), v.to_string());
        }
        if let Some(ref v) = self.saas_full_usage {
            m.insert("saas_full_usage".to_string(), v.to_string());
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Serialize, Deserialize, Clone)]
pub struct CustomAttributeSearchMetadata {
    pub suggest: bool,
}

/// Custom subscription type
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct CustomSubscriptionType {
    pub max_plugin: Option<u32>,
    pub max_app: Option<u32>,
    pub max_hsmg: Option<u32>,
    pub max_operation: Option<u64>,
    pub max_tokenization_operation: Option<u64>,
    pub count_transient_ops: Option<bool>,
    pub package_name: Option<String>,
    pub features: Option<SubscriptionFeatures>,
    pub add_ons: Option<HashMap<String, String>>,
}

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct FreemiumSubscriptionType {
    pub max_app: Option<u32>,
    pub max_hsmg: Option<u32>,
    pub max_operation: Option<u64>,
    pub max_tokenization_operation: Option<u64>,
    pub max_plugin: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GetAccountParams {
    pub with_totals: bool,
}

impl UrlEncode for GetAccountParams {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        m.insert("with_totals".to_string(), self.with_totals.to_string());
    }
}

#[derive(Debug, Eq, PartialEq, Default, Serialize, Deserialize, Clone)]
pub struct GetUsageResponse {
    pub num_operations: u64,
    #[serde(default)]
    pub encryption_operations: Option<u64>,
    #[serde(default)]
    pub decryption_operations: Option<u64>,
    #[serde(default)]
    pub sign_operations: Option<u64>,
    #[serde(default)]
    pub verify_operations: Option<u64>,
    #[serde(default)]
    pub tokenization_operations: Option<u64>,
    #[serde(default)]
    pub detokenization_operations: Option<u64>,
    #[serde(default)]
    pub secrets_operations: Option<u64>,
    #[serde(default)]
    pub plugin_invoke_operations: Option<u64>,
    #[serde(default)]
    pub apps: Option<AppCreditsUsage>,
    #[serde(default)]
    pub plugin: Option<u32>,
    #[serde(default)]
    pub sobjects: Option<u64>,
    #[serde(default)]
    pub hsm_gateway: Option<u32>,
    #[serde(default)]
    pub operation_top_app: Option<HashMap<String, u64>>,
    #[serde(default)]
    pub operation_top_sobject: Option<HashMap<String, u64>>,
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

/// Notification preferences.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub enum NotificationPref {
    None,
    Email,
    Phone,
    Both,
}

/// Counts of objects of various types in an account.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct ObjectCounts {
    pub groups: u64,
    pub apps: u64,
    pub users: u64,
    pub plugins: u64,
    pub sobjects: u64,
    pub child_accounts: u64,
}

/// Reseller subscription type
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct ResellerSubscriptionType {
    pub max_plugin: Option<u32>,
    pub max_operation: Option<u64>,
    pub max_tenant: Option<u32>,
    pub max_tenant_plugin: Option<u32>,
    pub max_tenant_operation: Option<u64>,
    pub package_name: Option<String>,
    pub features: Option<SubscriptionFeatures>,
    pub add_ons: Option<HashMap<String, String>>,
    pub tenant_features: Option<SubscriptionFeatures>,
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

#[derive(PartialEq, Eq, Debug, Default, Serialize, Deserialize, Clone)]
pub struct Subscription {
    #[serde(default)]
    pub memo: Option<String>,
    #[serde(flatten)]
    pub subscription_type: SubscriptionType,
}

/// A request to update subscription type.
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct SubscriptionChangeRequest {
    pub subscription: Subscription,
    #[serde(default)]
    pub contact: Option<String>,
    #[serde(default)]
    pub comment: Option<String>,
}

/// Features in subscription
pub use self::subscription_features::SubscriptionFeatures;
pub mod subscription_features {
    bitflags_set! {
        pub struct SubscriptionFeatures: u64 {
            const TOKENIZATION = 0x0000000000000001;
            const HMG = 0x0000000000000002;
            const AWSBYOK = 0x0000000000000004;
            const AZUREBYOK = 0x0000000000000008;
            const GCPBYOK = 0x0000000000000010;
        }
    }
}

/// Type of subscription.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionType {
    Trial { expires_at: Option<Time> },
    Standard {},
    Enterprise {},
    Custom(Box<CustomSubscriptionType>),
    Freemium(Box<FreemiumSubscriptionType>),
    OnPrem {},
    Reseller(Box<ResellerSubscriptionType>),
}

#[derive(Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Clone)]
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

/// These settings will allow the service to validate the Google-issued
/// authorization tokens used in Workspace CSE APIs.
///
/// For example, the specific settings for CSE Docs & Drive are:
/// - JWKS URL: https://www.googleapis.com/service_accounts/v1/jwk/gsuitecse-tokenissuer-drive@system.gserviceaccount.com
/// - Issuer: gsuitecse-tokenissuer-drive@system.gserviceaccount.com
/// - Audience: cse-authorization
///
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct WorkspaceCseAuthorizationProvider {
    /// Authorization provider's name
    pub name: String,
    /// A URL pointing to the JWKS endpoint
    pub jwks_url: String,
    /// Number of seconds that the service is allowed to cache the fetched keys
    pub cache_duration: u64,
    /// Acceptable values for the `iss` (issuer) field used in Google's
    /// authorization tokens
    pub valid_issuers: HashSet<String>,
    /// Acceptable values for the `aud` (audience) field used in Google's
    /// authorization tokens
    pub valid_audiences: HashSet<String>,
}

/// Workspace CSE API settings. Specifying these settings enables the CSE APIs
/// for the account.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct WorkspaceCseConfig {
    /// One or more Identity Providers (IdP) trusted to authenticate users.
    /// Note that we don't check if Single Sign-On (SSO) settings exist for
    /// each IdP listed here, but it is recommended to add these IdPs in SSO
    /// settings as well (usually as OAuth/OIDC providers).
    pub identity_providers: Vec<WorkspaceCseIdentityProvider>,
    /// One or more authorization providers used to validate authorization
    /// tokens. Different Workspace applications might require different
    /// authorization settings.
    pub authorization_providers: Vec<WorkspaceCseAuthorizationProvider>,
}

/// An identity provider trusted to authenticate users for Workspace CSE APIs
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct WorkspaceCseIdentityProvider {
    /// Identity provider's name
    pub name: String,
    /// The public key(s) used to validate the authentication tokens
    pub signing_keys: JwtSigningKeys,
    /// Acceptable values for the `iss` (issuer) field used in authentication
    /// tokens
    pub valid_issuers: HashSet<String>,
    /// Acceptable values for the `aud` (audience) field used in authentication
    /// tokens
    pub valid_audiences: HashSet<String>,
}

pub struct OperationAccountUsage;
#[allow(unused)]
impl Operation for OperationAccountUsage {
    type PathParams = (Uuid,);
    type QueryParams = CountParams;
    type Body = ();
    type Output = GetUsageResponse;

    fn method() -> Method {
        Method::GET
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

pub struct OperationCreateAccount;
#[allow(unused)]
impl Operation for OperationCreateAccount {
    type PathParams = ();
    type QueryParams = ();
    type Body = AccountRequest;
    type Output = Account;

    fn method() -> Method {
        Method::POST
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

pub struct OperationDeleteAccount;
#[allow(unused)]
impl Operation for OperationDeleteAccount {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = ();

    fn method() -> Method {
        Method::DELETE
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

pub struct OperationGetAccount;
#[allow(unused)]
impl Operation for OperationGetAccount {
    type PathParams = (Uuid,);
    type QueryParams = GetAccountParams;
    type Body = ();
    type Output = Account;

    fn method() -> Method {
        Method::GET
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

pub struct OperationListAccounts;
#[allow(unused)]
impl Operation for OperationListAccounts {
    type PathParams = ();
    type QueryParams = GetAccountParams;
    type Body = ();
    type Output = Vec<Account>;

    fn method() -> Method {
        Method::GET
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

pub struct OperationUpdateAccount;
#[allow(unused)]
impl Operation for OperationUpdateAccount {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = AccountRequest;
    type Output = Account;

    fn method() -> Method {
        Method::PATCH
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
