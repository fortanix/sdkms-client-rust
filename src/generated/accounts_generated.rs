/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub acct_id: Uuid,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_policy: Option<AccountApprovalPolicy>,
    /// Configurations for group-level or account-level approval requests.
    #[serde(flatten)]
    pub approval_request_settings: ApprovalRequestSettings,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_config: Option<AuthConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_configurations: Option<ClientConfigurations>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Time>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cryptographic_policy: Option<CryptographicPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_logo: Option<Blob>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_metadata: Option<HashMap<String,String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_metadata_attributes: Option<HashMap<String,CustomAttributeSearchMetadata>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled_at: Option<Time>,
    pub enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial_purchase_at: Option<Time>,
    /// Enable the customer to configure when to receive alerts through SIEM tools ahead of key deactivation time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_expiry_alert_config: Option<KeyExpiryAlertConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_history_policy: Option<KeyHistoryPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_metadata_policy: Option<KeyMetadataPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log_bad_requests: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log_retention_days: Option<u64>,
    pub logging_configs: HashMap<Uuid,LoggingConfig>,
    /// Enable the user to opt out from the current behaviour of key being marked as disabled at time of deactivation.
    pub mark_key_disable_when_deactivated: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_app: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_group: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_operation: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_plugin: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_sobj: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_user: Option<u32>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_pref: Option<NotificationPref>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// Indicates the original purpose of the account when it was first created.
    pub original_purpose: AccountPurposeType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_acct_id: Option<Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending_subscription_change_request: Option<SubscriptionChangeRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Plugin code signing policy allows account administrators to control the plugins that can
    /// be added to the account. If a code signing policy is set, all requests to create new
    /// plugins or update existing plugins (if updating the code) would need to provide a
    /// valid signature.
    /// 
    /// NOTE: if the DSM cluster is running in FIPS mode, code signing is required for plugins.
    /// Therefore, if a plugin code signing policy is not set for an account, no plugins can be
    /// added in that account if the DSM cluster is running in FIPS mode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugin_code_signing_policy: Option<PluginCodeSigningPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugin_enabled: Option<bool>,
    /// The purpose of the account. Unless the account is meant for backup purposes (like disaster recovery), the account is a standard account, which is the default value. Additionally, on DSM SaaS, all accounts are standard accounts. Replication accounts are only available for onprem clusters.
    /// 
    /// A standard account cannot be changed to a replication account. A replication account can transition into a standard account, but doing so will sever the replication relationship between the source and destination accounts, and hence the two accounts are allowed to "diverge." Additionally, replication accounts are, for all practical purposes, read-only; in order to make one fully writeable, the account must first be converted to a standard account.
    /// 
    /// When creating or updating a replication account, the only fields allowed in the AccountRequest are the following:
    /// - this field itself, `purpose`
    /// - `enabled`
    /// - `name`
    /// - `auth_config`, plus `add_ldap`, `mod_ldap`, and `del_ldap`
    /// - `log_bad_requests`, `log_retention_days`, plus `add_logging_configs`, `mod_logging_configs`, and `del_logging_configs`
    /// The replication process would preserve most of the other fields from the source account.
    /// 
    /// For a given source account, a destination cluster can have at most one account that is either currently replicating or has previously replicated the source account. This means that if a customer wants to "start afresh" with a new replication account, simply converting their current account to a standard account does not help; the account needs to be deleted outright.
    /// 
    /// Note that this field is independent of the account's subscription, which controls the _features_ available for the account.
    pub purpose: AccountPurpose,
    pub subscription: Subscription,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub totals: Option<ObjectCounts>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trial_expires_at: Option<Time>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_cse_config: Option<WorkspaceCseConfig>
}

/// Account approval policy.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AccountApprovalPolicy {
    pub policy: QuorumPolicy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_groups: Option<bool>,
    /// When this is true, changes to the account authentication methods require approval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_authentication_methods: Option<bool>,
    /// When this is true, changes to the account cryptographic policy requires approval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_cryptographic_policy: Option<bool>,
    /// When this is true, changes to logging configuration require approval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_logging_config: Option<bool>,
    /// When set to true, updating custom roles would require approval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_custom_role_updates: Option<bool>
}

/// Describes the purpose of the account.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(tag = "$type")]
pub enum AccountPurpose {
    /// An ordinary account.
    Standard,
    /// An account that replicates another account (e.g., for disaster
    /// recovery purposes). Replication settings are contained here.
    AccountReplication (
        AccountReplicationConfiguration
    )
}

/// The purpose of the account (minus any configuration-related details).
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(tag = "$type")]
pub enum AccountPurposeType {
    /// An ordinary account.
    Standard,
    /// An account that replicates another account.
    AccountReplication {

    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct AccountReplicationConfiguration {
    /// Settings for how DSM should connect to the source account to be replicated.
    pub connection_settings: AccountReplicationConnection,
    /// Settings for how DSM should replicate objects from the source account, once a connection has
    /// been established.
    pub scan_settings: AccountReplicationScanSettings
}

/// Settings for how a replication account should connect to a source cluster. This type does not
/// handle configuration of a source-side admin app used in the replication process; such setup is
/// handled by separate endpoints.
#[derive(Debug, Hash, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct AccountReplicationConnection {
    /// The URL of the DSM cluster containing the account to back up. Only HTTPS is supported.
    pub url: Url,
    /// The ID of the currently-active replication credentials used to fetch objects from the source
    /// account. In Create requests, this field should not be specified (since credential creation is
    /// done via a separate endpoint), and in Update requests, this field can be omitted if no change
    /// is desired for the field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_replication_credential: Option<ReplicationCredentialId>
}

/// Settings for how DSM should go about replicating objects from the source account once a connection
/// has been established.
///
/// Today, account replication is performed using "basic" replication, which exports key material in
/// the clear (over a TLS connection).
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct AccountReplicationScanSettings {
    /// Settings for configuring how DSM periodically fetches information from the source. Today, the
    /// only configurable setting is the frequency of scans.
    pub auto_scan: AutoScanSettings
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct AccountRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_ldap: Option<Vec<AuthConfigLdap>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_logging_configs: Option<Vec<LoggingConfigRequest>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_policy: Option<AccountApprovalPolicy>,
    /// Configurations for group-level or account-level approval requests.
    #[serde(default, flatten, skip_serializing_if = "Option::is_none")]
    pub approval_request_settings: Option<ApprovalRequestSettingsRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_config: Option<AuthConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_configurations: Option<ClientConfigurationsRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cryptographic_policy: Option<Removable<CryptographicPolicy>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_logo: Option<Blob>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_metadata: Option<HashMap<String,String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_metadata_attributes: Option<HashMap<String,CustomAttributeSearchMetadata>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub del_ldap: Option<HashSet<Uuid>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub del_logging_configs: Option<HashSet<Uuid>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Enable the customer to configure when to receive alerts through SIEM tools ahead of key deactivation time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_expiry_alert_config: Option<KeyExpiryAlertConfigRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_history_policy: Option<Removable<KeyHistoryPolicy>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_metadata_policy: Option<Removable<KeyMetadataPolicy>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log_bad_requests: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log_retention_days: Option<u64>,
    /// Enable the user to opt out from the current behaviour of key being marked as disabled at time of deactivation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mark_key_disable_when_deactivated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mod_ldap: Option<HashMap<Uuid,AuthConfigLdap>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mod_logging_configs: Option<HashMap<Uuid,LoggingConfigRequest>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_pref: Option<NotificationPref>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_acct_id: Option<Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending_subscription_change_request: Option<SubscriptionChangeRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Plugin code signing policy allows account administrators to control the plugins that can
    /// be added to the account. If a code signing policy is set, all requests to create new
    /// plugins or update existing plugins (if updating the code) would need to provide a
    /// valid signature.
    /// 
    /// NOTE: if the DSM cluster is running in FIPS mode, code signing is required for plugins.
    /// Therefore, if a plugin code signing policy is not set for an account, no plugins can be
    /// added in that account if the DSM cluster is running in FIPS mode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugin_code_signing_policy: Option<Removable<PluginCodeSigningPolicy>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugin_enabled: Option<bool>,
    /// The purpose of the account. Unless the account is meant for backup purposes (like disaster recovery), the account is a standard account, which is the default value. Additionally, on DSM SaaS, all accounts are standard accounts. Replication accounts are only available for onprem clusters.
    /// 
    /// A standard account cannot be changed to a replication account. A replication account can transition into a standard account, but doing so will sever the replication relationship between the source and destination accounts, and hence the two accounts are allowed to "diverge." Additionally, replication accounts are, for all practical purposes, read-only; in order to make one fully writeable, the account must first be converted to a standard account.
    /// 
    /// When creating or updating a replication account, the only fields allowed in the AccountRequest are the following:
    /// - this field itself, `purpose`
    /// - `enabled`
    /// - `name`
    /// - `auth_config`, plus `add_ldap`, `mod_ldap`, and `del_ldap`
    /// - `log_bad_requests`, `log_retention_days`, plus `add_logging_configs`, `mod_logging_configs`, and `del_logging_configs`
    /// The replication process would preserve most of the other fields from the source account.
    /// 
    /// For a given source account, a destination cluster can have at most one account that is either currently replicating or has previously replicated the source account. This means that if a customer wants to "start afresh" with a new replication account, simply converting their current account to a standard account does not help; the account needs to be deleted outright.
    /// 
    /// Note that this field is independent of the account's subscription, which controls the _features_ available for the account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<AccountPurpose>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Subscription>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_cse_config: Option<Removable<WorkspaceCseConfig>>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AccountSort {
    ByAcctId {
        order: Order
    }
}

impl UrlEncode for AccountSort {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        match *self {
            AccountSort::ByAcctId{ ref order } => {
                m.insert("sort_by".to_string(), format!("acct_id:{}", order));
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug, Default, Serialize, Deserialize, Clone)]
pub struct AppCreditsUsage {
    pub generic: u32,
    pub tokenization: u32,
    pub tep: u32,
    pub accelerator: u32,
    pub secrets_management: u32,
    pub aws_cloud_accounts: u32,
    pub azure_cloud_accounts: u32
}

/// Settings that apply to quorum approval requests.
#[derive(Debug, Eq, PartialEq, Default, Serialize, Deserialize, Clone)]
pub struct ApprovalRequestSettings {
    /// The number of seconds after which an approval request expires. If not
    /// specified, the cluster-wide setting will be used (30 days by default).
    ///
    /// Upon creation, an approval request's expiry date is (time of creation +
    /// expiry period). However, when the request is approved by all its approvers,
    /// its expiry date will be changed to (time of approval + expiry period).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_request_expiry: Option<u64>,
    /// Whether or not expired approval requests should be kept. (Obviously, any
    /// pending requests that have expired are no longer actionable!)
    ///
    /// This is only applicable for onprem clusters; the field is ignored in SaaS
    /// environments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_expired_requests: Option<bool>,
    /// Whether or not expiry of pending approval requests should be audit logged.
    ///
    /// This is only applicable for onprem clusters; the field is ignored in SaaS
    /// environments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_expired_pending_requests: Option<bool>,
    /// Whether or not the requester's access should be checked again when they
    /// request to see the operation results for an approved quorum request with
    /// sensitive data in the output. Sensitive data includes secret values such
    /// as API keys, decrypted plaintext, exported key material etc. Note that
    /// if the result is not deemed sensitive this setting does not apply, e.g.
    /// approval request to sign a message (signatures are not deemed secret) or
    /// encrypt data (ciphertext is not deemed secret). Here is the list of all
    /// operations that are deemed sensitive (this list may be expanded in the
    /// future):
    ///
    /// - Get App Credential: `GET /sys/v1/apps/${app_id}/credential`
    /// - Decrypt:
    ///   - Legacy version: `POST /crypto/v1/keys/${key_id}/decrypt`
    ///   - New version: `POST /crypto/v1/decrypt`
    /// - Export Object Value:
    ///   - Legacy version: `GET /crypto/v1/keys/${key_id}/export`
    ///   - New version: `POST /crypto/v1/keys/export`
    /// - Batch: `POST /batch/v1` if any of the operations in the batch input is
    ///   sensitive.
    ///
    /// This setting is introduced for backwards compatibility so that existing
    /// approval request workflows are not broken. For new use cases, it is
    /// recommended to leave this setting enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_access_for_sensitive_operation_results: Option<bool>
}

/// A request struct for modifying settings that apply to quorum approval requests.
#[derive(Debug, Eq, PartialEq, Default, Serialize, Deserialize, Clone)]
pub struct ApprovalRequestSettingsRequest {
    /// The number of seconds after which an approval request expires. Changing this
    /// setting will not change the expiry of existing approval requests, but it may
    /// still affect the "updated" expiry period assigned to existing requests upon
    /// their approval (see below for details).
    ///
    /// Upon creation, an approval request's expiry date is (time of creation +
    /// expiry period). However, when the request is approved by all its approvers,
    /// its expiry date will be changed to (time of approval + expiry period).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_request_expiry: Option<u64>,
    /// Whether or not expired approval requests should be kept. (Obviously, any
    /// pending requests that have expired are no longer actionable!)
    ///
    /// This is only applicable for onprem clusters; the field is ignored in SaaS
    /// environments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_expired_requests: Option<bool>,
    /// Whether or not expiry of pending approval requests should be audit logged.
    /// Changing this setting will not retroactively apply to existing expired
    /// approval requests.
    ///
    /// This is only applicable for onprem clusters; the field is ignored in SaaS
    /// environments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_expired_pending_requests: Option<bool>,
    /// Whether or not the requester's access should be checked again when they
    /// request to see the operation results for an approved quorum request with
    /// sensitive data in the output. Sensitive data includes secret values such
    /// as API keys, decrypted plaintext, exported key material etc. Note that
    /// if the result is not deemed sensitive this setting does not apply, e.g.
    /// approval request to sign a message (signatures are not deemed secret) or
    /// encrypt data (ciphertext is not deemed secret). Here is the list of all
    /// operations that are deemed sensitive (this list may be expanded in the
    /// future):
    ///
    /// - Get App Credential: `GET /sys/v1/apps/${app_id}/credential`
    /// - Decrypt:
    ///   - Legacy version: `POST /crypto/v1/keys/${key_id}/decrypt`
    ///   - New version: `POST /crypto/v1/decrypt`
    /// - Export Object Value:
    ///   - Legacy version: `GET /crypto/v1/keys/${key_id}/export`
    ///   - New version: `POST /crypto/v1/keys/export`
    /// - Batch: `POST /batch/v1` if any of the operations in the batch input is
    ///   sensitive.
    ///
    /// This setting is introduced for backwards compatibility so that existing
    /// approval request workflows are not broken. For new use cases, it is
    /// recommended to leave this setting enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_access_for_sensitive_operation_results: Option<bool>
}

/// Account authentication settings.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct AuthConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<AuthConfigPassword>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub saml: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth: Option<AuthConfigOauth>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ldap: Option<HashMap<Uuid,AuthConfigLdap>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signed_jwt: Option<AuthConfigSignedJwt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vcd: Option<AuthConfigVcd>
}

/// OAuth single sign-on authentication settings.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct AuthConfigOauth {
    pub idp_name: String,
    pub idp_icon_url: String,
    pub idp_authorization_endpoint: String,
    pub idp_token_endpoint: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idp_userinfo_endpoint: Option<String>,
    pub idp_requires_basic_auth: bool,
    pub tls: TlsConfig,
    pub client_id: String,
    pub client_secret: ZeroizedString,
    /// Parameters to set when calling `idp_authorization_endpoint`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_params: Option<OauthAuthenticationParameters>
}

/// Password authentication settings.
#[derive(PartialEq, Eq, Debug, Default, Serialize, Deserialize, Clone)]
pub struct AuthConfigPassword {
    pub require_2fa: bool,
    pub administrators_only: bool
}

/// Signed JWT authentication settings.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct AuthConfigSignedJwt {
    pub valid_issuers: HashSet<String>,
    pub signing_keys: SigningKeys
}

/// Vcd single sign-on authentication settings.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct AuthConfigVcd {
    pub idp_name: String,
    pub idp_authorization_endpoint: String,
    pub org: String,
    pub tls: TlsConfig
}

/// Details about a certificate-based admin app credential used for account replication.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct CertificateReplicationCredential {
    /// The ID of the source-side admin app that uses this credential.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<Uuid>,
    /// The ID assigned to the credential.
    pub credential_id: ReplicationCredentialId,
    /// The certificate chain associated with the credential. This is a list of DER-encoded
    /// certificates, starting from the leaf certificate, and can consist of a single certificate if
    /// no intermediate certificates are necessary when authenticating with the source cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<Vec<ZeroizedBlob>>
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct CountParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_from: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_to: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_usage: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saas_full_usage: Option<bool>
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

/// A request to create a new admin app credential for account replication
/// purposes. Note that the result is not immediately usable; further steps
/// are needed in order to configure this and set it as the account's active
/// credential.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "$type")]
pub enum CreateReplicationCredentialRequest {
    /// Create a private key as part of a client certificate (or trusted CA)
    /// admin app credential for account replication. A new self-signed cert
    /// needs to be requested afterwards. (In the future, DSM will also allow
    /// a CSR to be requested instead.)
    ///
    /// The exact details of the private key (e.g., object type, key size)
    /// are an implementation detail, and may change between DSM versions.
    Certificate {

    }
}

/// Custom subscription type
#[derive(Eq, PartialEq, Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct CustomSubscriptionType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_plugin: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_app: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_hsmg: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_operation: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokenization_operation: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_transient_ops: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<SubscriptionFeatures>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_ons: Option<HashMap<String,String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub soft_ops_per_second_limit: Option<u32>
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct DaysAhead {
    pub days: u16
}

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct FreemiumSubscriptionType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_app: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_hsmg: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_operation: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokenization_operation: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_plugin: Option<u32>
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GetAccountParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_totals: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    #[serde(flatten)]
    pub sort_by: AccountSort
}

impl UrlEncode for GetAccountParams {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        if let Some(ref v) = self.with_totals {
            m.insert("with_totals".to_string(), v.to_string());
        }
        if let Some(ref v) = self.previous_id {
            m.insert("previous_id".to_string(), v.to_string());
        }
        if let Some(ref v) = self.limit {
            m.insert("limit".to_string(), v.to_string());
        }
        self.sort_by.url_encode(m);
    }
}

#[derive(Debug, Eq, PartialEq, Default, Serialize, Deserialize, Clone)]
pub struct GetUsageResponse {
    pub num_operations: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption_operations: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decryption_operations: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sign_operations: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verify_operations: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tokenization_operations: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detokenization_operations: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets_operations: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugin_invoke_operations: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apps: Option<AppCreditsUsage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugin: Option<u32>,
    /// The total number of sobjects in the account, or
    /// an approximation thereof. This field is present if
    /// the `saas_full_usage` query parameter is specified
    /// when retrieving account usage statistics.
    ///
    /// Note that all sobjects in the account are counted,
    /// regardless of whether the user has access to them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sobjects: Option<u64>,
    /// The accuracy of the `sobjects` count (whether it
    /// is an exact count or an approximate count).
    ///
    /// If the total number of sobjects in the account is
    /// less than 5000, DSM will return an exact number.
    /// Additionally, if DSM estimates the total number of
    /// sobjects to be less than 10000, it will still attempt
    /// to return an exact count. Otherwise, DSM will return
    /// an approximation.
    ///
    /// These rules are subject to change in the future.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sobjects_accuracy: Option<CountAccuracy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hsm_gateway: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation_top_app: Option<HashMap<String,u64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation_top_sobject: Option<HashMap<String,u64>>
}

/// A Google service account key object. See https://cloud.google.com/video-intelligence/docs/common/auth.
#[derive(Default, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct GoogleServiceAccountKey {
    #[serde(rename = "type")]
    pub type_: String,
    pub project_id: String,
    pub private_key_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private_key: Option<ZeroizedString>,
    pub client_email: String
}

#[derive(PartialEq, Eq, Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeyExpiryAlertConfig {
    pub triggers: HashMap<Uuid,KeyExpiryAlertTrigger>,
    pub siem_tool_configs: HashMap<Uuid,KeyExpiryAlertSiemToolConfig>
}

#[derive(PartialEq, Eq, Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeyExpiryAlertConfigRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_triggers: Option<Vec<KeyExpiryAlertTrigger>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mod_triggers: Option<HashMap<Uuid,KeyExpiryAlertTrigger>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub del_triggers: Option<HashSet<Uuid>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_siem_tool_configs: Option<Vec<KeyExpiryAlertSiemToolConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mod_siem_tool_configs: Option<HashMap<Uuid,KeyExpiryAlertSiemToolConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub del_siem_tool_configs: Option<HashSet<Uuid>>
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct KeyExpiryAlertSiemToolConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_key_info_per_alert: Option<u16>,
    pub config: LoggingConfig
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "$type")]
pub enum KeyExpiryAlertTrigger {
    DaysAhead (
        DaysAhead
    )
}

/// Response body for a GET call to retrieve all replication credentials.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListReplicationCredentialsResponse {
    /// The list of replication credentials.
    pub items: Vec<ReplicationCredential>
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum LoggingConfig {
    Splunk (
        SplunkLoggingConfig
    ),
    Stackdriver (
        StackdriverLoggingConfig
    ),
    Syslog (
        SyslogLoggingConfig
    )
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum LoggingConfigRequest {
    Splunk (
        SplunkLoggingConfigRequest
    ),
    Stackdriver (
        StackdriverLoggingConfigRequest
    ),
    Syslog (
        SyslogLoggingConfigRequest
    )
}

/// Notification preferences.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub enum NotificationPref {
    None,
    Email,
    Phone,
    Both
}

/// Counts of objects of various types in an account.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct ObjectCounts {
    pub groups: u64,
    pub apps: u64,
    pub users: u64,
    pub plugins: u64,
    /// The total number of sobjects in the account, or
    /// an approximation thereof.
    ///
    /// Note that all sobjects in the account are counted,
    /// regardless of whether the user has access to them.
    pub sobjects: u64,
    /// The accuracy of the `sobjects` count (whether it
    /// is an exact count or an approximate count).
    ///
    /// If the total number of sobjects in the account is
    /// less than 5000, DSM will return an exact number.
    /// Additionally, if DSM estimates the total number of
    /// sobjects to be less than 10000, it will still attempt
    /// to return an exact count. Otherwise, DSM will return
    /// an approximation.
    ///
    /// These rules are subject to change in the future.
    pub sobjects_accuracy: CountAccuracy,
    pub child_accounts: u64
}

/// A summary of the latest scans for a replication account.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct RecentScanSummary {
    /// Information about any currently in-progress scan.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_progress: Option<ReplicationScan>,
    /// Information about the last finished scan on the account, whether
    /// successful or not.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_completed: Option<ReplicationScan>,
    /// Information about the last finished scan on the account that finished
    /// successfully.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_successful: Option<ReplicationScan>
}

/// Details about the admin app credential used to replicate objects from the source account.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(tag = "$type")]
pub enum ReplicationCredential {
    /// A client certificate (or trusted CA) app credential. This is the only available option today.
    Certificate (
        CertificateReplicationCredential
    )
}

/// The ID of a replication credential.
pub type ReplicationCredentialId = String;

/// A request to generate a new self-signed certificate for a
/// replication credential.
///
/// For now, the default attributes will include the following:
/// - Version 3 certificate
/// - Subject:
///   - Common name is "<acct-id> replication credential <credential-id>",
///     where <acct-id> and <credential-id> are replaced with the actual IDs
///   - No other attributes in the subject
/// - No expiry (represented by 99991231235959Z as per RFC 5280)
/// - There will not be a basic constraints extension
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ReplicationCredentialSelfSignedCertRequest {

}

/// Response from the endpoint to generate a new self-signed cert for a replication credential.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReplicationCredentialSelfSignedCertResponse {
    /// The self-signed certificate generated by DSM.
    pub certificate: ZeroizedBlob
}

/// Information about a scan performed under a replication account.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReplicationScan {
    /// The time the scan began.
    pub started_at: Time,
    /// The time the scan finished.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<Time>,
    /// Any error message returned by the scan. If this field is empty, the
    /// scan is either ongoing, or returned successfully.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>
}

/// Reseller subscription type
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct ResellerSubscriptionType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_plugin: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_operation: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tenant: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tenant_plugin: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tenant_operation: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<SubscriptionFeatures>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_ons: Option<HashMap<String,String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_features: Option<SubscriptionFeatures>
}

/// Splunk logging configuration.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct SplunkLoggingConfig {
    pub enabled: bool,
    pub host: String,
    pub port: u16,
    pub index: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<ZeroizedString>,
    pub tls: TlsConfig
}

#[derive(Default, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct SplunkLoggingConfigRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
    /// The Splunk index that will receive log items.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<String>,
    /// The Splunk authentication token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<ZeroizedString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<TlsConfig>
}

/// Stackdriver logging configuration.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct StackdriverLoggingConfig {
    pub enabled: bool,
    /// The log ID that will receive the log items (see https://cloud.google.com/logging/docs/reference/v2/rest/v2/LogEntry).
    pub log_id: String,
    pub service_account_key: GoogleServiceAccountKey
}

#[derive(Default, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct StackdriverLoggingConfigRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The log ID that will receive the log items (see https://cloud.google.com/logging/docs/reference/v2/rest/v2/LogEntry).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_key: Option<GoogleServiceAccountKey>
}

#[derive(PartialEq, Eq, Debug, Default, Serialize, Deserialize, Clone)]
pub struct Subscription {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub experimental_features: Option<SubscriptionExperimentalFeatures>,
    #[serde(flatten)]
    pub subscription_type: SubscriptionType
}

/// A request to update subscription type.
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct SubscriptionChangeRequest {
    pub subscription: Subscription,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>
}

#[derive(Debug, Eq, PartialEq, Default, Serialize, Deserialize, Clone)]
pub struct SubscriptionExperimentalFeatures {

}

/// Features in subscription
pub use self::subscription_features::SubscriptionFeatures;
pub mod subscription_features {
    bitflags_set!{
        pub struct SubscriptionFeatures: u64 {
            const TOKENIZATION = 0x0000000000000001;
            const HMG = 0x0000000000000002;
            const AWSBYOK = 0x0000000000000004;
            const AZUREBYOK = 0x0000000000000008;
            const GCPBYOK = 0x0000000000000010;
            const GCPEKMCONTROLPLANE = 0x0000000000000020;
        }
    }
}

/// Type of subscription.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionType {
    Trial {
        #[serde(skip_serializing_if = "Option::is_none")]
        expires_at: Option<Time>
    },
    Standard {

    },
    Enterprise {

    },
    Custom (
        Box<CustomSubscriptionType>
    ),
    Freemium (
        Box<FreemiumSubscriptionType>
    ),
    OnPrem {

    },
    Reseller (
        Box<ResellerSubscriptionType>
    )
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
    Local7
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct SyslogLoggingConfig {
    pub enabled: bool,
    pub host: String,
    pub port: u16,
    pub tls: TlsConfig,
    pub facility: SyslogFacility
}

#[derive(Default, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct SyslogLoggingConfigRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<TlsConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub facility: Option<SyslogFacility>
}

/// A request to update a certificate-based replication credential.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateCertificateReplicationCredentialRequest {
    /// The app ID to associate with the credential. This should be the ID of
    /// a source-side admin app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<Uuid>,
    /// The certificate chain to associate with the credential. This is a
    /// list of DER-encoded certificates, starting from the leaf certificate,
    /// and may consist of a single certificate if no intermediate
    /// certificates are necessary when authenticating with the source
    /// cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<Vec<ZeroizedBlob>>
}

/// A request to update a replication credential (e.g., associating it with
/// an app ID).
///
/// Note that changing the credential from one type to another is disallowed;
/// users should create a new credential instead.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "$type")]
pub enum UpdateReplicationCredentialRequest {
    /// Request to update a certificate-based credential.
    Certificate (
        UpdateCertificateReplicationCredentialRequest
    )
}

/// Authentication method for Google Workspace CSE, `User` (default choice) requires each CSE user
/// to be registered as a DSM user, while `App` requires each CSE user to be represented by a DSM app.
///
/// Note:
/// For large organizations where lots of users use Google Workspace CSE but are not otherwise expected
/// to be able to access DSM, App authentication method could be easier to implement.
#[derive(Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Clone)]
pub enum WorkspaceCseAuthMethod {
    /// Each CSE user must be registered as a DSM user
    User,
    /// Each CSE user is represented by a DSM app and only needs access to cse specific endpoints.
    App
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
    pub valid_audiences: HashSet<String>
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
    /// An accounts method of authenticating users via the CSE integration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_method: Option<WorkspaceCseAuthMethod>
}

/// An identity provider trusted to authenticate users for Workspace CSE APIs
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct WorkspaceCseIdentityProvider {
    /// Identity provider's name
    pub name: String,
    /// The public key(s) used to validate the authentication tokens
    pub signing_keys: SigningKeys,
    /// Acceptable values for the `iss` (issuer) field used in authentication
    /// tokens
    pub valid_issuers: HashSet<String>,
    /// Acceptable values for the `aud` (audience) field used in authentication
    /// tokens
    pub valid_audiences: HashSet<String>
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
        format!("/sys/v1/accounts/{acct_id}/usage?{q}", acct_id = p.0, q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn account_usage(&self, acct_id: &Uuid, query_params: Option<&CountParams>) -> Result<GetUsageResponse> {
        self.execute::<OperationAccountUsage>(&(), (acct_id,), query_params)
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
        "/sys/v1/accounts".to_string()
    }
}

impl SdkmsClient {
    pub fn create_account(&self, req: &AccountRequest) -> Result<Account> {
        self.execute::<OperationCreateAccount>(req, (), None)
    }
    pub fn request_approval_to_create_account(
        &self, req: &AccountRequest,
        description: Option<String>) -> Result<PendingApproval<OperationCreateAccount>> {
        self.request_approval::<OperationCreateAccount>(req, (), None, description)
    }
}

pub struct OperationCreateReplicationCredential;
#[allow(unused)]
impl Operation for OperationCreateReplicationCredential {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = CreateReplicationCredentialRequest;
    type Output = ReplicationCredential;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/accounts/{acct_id}/replication/credentials", acct_id = p.0)
    }
}

impl SdkmsClient {
    pub fn create_replication_credential(&self, acct_id: &Uuid, req: &CreateReplicationCredentialRequest) -> Result<ReplicationCredential> {
        self.execute::<OperationCreateReplicationCredential>(req, (acct_id,), None)
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
        format!("/sys/v1/accounts/{acct_id}", acct_id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn delete_account(&self, acct_id: &Uuid) -> Result<()> {
        self.execute::<OperationDeleteAccount>(&(), (acct_id,), None)
    }
}

pub struct OperationDeleteReplicationCredential;
#[allow(unused)]
impl Operation for OperationDeleteReplicationCredential {
    type PathParams = (Uuid, Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = ();

    fn method() -> Method {
        Method::Delete
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/accounts/{acct_id}/replication/credentials/{credential_id}", acct_id = p.0, credential_id = p.1)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn delete_replication_credential(&self, acct_id: &Uuid, credential_id: &Uuid) -> Result<()> {
        self.execute::<OperationDeleteReplicationCredential>(&(), (acct_id, credential_id,), None)
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
        format!("/sys/v1/accounts/{acct_id}?{q}", acct_id = p.0, q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_account(&self, acct_id: &Uuid, query_params: Option<&GetAccountParams>) -> Result<Account> {
        self.execute::<OperationGetAccount>(&(), (acct_id,), query_params)
    }
}

pub struct OperationGetReplicationCredential;
#[allow(unused)]
impl Operation for OperationGetReplicationCredential {
    type PathParams = (Uuid, Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = ReplicationCredential;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/accounts/{acct_id}/replication/credentials/{credential_id}", acct_id = p.0, credential_id = p.1)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_replication_credential(&self, acct_id: &Uuid, credential_id: &Uuid) -> Result<ReplicationCredential> {
        self.execute::<OperationGetReplicationCredential>(&(), (acct_id, credential_id,), None)
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
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/accounts?{q}", q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn list_accounts(&self, query_params: Option<&GetAccountParams>) -> Result<Vec<Account>> {
        self.execute::<OperationListAccounts>(&(), (), query_params)
    }
}

pub struct OperationListReplicationCredentials;
#[allow(unused)]
impl Operation for OperationListReplicationCredentials {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = ListReplicationCredentialsResponse;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/accounts/{acct_id}/replication/credentials", acct_id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn list_replication_credentials(&self, acct_id: &Uuid) -> Result<ListReplicationCredentialsResponse> {
        self.execute::<OperationListReplicationCredentials>(&(), (acct_id,), None)
    }
}

pub struct OperationRecentReplicationScanSummary;
#[allow(unused)]
impl Operation for OperationRecentReplicationScanSummary {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = RecentScanSummary;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/accounts/{acct_id}/replication/recent_scan_summary", acct_id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn recent_replication_scan_summary(&self, acct_id: &Uuid) -> Result<RecentScanSummary> {
        self.execute::<OperationRecentReplicationScanSummary>(&(), (acct_id,), None)
    }
}

pub struct OperationReplicationCredentialSelfSignedCert;
#[allow(unused)]
impl Operation for OperationReplicationCredentialSelfSignedCert {
    type PathParams = (Uuid, Uuid,);
    type QueryParams = ();
    type Body = ReplicationCredentialSelfSignedCertRequest;
    type Output = ReplicationCredentialSelfSignedCertResponse;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/accounts/{acct_id}/replication/credentials/{credential_id}/self_sign", acct_id = p.0, credential_id = p.1)
    }
}

impl SdkmsClient {
    pub fn replication_credential_self_signed_cert(&self, acct_id: &Uuid, credential_id: &Uuid, req: &ReplicationCredentialSelfSignedCertRequest) -> Result<ReplicationCredentialSelfSignedCertResponse> {
        self.execute::<OperationReplicationCredentialSelfSignedCert>(req, (acct_id, credential_id,), None)
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
        format!("/sys/v1/accounts/{acct_id}", acct_id = p.0)
    }
}

impl SdkmsClient {
    pub fn update_account(&self, acct_id: &Uuid, req: &AccountRequest) -> Result<Account> {
        self.execute::<OperationUpdateAccount>(req, (acct_id,), None)
    }
    pub fn request_approval_to_update_account(
        &self, acct_id: &Uuid, req: &AccountRequest,
        description: Option<String>) -> Result<PendingApproval<OperationUpdateAccount>> {
        self.request_approval::<OperationUpdateAccount>(req, (acct_id,), None, description)
    }
}

pub struct OperationUpdateReplicationCredential;
#[allow(unused)]
impl Operation for OperationUpdateReplicationCredential {
    type PathParams = (Uuid, Uuid,);
    type QueryParams = ();
    type Body = UpdateReplicationCredentialRequest;
    type Output = ReplicationCredential;

    fn method() -> Method {
        Method::Patch
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/accounts/{acct_id}/replication/credentials/{credential_id}", acct_id = p.0, credential_id = p.1)
    }
}

impl SdkmsClient {
    pub fn update_replication_credential(&self, acct_id: &Uuid, credential_id: &Uuid, req: &UpdateReplicationCredentialRequest) -> Result<ReplicationCredential> {
        self.execute::<OperationUpdateReplicationCredential>(req, (acct_id, credential_id,), None)
    }
}

