/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

pub use self::account_permissions::AccountPermissions;
pub mod account_permissions {
    bitflags_set!{
        #[derive(Default)]
        pub struct AccountPermissions: u64 {
            ///  Permission to manage logging integrations, and enable/disable error
            ///  logging.
            const MANAGE_LOGGING = 0x0000000000000001;
            ///  Permission to manage SSO and password policy.
            const MANAGE_AUTH = 0x0000000000000002;
            ///  Permission to manage Workspace CSE configuration.
            const MANAGE_WORKSPACE_CSE = 0x0000000000000004;
            ///  Permission required for Workspace CSE PrivilegedUnwrap API. Note
            ///  that `UNWRAP_WORKSPACE_CSE` permission in the group where the key is
            ///  stored is also required.
            const UNWRAP_WORKSPACE_CSE_PRIVILEGED = 0x0000000000000008;
            ///  Permission to manage account level client configurations.
            const MANAGE_ACCOUNT_CLIENT_CONFIGS = 0x0000000000000010;
            ///  Permission to manage plugin code signing policy.
            const MANAGE_PLUGIN_CODE_SIGNING_POLICY = 0x0000000000000020;
            ///  Permission to create account-level approval policy. Note that
            ///  updating/deleting the approval policy is protected by the approval
            ///  policy itself.
            const CREATE_ACCOUNT_APPROVAL_POLICY = 0x0000000000000040;
            ///  Permission to set approval request expiry for all approval requests
            ///  created in the account.
            const SET_APPROVAL_REQUEST_EXPIRY = 0x0000000000000080;
            ///  Permission to manage all approval request settings including
            ///  approval request expiry. Implies `SET_APPROVAL_REQUEST_EXPIRY`.
            const MANAGE_APPROVAL_REQUEST_SETTINGS = 0x0000000000000100;
            ///  Permission to update account's custom metadata attributes.
            const UPDATE_ACCOUNT_CUSTOM_METADATA_ATTRIBUTES = 0x0000000000000200;
            ///  Permission to manage account subscription (only relevant for SaaS
            ///  accounts).
            const MANAGE_ACCOUNT_SUBSCRIPTION = 0x0000000000000400;
            ///  Permission to update account name, custom logo, and other profile
            ///  information.
            const MANAGE_ACCOUNT_PROFILE = 0x0000000000000800;
            ///  Permission to delete the account.
            const DELETE_ACCOUNT = 0x0000000000001000;
            ///  Permission to create administrative apps. Implies `GET_ADMIN_APPS`.
            const CREATE_ADMIN_APPS = 0x0000000000002000;
            ///  Permission to update administrative apps. Implies `GET_ADMIN_APPS`.
            const UPDATE_ADMIN_APPS = 0x0000000000004000;
            ///  Permission to delete administrative apps. Implies `GET_ADMIN_APPS`.
            const DELETE_ADMIN_APPS = 0x0000000000008000;
            ///  Permission to retrieve administrative apps' secrets. Note that not
            ///  all admin app credentials contain secrets. If an admin app's
            ///  credential does not contain any secrets, `GET_ADMIN_APPS` permission
            ///  is sufficient to call the `GetAppCredential` API. Implies
            ///  `GET_ADMIN_APPS`.
            const RETRIEVE_ADMIN_APP_SECRETS = 0x0000000000010000;
            ///  Currently implies `CREATE_ADMIN_APPS`, `UPDATE_ADMIN_APPS`,
            ///  `DELETE_ADMIN_APPS`, `RETRIEVE_ADMIN_APP_SECRETS` and
            ///  `GET_ADMIN_APPS` permissions.
            const MANAGE_ADMIN_APPS = 0x0000000000020000;
            ///  Permission to create custom user roles. Implies `GET_CUSTOM_ROLES`.
            const CREATE_CUSTOM_ROLES = 0x0000000000040000;
            ///  Permission to update custom user roles. Implies `GET_CUSTOM_ROLES`.
            const UPDATE_CUSTOM_ROLES = 0x0000000000080000;
            ///  Permission to delete custom user roles. Implies `GET_CUSTOM_ROLES`.
            const DELETE_CUSTOM_ROLES = 0x0000000000100000;
            ///  Currently implies `CREATE_CUSTOM_ROLES`, `UPDATE_CUSTOM_ROLES`,
            ///  `DELETE_CUSTOM_ROLES` and `GET_CUSTOM_ROLES` permissions.
            const MANAGE_CUSTOM_ROLES = 0x0000000000200000;
            ///  Permission to invite users to the account. Implies `GET_ALL_USERS`.
            const INVITE_USERS_TO_ACCOUNT = 0x0000000000400000;
            ///  Permission to remove users from the account. Implies
            ///  `GET_ALL_USERS`.
            const DELETE_USERS_FROM_ACCOUNT = 0x0000000000800000;
            ///  Permission to change users' role in the account. Implies
            ///  `GET_ALL_USERS`.
            const UPDATE_USERS_ACCOUNT_ROLE = 0x0000000001000000;
            ///  Permission to enable/disable users in the account. Implies
            ///  `GET_ALL_USERS`.
            const UPDATE_USERS_ACCOUNT_ENABLED_STATE = 0x0000000002000000;
            ///  Currently implies `INVITE_USERS_TO_ACCOUNT`,
            ///  `DELETE_USERS_FROM_ACCOUNT`, `UPDATE_USERS_ACCOUNT_ROLE`,
            ///  `UPDATE_USERS_ACCOUNT_ENABLED_STATE` and `GET_ALL_USERS`
            ///  permissions.
            const MANAGE_ACCOUNT_USERS = 0x0000000004000000;
            ///  Permission to create external roles. Implies `GET_EXTERNAL_ROLES`.
            const CREATE_EXTERNAL_ROLES = 0x0000000008000000;
            ///  Permission to synchronize external roles. Implies
            ///  `GET_EXTERNAL_ROLES`.
            const SYNC_EXTERNAL_ROLES = 0x0000000010000000;
            ///  Permission to delete external roles. Implies `GET_EXTERNAL_ROLES`.
            const DELETE_EXTERNAL_ROLES = 0x0000000020000000;
            ///  Currently implies `CREATE_EXTERNAL_ROLES`, `SYNC_EXTERNAL_ROLES`,
            ///  `DELETE_EXTERNAL_ROLES` and `GET_EXTERNAL_ROLES` permissions.
            const MANAGE_EXTERNAL_ROLES = 0x0000000040000000;
            ///  Permission to create various account-level security object policies
            ///  including cryptographic policy, key metadata policy and key history
            ///  policy.
            const CREATE_ACCOUNT_SOBJECT_POLICIES = 0x0000000080000000;
            ///  Permission to update various account-level security object policies
            ///  including cryptographic policy, key metadata policy and key history
            ///  policy.
            const UPDATE_ACCOUNT_SOBJECT_POLICIES = 0x0000000100000000;
            ///  Permission to delete various account-level security object policies
            ///  including cryptographic policy, key metadata policy and key history
            ///  policy.
            const DELETE_ACCOUNT_SOBJECT_POLICIES = 0x0000000200000000;
            ///  Currently implies `CREATE_ACCOUNT_SOBJECT_POLICIES`,
            ///  `UPDATE_ACCOUNT_SOBJECT_POLICIES`, and
            ///  `DELETE_ACCOUNT_SOBJECT_POLICIES` permissions.
            const MANAGE_ACCOUNT_SOBJECT_POLICIES = 0x0000000400000000;
            ///  Permission to create child accounts. Note that this is only
            ///  applicable to SaaS accounts with reseller subscription. Implies
            ///  `GET_CHILD_ACCOUNTS`.
            const CREATE_CHILD_ACCOUNTS = 0x0000000800000000;
            ///  Permission to update child accounts. Note that this is only
            ///  applicable to SaaS accounts with reseller subscription. Implies
            ///  `GET_CHILD_ACCOUNTS`.
            const UPDATE_CHILD_ACCOUNTS = 0x0000001000000000;
            ///  Permission to delete child accounts. Note that this is only
            ///  applicable to SaaS accounts with reseller subscription. Implies
            ///  `GET_CHILD_ACCOUNTS`.
            const DELETE_CHILD_ACCOUNTS = 0x0000002000000000;
            ///  Permission to create users in child accounts. Note that this is only
            ///  applicable to SaaS accounts with reseller subscription. Implies
            ///  `GET_CHILD_ACCOUNTS` and `GET_CHILD_ACCOUNT_USERS`.
            const CREATE_CHILD_ACCOUNT_USERS = 0x0000004000000000;
            ///  Permission to get child accounts. Note that this is only applicable
            ///  to SaaS accounts with reseller subscription.
            const GET_CHILD_ACCOUNTS = 0x0000008000000000;
            ///  Permission to get child account users. Note that this is only
            ///  applicable to SaaS accounts with reseller subscription.
            const GET_CHILD_ACCOUNT_USERS = 0x0000010000000000;
            ///  Currently implies `CREATE_CHILD_ACCOUNTS`, `UPDATE_CHILD_ACCOUNTS`,
            ///  `DELETE_CHILD_ACCOUNTS`, `CREATE_CHILD_ACCOUNT_USERS`,
            ///  `GET_CHILD_ACCOUNTS`, and `GET_CHILD_ACCOUNT_USERS` permissions.
            const MANAGE_CHILD_ACCOUNTS = 0x0000020000000000;
            ///  Permission to create new local groups.
            const CREATE_LOCAL_GROUPS = 0x0000040000000000;
            ///  Permission to create new group backed by external HSM/KMS.
            const CREATE_EXTERNAL_GROUPS = 0x0000080000000000;
            ///  Controls if the user can act as an approval policy reviewer.
            const ALLOW_QUORUM_REVIEWER = 0x0000100000000000;
            ///  Controls if the user can act as a key custodian.
            const ALLOW_KEY_CUSTODIAN = 0x0000200000000000;
            ///  Grants read access to **all** approval requests in the account. Note
            ///  that there is a related group-level permission that is restricted to
            ///  approval requests related to one group.
            const GET_ALL_APPROVAL_REQUESTS = 0x0000400000000000;
            ///  Permission to get administrative apps.
            const GET_ADMIN_APPS = 0x0000800000000000;
            ///  Permission to get custom user roles.
            const GET_CUSTOM_ROLES = 0x0001000000000000;
            ///  Permission to get external roles.
            const GET_EXTERNAL_ROLES = 0x0002000000000000;
            ///  Permission to get all users. Note that users can always get
            ///  themselves.
            const GET_ALL_USERS = 0x0004000000000000;
            ///  Grants access to accounts::GetAccountUsage API.
            const GET_ACCOUNT_USAGE = 0x0008000000000000;
            ///  Permission to manage key expiry alert configurations.
            const MANAGE_KEY_EXPIRY_ALERTS = 0x0010000000000000;
            ///  Permission to modify an account's `purpose` field (e.g., changing a
            ///  replication account's settings), or to call any APIs involving
            ///  replication credentials. If the account is not a replication account,
            ///  this permission has no effect.
            const MANAGE_REPLICATION = 0x0020000000000000;
        }
    }
}

#[derive(PartialEq, Eq, Debug, Default, Serialize, Deserialize, Clone)]
pub struct AesOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_sizes: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpe: Option<FpeOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_length: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cipher_mode: Option<CipherMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_iv: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iv_length: Option<i32>
}

#[derive(PartialEq, Eq, Debug, Default, Serialize, Deserialize, Clone)]
pub struct AesOptionsPolicy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_sizes: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_iv: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpe: Option<FpeOptions>
}

/// A cryptographic algorithm.
#[derive(Debug, Eq, PartialEq, Copy, Hash, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum Algorithm {
    Aes,
    Aria,
    Des,
    Des3,
    Seed,
    Rsa,
    Dsa,
    Kcdsa,
    Ec,
    EcKcdsa,
    Bip32,
    Bls,
    Lms,
    MlDsaBeta,
    MlKemBeta,
    Hmac,
    LedaBeta,
    Round5Beta,
    Pbe
}

/// A helper enum with a single variant, All, which indicates that something should apply to an
/// entire part. (This is here mainly to allow other untagged enums to work properly.)
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum All {
    All
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct ApiPath {
    pub api_path: String,
    pub method: HyperHttpMethod,
    pub context: TepKeyContext,
    pub key_path: String
}

/// Operations allowed to be performed by an app.
pub use self::app_permissions::AppPermissions;
pub mod app_permissions {
    bitflags_set!{
        pub struct AppPermissions: u64 {
            const SIGN = 0x0000000000000001;
            const VERIFY = 0x0000000000000002;
            const ENCRYPT = 0x0000000000000004;
            const DECRYPT = 0x0000000000000008;
            const WRAPKEY = 0x0000000000000010;
            const UNWRAPKEY = 0x0000000000000020;
            const DERIVEKEY = 0x0000000000000040;
            const MACGENERATE = 0x0000000000000080;
            const MACVERIFY = 0x0000000000000100;
            const EXPORT = 0x0000000000000200;
            const MANAGE = 0x0000000000000400;
            const AGREEKEY = 0x0000000000000800;
            const MASKDECRYPT = 0x0000000000001000;
            const AUDIT = 0x0000000000002000;
            const TRANSFORM = 0x0000000000004000;
            const CREATE_SOBJECTS = 0x0000000000008000;
            const COPY_SOBJECTS = 0x0000000000010000;
            const ROTATE_SOBJECTS = 0x0000000000020000;
            const ACTIVATE_SOBJECTS = 0x0000000000040000;
            const REVOKE_SOBJECTS = 0x0000000000080000;
            const REVERT_SOBJECTS = 0x0000000000100000;
            const MOVE_SOBJECTS = 0x0000000000200000;
            const UPDATE_SOBJECTS_PROFILE = 0x0000000000400000;
            const UPDATE_SOBJECTS_ENABLED_STATE = 0x0000000000800000;
            const UPDATE_SOBJECT_POLICIES = 0x0000000001000000;
            const UPDATE_KEY_OPS = 0x0000000002000000;
            const DELETE_KEY_MATERIAL = 0x0000000004000000;
            const DELETE_SOBJECTS = 0x0000000008000000;
            const DESTROY_SOBJECTS = 0x0000000010000000;
            const RESTORE_EXTERNAL_SOBJECTS = 0x0000000020000000;
            const CALCULATE_DIGEST = 0x0000000040000000;
            const ENCAPSULATE = 0x0000000080000000;
            const DECAPSULATE = 0x0000000100000000;
        }
    }
}

/// Authentication requirements for approval request reviewers.
#[derive(Copy, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct ApprovalAuthConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_password: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_2fa: Option<bool>
}

/// Configurations for waiting for quorum approval.
#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct ApprovalWaitConfig {
    /// Indicates whether waiting for quorum approval is activated or disabled
    pub enabled: bool,
    /// Time interval in seconds for client lib to check quorum status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_interval_secs: Option<u64>,
    /// Maximum time in seconds for client lib to wait for quorum reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_wait_for_secs: Option<u64>
}

#[derive(PartialEq, Eq, Debug, Default, Serialize, Deserialize, Clone)]
pub struct AriaOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_sizes: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_length: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cipher_mode: Option<CipherMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_iv: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iv_length: Option<u8>
}

#[derive(PartialEq, Eq, Debug, Default, Serialize, Deserialize, Clone)]
pub struct AriaOptionsPolicy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_sizes: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_iv: Option<bool>
}

/// <https://www.w3.org/TR/webauthn-2/#enum-attestation-convey>
/// <https://www.w3.org/TR/webauthn-2/#sctn-attestation>
///
/// If you really want to understand attestation, read the following:
///   <https://fidoalliance.org/fido-technotes-the-truth-about-attestation/>
///   <https://medium.com/webauthnworks/webauthn-fido2-demystifying-attestation-and-mds-efc3b3cb3651>
///
/// This enum just specified how the attestation should be conveyed
/// to the RP. You can see doc of the individual variants to understand
/// various ways.
#[derive(Debug, Copy, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum AttestationConveyancePreference {
    /// When RP is not interested in attestation. In this case,
    /// attestation statement is None and RP can't identify the
    /// device.
    ///
    /// <https://www.w3.org/TR/webauthn-2/#sctn-none-attestation>
    ///
    /// This maybe good for UX as attestation may need user consent.
    None,
    /// RP prefers getting attestation statement but allows client
    /// to decide how to obtain it. (e.g., client may replace
    /// authenticator generated statement with [Anonymization CA])
    ///
    /// [Anonymization CA]: <https://www.w3.org/TR/webauthn-2/#anonymization-ca>
    Indirect,
    /// RP wants attestation statement as generated by the authenticator.
    Direct,
    /// RP wants attestation statement which can uniquely identify
    /// the authenticator. Generally meant for enterpise use.
    /// See spec for more info.
    Enterprise
}

/// LDAP authentication settings.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct AuthConfigLdap {
    pub name: String,
    pub icon_url: String,
    pub ldap_url: String,
    pub dn_resolution: LdapDnResolution,
    pub tls: TlsConfig,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_dn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_object_class: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_account: Option<LdapServiceAccount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization: Option<LdapAuthorizationConfig>
}

/// Extensions for webauthn. For every extension input, an
/// output must be returned if the input was considered.
///
/// https://www.w3.org/TR/webauthn-2/#dictdef-authenticationextensionsclientinputs
#[derive(Debug, Eq, PartialEq, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationExtensionsClientInputs {
    /// This extension excludes authenticators during registration
    /// based on legacy u2f key handles specified in "excludeCredentials".
    /// If that key handle was created with that device, it is excluded.
    ///
    /// https://www.w3.org/TR/webauthn-2/#sctn-appid-exclude-extension
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub appid_exclude: Option<String>,
    /// This extension allows RPs that have previously registered a cred
    /// using legacy U2F APIs to request an assertion.
    ///
    /// https://www.w3.org/TR/webauthn-2/#sctn-appid-extension
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub appid: Option<String>,
    /// Dummy extension used by conformance tests
    #[serde(default, rename = "example.extension.bool", skip_serializing_if = "Option::is_none")]
    pub example: Option<bool>
}

/// This is the response of extension inputs. For every input,
/// an output must be returned if the input was considered.
///
/// <https://www.w3.org/TR/webauthn-2/#iface-authentication-extensions-client-outputs>
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AuthenticationExtensionsClientOutputs {
    /// Response of `appidExclude` extension.
    /// See [AuthenticationExtensionsClientInputs::appid_exclude].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appid_exclude: Option<bool>,
    /// Response of `appid` extension.
    /// See [AuthenticationExtensionsClientInputs::appid].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appid: Option<bool>
}

/// <https://www.w3.org/TR/webauthn-2/#iface-authenticatorassertionresponse>
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticatorAssertionResponse {
    /// Base64url of client_data in JSON format.
    #[serde(rename = "clientDataJSON")]
    pub client_data_json: Base64<UrlSafe>,
    /// Data returned by authenticator.
    /// <https://www.w3.org/TR/webauthn-2/#sctn-authenticator-data>
    pub authenticator_data: Base64<UrlSafe>,
    /// Raw signature returned by authenticator.
    /// <https://www.w3.org/TR/webauthn-2/#sctn-op-get-assertion>
    pub signature: Base64<UrlSafe>,
    /// Corresponds to [PublicKeyCredentialUserEntity::id] sent during
    /// credential creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_handle: Option<Base64<UrlSafe>>
}

/// <https://www.w3.org/TR/webauthn-2/#enumdef-authenticatorattachment>
#[derive(Debug, Copy, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum AuthenticatorAttachment {
    /// An authenticator that is part of the client
    /// device. Usually not removable from the client
    /// device.
    Platform,
    /// Authenticator that can be removed and used on various
    /// devices via cross-platform transport protocols.
    CrossPlatform
}

/// Parameters for deciding which authenticators should be selected.
///
/// <https://www.w3.org/TR/webauthn-2/#dictdef-authenticatorselectioncriteria>
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticatorSelectionCriteria {
    /// Kind of authenticator attachment: attached to the
    /// client device or a roaming authenticator.
    /// See type level doc for more info.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authenticator_attachment: Option<AuthenticatorAttachment>,
    /// Preference about creating resident keys or not.
    /// See type level doc for more info.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resident_key: Option<ResidentKeyRequirement>,
    /// Exists for backcompat with webauthn level 1.
    /// By default it is false and should be set to true
    /// if `residentKey` is set to `required`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub require_resident_key: Option<bool>,
    /// Authenticator should support user verification by
    /// ways like pin code, biometrics, etc.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_verification: Option<UserVerificationRequirement>
}

/// Hints by relying party on how client should communicate
/// with the authenticator.
///
/// https://www.w3.org/TR/webauthn-2/#enum-transport
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum AuthenticatorTransport {
    /// Values known to the spec and DSM.
    Known (
        AuthenticatorTransportInner
    ),
    /// Unknown values are stored as spec asks to do so.
    /// As per the spec level 3 (which is draft):
    ///   "The values SHOULD be members of AuthenticatorTransport
    ///   but Relying Parties SHOULD accept and store unknown values."
    /// See `[[transports]]` in https://w3c.github.io/webauthn/#iface-authenticatorattestationresponse
    ///
    /// Level 2 also says that but comparitively unclear.
    ///   "The values SHOULD be members of AuthenticatorTransport but
    ///   Relying Parties MUST ignore unknown values."
    /// See `[[transports]]` in https://www.w3.org/TR/webauthn-2/#iface-authenticatorattestationresponse
    Unknown (
        String
    )
}

/// See [AuthenticatorTransport] type.
#[derive(Debug, Copy, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum AuthenticatorTransportInner {
    /// Over removable USB.
    Usb,
    /// Over Near Field Communication (NFC).
    Nfc,
    /// Over Bluetooth Smart (Bluetooth Low Energy / BLE).
    Ble,
    /// Indicates the respective authenticator is contacted using
    /// a client device-specific transport, i.e., it is a platform
    /// authenticator. These authenticators are not removable from
    /// the client device.
    Internal
}

/// Settings for automatic scanning in DSM-backed groups or replication accounts.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct AutoScanSettings {
    /// The number of hours between successive automatic scans. Must be greater than 0.
    pub scan_interval_hours: u8
}

#[derive(Copy, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum AwsKeyRotationStatus {
    KeyRotationDisabled,
    KeyRotationEnabled {
        #[serde(skip_serializing_if = "Option::is_none")]
        rotation_period_in_days: Option<u16>
    }
}

/// Information and properties of AWS KMS resources.
#[derive(PartialEq, Eq, Debug, Default, Serialize, Deserialize, Clone)]
pub struct AwsKmsInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region: Option<AwsMultiRegionInfo>,
    /// Rotation setting of the AWS KMS key. Note that this is unrelated to
    /// DSM key rotation/key rotation policy.
    ///
    /// See the [AWS documentation](https://docs.aws.amazon.com/kms/latest/APIReference/API_GetKeyRotationStatus.html)
    /// for more information.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aws_key_rotation_status: Option<AwsKeyRotationStatus>
}

/// This structure mentions various properties
/// of AWS multi region keys.
/// https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct AwsMultiRegionInfo {
    /// Specifies the type of multi region key to be
    /// either a Primary key or a Replica key.
    pub multi_region_key_type: AwsMultiRegionKeyType,
    /// Specifies a replica key's primary key ARN.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_key_arn: Option<String>,
    /// Specifies ARNs of a primary key's replicas.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replica_key_arns: Option<Vec<String>>
}

/// Specifies the type of multi-Region keys.
/// https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html#multi-region-concepts
#[derive(Copy, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum AwsMultiRegionKeyType {
    Primary,
    Replica
}

/// The BIP32 network
/// The Testnet network is usually an actual network with nodes and miners, and
/// free cryptocurrency. This provides a testing environment for developers.
#[derive(Debug, Eq, PartialEq, Copy, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Bip32Network {
    Mainnet,
    Testnet
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct Bip32Options {
    /// The BIP32 path, starting from master. Master key is Some([]).
    /// Ex: m/42/42'/0 -> Some([42, 2**31 + 42, 0])
    #[serde(skip_serializing_if = "Option::is_none")]
    pub derivation_path: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<Bip32Network>
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct Bip32OptionsPolicy {

}

#[derive(Copy, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct BlsOptions {
    pub variant: BlsVariant
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct BlsOptionsPolicy {

}

/// Signature/public-key size trade-off for BLS.
#[derive(Debug, Eq, PartialEq, Copy, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum BlsVariant {
    SmallSignatures,
    SmallPublicKeys
}

/// CA settings.
#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum CaConfig {
    CaSet (
        CaSet
    ),
    Pinned (
        Vec<Blob>
    )
}

/// Predefined CA sets.
#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum CaSet {
    GlobalRoots
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct CertificateOptionsPolicy {

}

/// Cipher mode used for symmetric key algorithms.
#[derive(Debug, Eq, PartialEq, Copy, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum CipherMode {
    Ecb,
    Cbc,
    CbcNoPad,
    Cfb,
    Ofb,
    Ctr,
    Gcm,
    Ccm,
    Kw,
    Kwp,
    Ff1
}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct ClientConfigurations {
    /// NOTE: not all clients use `common` configurations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub common: Option<CommonClientConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pkcs11: Option<Pkcs11ClientConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kmip: Option<KmipClientConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tep: Option<TepClientConfig>
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ClientConfigurationsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common: Option<Removable<CommonClientConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkcs11: Option<Removable<Pkcs11ClientConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kmip: Option<Removable<KmipClientConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tep: Option<Removable<TepClientConfig>>
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", tag = "mode")]
pub enum ClientFileLogging {
    Enabled (
        ClientFileLoggingConfig
    ),
    Disabled
}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct ClientFileLoggingConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_size_kb: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_files: Option<u32>
}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct ClientLogConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<ClientFileLogging>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct CollectionMetadata {
    /// Continuation token to continue getting results. If the response contains a continuation_token,
    /// the results returned are partial. In that case, the client can make the same request with this
    /// continuation token to continue getting results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    /// Count info about the items in the collection matching the request
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub count: Option<ObjectCount>
}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct CommonClientConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry_timeout_millis: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_ttl: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log: Option<ClientLogConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub h2_num_connections: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quorum_approval: Option<QuorumApprovalConfig>
}

/// An indicator of how accurate a count of objects is.
#[derive(Copy, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "$type")]
pub enum CountAccuracy {
    /// An exact count of objects.
    Exact,
    /// An approximate count of objects.
    Approximate {

    }
}

/// `CipherMode` or `RsaEncryptionPadding`, depending on the encryption algorithm.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum CryptMode {
    /// Block cipher mode of operation
    Symmetric (
        CipherMode
    ),
    /// RSA padding scheme
    Rsa (
        RsaEncryptionPadding
    )
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct CryptographicPolicy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aes: Option<AesOptionsPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aria: Option<AriaOptionsPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub des3: Option<Des3OptionsPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rsa: Option<RsaOptionsPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hmac: Option<HmacOptionsPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec: Option<EcOptionsPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_policy: Option<LegacyKeyPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_ops: Option<KeyOperations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub des: Option<DesOptionsPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<SeedOptionsPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dsa: Option<DsaOptionsPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kcdsa: Option<KcdsaOptionsPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eckcdsa: Option<EcKcdsaOptionsPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lms: Option<LmsOptionsPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mldsa_beta: Option<MlDsaBetaOptionsPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mlkem_beta: Option<MlKemBetaOptionsPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bip32: Option<Bip32OptionsPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bls: Option<BlsOptionsPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opaque: Option<OpaqueOptionsPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<SecretOptionsPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<CertificateOptionsPolicy>
}

#[derive(Debug, Eq, PartialEq, Copy, Serialize, Deserialize, Clone)]
pub struct CustomAttributeSearchMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggest: Option<bool>
}

#[derive(PartialEq, Eq, Debug, Default, Serialize, Deserialize, Clone)]
pub struct Des3Options {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_sizes: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cipher_mode: Option<CipherMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_iv: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iv_length: Option<i32>
}

/// Cryptographic policy for triple DES objects. Setting `key_sizes: [168]`
/// forbids two-key triple DES.
#[derive(PartialEq, Eq, Debug, Default, Serialize, Deserialize, Clone)]
pub struct Des3OptionsPolicy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_sizes: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_iv: Option<bool>
}

#[derive(PartialEq, Eq, Debug, Default, Serialize, Deserialize, Clone)]
pub struct DesOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cipher_mode: Option<CipherMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_iv: Option<bool>
}

#[derive(PartialEq, Eq, Debug, Default, Serialize, Deserialize, Clone)]
pub struct DesOptionsPolicy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_iv: Option<bool>
}

/// A hash algorithm.
#[derive(Debug, Eq, PartialEq, Copy, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum DigestAlgorithm {
    Blake2b256,
    Blake2b384,
    Blake2b512,
    Blake2s256,
    Ripemd160,
    Ssl3,
    Sha1,
    Sha224,
    Sha256,
    Sha384,
    Sha512,
    Streebog256,
    Streebog512,
    Sha3_224,
    Sha3_256,
    Sha3_384,
    Sha3_512
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct DsaOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subgroup_size: Option<u32>
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct DsaOptionsPolicy {

}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct EcKcdsaOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_alg: Option<DigestAlgorithm>
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct EcKcdsaOptionsPolicy {

}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct EcOptionsPolicy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elliptic_curves: Option<Vec<EllipticCurve>>
}

/// Operations allowed to be performed on a given key by a given User or an app
pub use self::effective_key_operations::EffectiveKeyOperations;
pub mod effective_key_operations {
    bitflags_set!{
        pub struct EffectiveKeyOperations: u64 {
            ///  If this is set, the key can be used for signing.
            const SIGN = 0x0000000000000001;
            ///  If this is set, the key can used for verifying a signature.
            const VERIFY = 0x0000000000000002;
            ///  If this is set, the key can be used for encryption.
            const ENCRYPT = 0x0000000000000004;
            ///  If this is set, the key can be used for decryption.
            const DECRYPT = 0x0000000000000008;
            ///  If this is set, the key can be used wrapping other keys.
            ///  The key being wrapped must have the EXPORT operation enabled.
            const WRAPKEY = 0x0000000000000010;
            ///  If this is set, the key can be used to unwrap a wrapped key.
            const UNWRAPKEY = 0x0000000000000020;
            ///  If this is set, the key can be used to derive another key.
            const DERIVEKEY = 0x0000000000000040;
            ///  If this is set, the key can be transformed.
            const TRANSFORM = 0x0000000000000080;
            ///  If this is set, the key can be used to compute a cryptographic
            ///  Message Authentication Code (MAC) on a message.
            const MACGENERATE = 0x0000000000000100;
            ///  If they is set, the key can be used to verify a MAC.
            const MACVERIFY = 0x0000000000000200;
            ///  If this is set, the value of the key can be retrieved
            ///  with an authenticated request. This shouldn't be set unless
            ///  required. It is more secure to keep the key's value inside DSM only.
            const EXPORT = 0x0000000000000400;
            ///  Without this operation, management operations like delete, destroy,
            ///  rotate, activate, restore, revoke, revert, update, remove_private, etc.
            ///  cannot be performed by a crypto App.
            ///  A user with access or admin app can still perform these operations.
            ///  This option is only relevant for crypto apps.
            const APPMANAGEABLE = 0x0000000000000800;
            ///  If this is set, audit logs will not be recorded for the key.
            ///   High volume here tries to signify a key that is being used a lot
            ///   and will produce lots of logs. Setting this operation disables
            ///   audit logs for the key.
            const HIGHVOLUME = 0x0000000000001000;
            ///  If this is set, the key can be used for key agreement.
            ///  Both the private and public key should have this option enabled
            ///  to perform an agree operation.
            const AGREEKEY = 0x0000000000002000;
            ///  If this is set, the key can be used for key encapsulation. The
            ///  result is a new symmetric key and a ciphertext.
            const ENCAPSULATE = 0x0000000000004000;
            ///  If this is set, the key can be used for key decapsulation. If
            ///  decapsulation succeeds, the result is a new symmetric key.
            const DECAPSULATE = 0x0000000000008000;
            ///  If this is set, the key can be used for masked decryption only.
            const MASKDECRYPT = 0x0000000000010000;
        }
    }
}

/// An aggregation of policies and permissions of the session creator for
/// a security object.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct EffectiveKeyPolicy {
    /// Indicates allowed key operations for the security key.
    pub key_ops: EffectiveKeyOperations,
    /// The effective export policy for the sobject. This takes both the sobject-level and group-level
    /// export policies into account.
    ///
    /// This field will only be present if the request explicitly asks for it.
    ///
    /// Note: The wrapping keys in the effective export policy may be specified either by name or
    /// by key id, and this may not correspond to how the wrapping key is specified either at the
    /// sobject-level or group-level export policy.
    ///
    /// Note: presence of a wrapping key in effective export policy DOES NOT guarantee that the
    /// wrapping key can be used by the current user/app to wrap the sobject for several reasons:
    /// the wrapping key may not actually exist, it may not have the necessary permissions, the
    /// user/app may not have access to the wrapping key, etc.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub export_policy: Option<ExportPolicy>
}

/// Identifies a standardized elliptic curve.
#[derive(Copy, Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub enum EllipticCurve {
    X25519,
    Ed25519,
    X448,
    SecP192K1,
    SecP224K1,
    SecP256K1,
    NistP192,
    NistP224,
    NistP256,
    NistP384,
    NistP521,
    Gost256A
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "$type")]
pub enum ExportPolicy {
    /// The sobject can only be exported wrapped by a key as specified
    /// by the `WrappingKeys`.
    Wrapped {
        by: WrappingKeys
    },
    /// The sobject can be exported unwrapped. This is the default policy
    /// for groups and also sobjects that have the `EXPORT` permission but
    /// do not specify an explicit export policy.
    Unrestricted
}

#[derive(PartialEq, Eq, Debug, Hash, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ExternalKeyId {
    Pkcs11 {
        id: Blob,
        label: Blob
    },
    Fortanix {
        id: Uuid
    },
    AwsKms {
        key_arn: String,
        key_id: String
    },
    AzureKeyVault {
        version: Uuid,
        label: String
    },
    GcpKeyRing {
        version: u32,
        label: String
    },
    Wrapped {

    }
}

/// Information specific to an external KMS. Currently, it only has AWS
/// related information.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ExternalKmsInfo {
    AWS (
        AwsKmsInfo
    )
}

/// This describes an external object -- specifically, information about its source object.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct ExternalSobjectInfo {
    /// The ID of the external object in the external HSM.
    pub id: ExternalKeyId,
    /// The group which corresponds to the external HSM.
    pub hsm_group_id: Uuid,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_kms_info: Option<ExternalKmsInfo>
}

/// Fido2 options when requesting assertion or attestation to a device
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Fido2MfaChallengeResponse {
    /// Attestation options
    Registration (
        PublicKeyCredentialCreationOptions
    ),
    /// Assertion options
    Authentication (
        PublicKeyCredentialRequestOptions
    )
}

/// The alphabet to use for an encrypted portion of a complex tokenization data type.
/// Characters should be specified as a list of pairs, where each pair [a, b] represents the
/// range of Unicode code points from a to b, with both bounds being inclusive. A single
/// code point can be specified as [c, c].
///
/// Normally, each character is assigned a numeric value for FF1. The first character is
/// assigned a value of 0, and subsequent characters are assigned values of 1, 2, and so on,
/// up to the size of the alphabet. Note that the order of the ranges matters; characters
/// appearing in later ranges are assigned higher numerical values compared to earlier
/// characters. For instance, in the FpeCharSet [['a', 'z'], ['0', '9']], the digits '0' to
/// '9' are assigned values from 26 to 35, since they are listed after the 'a' to 'z' range.
///
/// In any case, ranges should not overlap with each other, and should not contain surrogate
/// code points.
pub type FpeCharSet = Vec<[char; 2]>;

/// Structure of a compound portion of a complex tokenization data type, itself composed of
/// smaller parts.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum FpeCompoundPart {
    /// Represents an OR of multiple structures.
    ///
    /// Implementation note: an OR is _not_ a union of `FpeDataPart`s. Rather, when parsing
    /// the input, the backend will simply choose the first subpart that matches the current
    /// portion of the input, and tokenize/detokenize accordingly. If that choice results in
    /// an invalid parse of the rest of the input, the backend ***will not backtrack*** and
    /// will simply return with an error.
    Or {
        /// The actual subparts that make up this compound part.
        or: Vec<FpeDataPart>,
        /// Additional constraints that the token type must satisfy.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        constraints: Option<FpeConstraints>,
        /// Whether the entire OR should be preserved as-is (i.e., not tokenized). If this is
        /// set, any descendant subparts cannot contain any preserve-related fields set.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        preserve: Option<bool>,
        /// Whether the entire OR should be masked when doing masked decryption. If this is set,
        /// any descendant subparts cannot contain any mask-related fields set.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        mask: Option<bool>,
        /// The minimum allowed length for this part (in chars).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        min_length: Option<u32>,
        /// The maximum allowed length for this part (in chars).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_length: Option<u32>
    },
    /// Represents a concatenation of multiple structures (in a particular order).
    Concat {
        /// The actual subparts that make up this compound part, in order.
        concat: Vec<FpeDataPart>,
        /// Additional constraints that the token type must satisfy.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        constraints: Option<FpeConstraints>,
        /// Whether the entire concat should be preserved as-is (i.e., not tokenized). If this is
        /// set, any descendant subparts cannot contain any preserve-related fields set.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        preserve: Option<bool>,
        /// Whether the entire concat should be masked when doing masked decryption. If this is
        /// set, any descendant subparts cannot contain any mask-related fields set.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        mask: Option<bool>,
        /// The minimum allowed length for this part (in chars).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        min_length: Option<u32>,
        /// The maximum allowed length for this part (in chars).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_length: Option<u32>
    },
    /// Indicates a part that is possibly repeated multiple times.
    ///
    /// Implementation note: the backend parser is locally "greedy" and will attempt to match
    /// as many repetitions as possible. If this later results in an invalid parse of the rest
    /// of the input, the backend ***will not backtrack*** and will simply return with an error.
    Multiple {
        /// The subpart that may be repeated.
        multiple: Box<FpeDataPart>,
        /// The minimum number of times the subpart may occur. (A value of 1 marks a single
        /// occurrence.)
        #[serde(skip_serializing_if = "Option::is_none")]
        min_repetitions: Option<usize>,
        /// The maximum number of times the subpart may occur. (A value of 1 marks a single
        /// occurrence.)
        #[serde(skip_serializing_if = "Option::is_none")]
        max_repetitions: Option<usize>,
        /// Additional constraints that the token type must satisfy.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        constraints: Option<FpeConstraints>,
        /// Whether the entire Multiple should be preserved as-is (i.e., not tokenized). If this
        /// is set, the `multiple` subpart and its descendants cannot contain any preserve-related
        /// fields set.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        preserve: Option<bool>,
        /// Whether the entire Multiple should be masked when doing masked decryption. If this is
        /// set, the `multiple` subpart and its descendants cannot contain any mask-related fields
        /// set.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        mask: Option<bool>,
        /// The minimum allowed length for this part (in chars).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        min_length: Option<u32>,
        /// The maximum allowed length for this part (in chars).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_length: Option<u32>
    }
}

/// Constraints on a portion of a complex tokenization data type.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct FpeConstraints {
    /// Whether the token part contains a checksum that satisfies the Luhn formula. It is an
    /// error to apply this constraint to non-numeric parts, or to have an encrypted part be
    /// under more than one Luhn check constraint. Also, if an encrypted part has a Luhn check
    /// constraint applied to it and may contain at least one digit that is not preserved, it
    /// must not specify any other constraints.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub luhn_check: Option<bool>,
    /// Number that the token part should be greater than.
    ///
    /// This constraint can only be specified on (non-compound) numeric encrypted parts
    /// guaranteed to preserve either everything or nothing at all. (For example, if an
    /// encrypted part consists of 5 to 10 digits, a `preserve` list that covers only the
    /// first five digits is not guaranteed to preserve everything, because if the input
    /// happens to be six or more digits long, there will be at least one digit that
    /// remains unpreserved.)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_gt: Option<usize>,
    /// Number that the token part should be smaller than.
    ///
    /// This constraint can only be specified on (non-compound) numeric encrypted parts
    /// guaranteed to preserve either everything or nothing at all. (For example, if an
    /// encrypted part consists of 5 to 10 digits, a `preserve` list that covers only the
    /// first five digits is not guaranteed to preserve everything, because if the input
    /// happens to be six or more digits long, there will be at least one digit that
    /// remains unpreserved.)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_lt: Option<usize>,
    /// Numbers that the token part should not be equal to. It is an error to apply this
    /// constraint to non-numeric parts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_ne: Option<Vec<usize>>,
    /// Specifies that this portion is supposed to represent a date, or part of one. If used,
    /// no other constraints can be specified on this part.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<FpeDateConstraint>,
    /// The subparts to apply the constaints to. If not specified, the constraints will be
    /// applied to all subparts (recursively).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applies_to: Option<FpeConstraintsApplicability>
}

/// A structure indicating which subparts to which to apply a set of constraints.
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum FpeConstraintsApplicability {
    /// Indicates that the constraints apply to the entire part (i.e., all of its subparts),
    /// including any descendants. This is the default value for this enum and the only option
    /// available for FpeEncryptedPart, literal, and OR subparts.
    Simple (
        All
    ),
    /// An object representing the individual subparts that the constraints should apply to. This
    /// is a BTreeMap where for each key-value pair, the key represents the "index" of the subpart
    /// (with the first subpart having index 0), and the value is an FpeConstraintsApplicability
    /// instance. Note that a Multiple part only allows for one possible key-value pair, since it
    /// only contains one subpart.
    ///
    /// This cannot be used with OR parts; instead, specify constraints individually on each
    /// relevant subpart.
    BySubparts (
        HashMap<FpeSubpartIndex,FpeConstraintsApplicability>
    )
}

/// Structure for specifying (part of) a complex tokenization data type.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum FpeDataPart {
    /// A portion of a data type consisting of characters that belong to a particular
    /// alphabet (e.g., letters, numbers, etc.).
    Encrypted (
        FpeEncryptedPart
    ),
    /// A section of the data type that is not to be tokenized (e.g., a delimiter).
    ///
    /// Unlike preserved characters, literal characters are not used for FF1 tweaks.
    Literal {
        /// The list of possible strings that make up this literal portion of the token.
        /// For example, if a delimiter can either be a space or a dash, the list would
        /// be `[" ", "-"]`.
        ///
        /// Implementation note: the backend will pick the first choice that matches when
        /// when parsing the input. If this results in an invalid parse of the rest of the
        /// input, the backend ***will not backtrack*** and will simply return with an error.
        literal: Vec<String>
    },
    /// A portion of a data type that consists of several smaller portions (e.g., an
    /// `Encrypted` part followed by a `Literal`).
    Compound (
        FpeCompoundPart
    )
}

/// A structure for specifying a token part representing a date that occurs after a specified date
/// and/or occurs before a specified date. Depending on the subparts that make up the date, one of
/// the three options is used.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub enum FpeDate {
    /// Represents a date that consists of a Month subpart, a Day subpart, and a Year subpart. The
    /// Year part is allowed to be preserved, and the Day and Month parts are allowed to be
    /// preserved together. (The Day part cannot be preserved if the Month part is not, and vice
    /// versa.)
    #[serde(rename = "dmy_date")]
    DayMonthYear {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        before: Option<FpeDayMonthYearDate>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        after: Option<FpeDayMonthYearDate>
    },
    /// Represents a date that consists of a Month subpart and a Day subpart. It is an error to
    /// preserve only the Month part or the Day part.
    #[serde(rename = "month_day_date")]
    MonthDay {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        before: Option<FpeDayMonthDate>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        after: Option<FpeDayMonthDate>
    },
    /// Represents a date that consists of a Month subpart and a Year subpart. The Year part is
    /// allowed to be preserved; however, the Month part cannot be preserved by itself.
    #[serde(rename = "month_year_date")]
    MonthYear {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        before: Option<FpeMonthYearDate>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        after: Option<FpeMonthYearDate>
    }
}

/// Possible date-related constraint types for a portion of a complex tokenization data type.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum FpeDateConstraint {
    /// Used to indicate that a token part represents a date, which should occur before and/or
    /// after any specified bounds. The part should be a concatenation that contains either
    /// - a Day part and a Month part
    /// - a Month part and a Year part
    /// - a Day part, a Month part, and a Year part
    /// (with this constraint applying to those subparts). Each of the three choices above
    /// corresponds to a particular FpeDate variant; using the wrong variant is an error.
    ///
    /// Furthermore, the individual Month, Day, and/or Year parts that comprise the date cannot
    /// appear under Or or Multiple compound part descendants of the overall Date part (i.e.,
    /// when applying the Date constraint, the "paths" from the Date part to the Month, Day,
    /// and/or Year parts can only "go through" concatenations, and not "through" Or or Multiple
    /// parts). Those parts also have additional restrictions on how they may be preserved; the
    /// exact rules depend on the FpeDate variant.
    ///
    /// It is an error to "share" Day, Month, or Year parts across multiple dates.
    Date (
        FpeDate
    ),
    /// Used to indicate that a token part represents a month, day, or year (either as part of a
    /// date, or independently).
    ///
    /// The token part must be a (non-compound) numeric encrypted part guaranteed to preserve either
    /// everything or nothing at all. (For example, if an encrypted part consists of 5 to 10 digits,
    /// a `preserve` list that covers only the first five digits is not guaranteed to preserve
    /// everything, because if the input happens to be six or more digits long, there will be at
    /// least one digit that remains unpreserved.)
    ///
    /// Additionally, the token part cannot be involved in any Luhn-check constraints.
    DatePart (
        FpeDatePart
    )
}

/// Possible date-related constraint types that do not form a complete date (by themselves) for a
/// complex tokenization data type.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum FpeDatePart {
    /// Used to indicate that a token part represents a month. The part should be an integer from 1
    /// to 12, have its min_length field be at least 1, and have its max_length field be 2. Any
    /// leading zero should be removed (unless the part is always 2 digits long, in which case a
    /// leading zero may be needed).
    Month,
    /// Used to indicate that a token part represents a day. The part should be an integer from 1 to
    /// 31, have its min_length field be at least 1, and have its max_length field be 2. Any
    /// leading zero should be removed (unless the part is always 2 digits long, in which case a
    /// leading zero may be needed). Further restrictions apply when the Day part occurs within a
    /// date; for instance, a date of 2/29/2000 is fine, but 4/31 is not.
    Day,
    /// Used to indicate that a token part represents a year, with any zero value being treated as
    /// a leap year. The part should be a two to five digit number.
    Year
}

/// A structure for specifying a particular date consisting of a day and a month, for use in an
/// FpeDate structure.
#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize, Clone)]
pub struct FpeDayMonthDate {
    /// The month, which should be an integer from 1 to 12.
    pub month: u8,
    /// The day, which should be an integer from 1 to either 29, 30, or 31, depending on the month
    /// and year. Here, February is treated as having 29 days.
    pub day: u8
}

/// A structure for specifying a particular date consisting of a day, month, and year, for use in
/// an FpeDate structure.
#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize, Clone)]
pub struct FpeDayMonthYearDate {
    /// The year, which should be an integer less than 100000. Zero is treated as a leap year.
    pub year: u32,
    /// The month, which should be an integer from 1 to 12.
    pub month: u8,
    /// The day, which should be an integer from 1 to either 28, 29, 30, or 31, depending on the
    /// month and year.
    pub day: u8
}

/// Structure of a tokenized portion of a complex tokenization data type.
///
/// Implementation note: the backend parser is locally "greedy" and will attempt to match
/// as many characters as possible. If this later results in an invalid parse of the rest
/// of the input, the backend ***will not backtrack*** and will simply return with an error.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct FpeEncryptedPart {
    /// The minimum allowed length for this part (in chars).
    pub min_length: u32,
    /// The maximum allowed length for this part (in chars).
    pub max_length: u32,
    /// The alphabet to use for this part.
    pub char_set: FpeCharSet,
    /// The output alphabet to use for this part. Defaults to `char_set` if not specified.
    /// When specified, the cardinality of `cipher_char_set` must be the same as `char_set`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cipher_char_set: Option<FpeCharSet>,
    /// Additional constraints that the token type must satisfy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<FpeConstraints>,
    /// The characters to be preserved while encrypting or decrypting.
    ///
    /// Any preserved characters will be concatenated together, and their UTF-8 bytes will be used
    /// as an FF1 tweak. For example, if the input data is "abcd", and the first and last characters
    /// are to be preserved, the FF1 tweak will be the bytes of the string "ad".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preserve: Option<FpePreserveMask>,
    /// The characters to be masked while performing masked decryption.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mask: Option<FpePreserveMask>
}

/// A structure for specifying a particular date consisting of a month and a year, for use in an
/// FpeDate structure.
#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize, Clone)]
pub struct FpeMonthYearDate {
    /// The year, which should be an integer less than 100000. Zero is treated as a leap year.
    pub year: u32,
    /// The month, which should be an integer from 1 to 12.
    pub month: u8
}

/// FPE-specific options (for specifying the format of the
/// data to be encrypted)
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum FpeOptions {
    /// Basic FPE options, suitable for simple datatypes. See the
    /// description of FpeOptionsBasic for more details.
    Basic (
        FpeOptionsBasic
    ),
    /// Advanced FPE options. It is recommended to use this for
    /// specifying any FPE options, as it is more expressive than
    /// FpeOptionsBasic.
    Advanced {
        /// The structure of the data type.
        format: FpeDataPart,
        /// The user-provided name for the data type.
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>
    }
}

/// Basic FPE-specific options. This is suitable for simple datatypes
/// that consist of ASCII digits, or ASCII digits and uppercase letters.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct FpeOptionsBasic {
    /// The FPE base for the input data (i.e., the size of the character
    /// set of the datatype). This must be an integer from 2 to 36.
    ///
    /// This also implicitly defines the alphabet of the datatype. A base
    /// from 2 to 10 implies ASCII digits (e.g., a radix of 3 can be used
    /// to represent a ternary string), and a base from 11 to 36 implies
    /// ASCII digits and uppercase letters (e.g., a radix of 16 can be
    pub radix: u32,
    /// The minimum allowed length for the input data.
    pub min_length: u32,
    /// The maximum allowed length for the input data.
    pub max_length: u32,
    /// The list of indices of characters to be preserved while performing encryption/decryption.
    /// Indices are Python-like; i.e., nonnegative indices index from the beginning of the input
    /// (where 0 is the first character), and negative indices index from the end of the input.
    /// (where -1 is the last character, -2 is second to last, and so on).
    ///
    /// Any preserved characters will be concatenated together and used as an FF1 tweak. For example,
    /// if the input data is "abcd", and the first and last characters are to be preserved, the FF1
    /// tweak will be the ASCII bytes of the string "ad".
    pub preserve: Vec<isize>,
    /// The list of indices of characters to be masked while performing masked decryption.
    /// Indices are Python-like; i.e., nonnegative indices index from the beginning of the input
    /// (where 0 is the first character), and negative indices index from the end of the input.
    /// (where -1 is the last character, -2 is second to last, and so on).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask: Option<Vec<isize>>,
    /// Whether the encrypted/decrypted data contains a checksum digit that satisfies the Luhn
    /// formula. (The output ciphertext/plaintext will also contain a Luhn checksum digit.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub luhn_check: Option<bool>,
    /// The user-provided name for the data type that represents the input data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>
}

/// A structure indicating which indices in an encrypted part to mask or preserve.
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum FpePreserveMask {
    /// Indicates that the entire encrypted part is to be preserved or masked.
    Entire (
        All
    ),
    /// Indicates that only certain characters are to be preserved or masked. Indices are
    /// Python-like; i.e., negative indices index from the end of the token portion, with
    /// index -1 denoting the last character. (Indicating that nothing should be preserved
    /// or masked can be done via an empty list, which is the default value for this enum.)
    ByChars (
        Vec<isize>
    )
}

/// An index for listing subparts of a compound part to which certain constraints are to be applied.
/// For Concat parts, this is the zero-based index of the subpart in the `concat` field, and for
/// Multiple parts, this is always 0 (due to a Multiple having only one subpart).
pub type FpeSubpartIndex = usize;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetGroupsParams {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<GroupSort>,
    /// If specified, only groups matching this `filter` are returned.
    ///
    /// The following fields can be referenced in the filter:
    /// - `name`
    /// - `created_at`
    /// - `description`
    /// - `wrapping_key_name`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// Number of groups to skip
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    /// Continuation token to continue getting results. It must be the same
    /// token returned from the backend from a previous call, or empty.
    ///
    /// Existence of this query parameter controls the response (and the backend behavior):
    /// - If specified (including an empty value), the backend returns metadata alongside
    ///   the collection of groups. The metadata will potentially contain a fresh `continuation_token`.
    ///
    ///   Note: If there is a `limit` specified in the request and DSM returns `limit`-many items in the
    ///   response, it will still include a fresh continuation token if there are more items in the collection.
    ///   Additionally, unlike other query parameters, `limit` is not required to remain unchanged in a chain of
    ///   requests with `coninutation_token`s.
    /// - If omitted, the backend returns just a collection of groups with no metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>
}

impl UrlEncode for GetGroupsParams {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        if let Some(ref v) = self.limit {
            m.insert("limit".to_string(), v.to_string());
        }
        self.sort_by.url_encode(m);
        if let Some(ref v) = self.filter {
            m.insert("filter".to_string(), v.to_string());
        }
        if let Some(ref v) = self.offset {
            m.insert("offset".to_string(), v.to_string());
        }
        if let Some(ref v) = self.continuation_token {
            m.insert("continuation_token".to_string(), v.to_string());
        }
    }
}

/// An access reason provided by Google when making EKMS API calls.
#[derive(Debug, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GoogleAccessReason {
    /// No reason is available for the access.
    ReasonUnspecified = 0,
    /// Access Transparency Types, public documentation can be found at:
    /// https://cloud.google.com/logging/docs/audit/reading-access-transparency-logs#justification-reason-codes
    CustomerInitiatedSupport = 1,
    GoogleInitiatedService = 2,
    ThirdPartyDataRequest = 3,
    GoogleInitiatedReview = 4,
    /// Customer uses their own account or grants IAM permission to some
    /// partner of theirs to perform any access to their own data which is
    /// authorized by their own IAM policy.
    CustomerInitiatedAccess = 5,
    /// Google access to data to help optimize the data's structure or quality
    /// for future uses by the customer. This includes but is not limited to
    /// accesses for the purposes of indexing, structuring, precomputation,
    /// hashing, sharding and caching. This also includes backing up data for disaster
    /// recovery or data integrity reasons, and detecting errors that can be
    /// remedied from that backup data.
    /// Note that where the customer has delegated a managed control plane
    /// operation to Google, such as the creation of a managed instance group,
    /// all managed operations will show as system operations. Services such as
    /// the managed instance group manager that trigger downstream decryption
    /// operations do not have access to clear-text customer data.
    GoogleInitiatedSystemOperation = 6,
    /// No reason is expected for this key request as the service in
    /// question has never integrated with Key Access Justifications, or is still
    /// in Pre GA state and therefore may still have residual methods that call
    /// the External Key Manager but still do not provide a justification.
    ReasonNotExpected = 7,
    /// A Customer uses their account to perform any access to their own data
    /// which is authorized by their own IAM policy, however a Google
    /// administrator has reset the superuser account associated with the user’s
    /// Organization within the last 7 days.
    ModifiedCustomerInitiatedAccess = 8,
    /// Google accesses customer data to help optimize the structure of the data or quality for future uses by the customer.
    /// These accesses can be for indexing, structuring, precomputation, hashing, sharding and caching customer data
    /// This also includes backing up data for disaster recovery or data integrity reasons,
    /// and detecting errors that the backup data could remedy. At the same time,
    /// a Google-initiated breakglass operation has affected the accessed resource.
    ModifiedGoogleInitiatedSystemOperation = 9,
    /// Refers to Google-initiated access to maintain system reliability.
    /// Google personnel can make this type of access for the following reasons:
    /// - To investigate and confirm that a suspected service outage doesn't affect the customer.
    /// - To ensure backup and recovery from outages and system failures.
    GoogleResponseToProductionAlert = 10,
    CustomerAuthorizedWorkflowServicing = 11
}

/// Policy specifying acceptable access reasons
/// by Google Service Account at App or Sobject level.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct GoogleAccessReasonPolicy {
    /// Set of allowed Google Access reasons.
    pub allow: HashSet<GoogleAccessReason>,
    /// Accept incoming requests which do not specify any access reasons.
    pub allow_missing_reason: bool
}

pub use self::group_permissions::GroupPermissions;
pub mod group_permissions {
    bitflags_set!{
        #[derive(Default)]
        pub struct GroupPermissions: u64 {
            ///  Permission to create group-level approval policy. Note that
            ///  updating/deleting the approval policy is protected by the approval
            ///  policy itself. Implies `GET_GROUP`.
            const CREATE_GROUP_APPROVAL_POLICY = 0x0000000000000001;
            ///  Permission to update external HSM/KMS configurations. Note that this
            ///  is only useful for groups backed by external HSM/KMS. Implies
            ///  `GET_GROUP`.
            const UPDATE_GROUP_EXTERNAL_LINKS = 0x0000000000000002;
            ///  Permission to manage group-level client configurations. Implies
            ///  `GET_GROUP`.
            const MANAGE_GROUP_CLIENT_CONFIGS = 0x0000000000000004;
            ///  Permission to update name, description and custom metadata of the
            ///  group. Implies `GET_GROUP`.
            const UPDATE_GROUP_PROFILE = 0x0000000000000008;
            ///  Permission to delete the group. Implies `GET_GROUP`.
            const DELETE_GROUP = 0x0000000000000010;
            ///  Permission to map external roles to DSM groups for apps authorized
            ///  through LDAP. Implies `GET_GROUP`.
            const MAP_EXTERNAL_ROLES_FOR_APPS = 0x0000000000000020;
            ///  Permission to map external roles to DSM groups for users authorized
            ///  through LDAP. Implies `GET_GROUP`.
            const MAP_EXTERNAL_ROLES_FOR_USERS = 0x0000000000000040;
            ///  Currently implies `MAP_EXTERNAL_ROLES_FOR_APPS`,
            ///  `MAP_EXTERNAL_ROLES_FOR_USERS`, and `GET_GROUP` permissions.
            const MAP_EXTERNAL_ROLES = 0x0000000000000080;
            ///  Permission to add users to the group.
            const ADD_USERS_TO_GROUP = 0x0000000000000100;
            ///  Permission to remove users from the group.
            const DELETE_USERS_FROM_GROUP = 0x0000000000000200;
            ///  Permission to change users' role in the group.
            const UPDATE_USERS_GROUP_ROLE = 0x0000000000000400;
            ///  Currently implies `ADD_USERS_TO_GROUP`, `DELETE_USERS_FROM_GROUP`,
            ///  and `UPDATE_USERS_GROUP_ROLE` permissions.
            const MANAGE_GROUP_USERS = 0x0000000000000800;
            ///  Permission to create various group-level security object policies
            ///  including cryptographic policy, key metadata policy and key history
            ///  policy. Implies `GET_GROUP`.
            const CREATE_GROUP_SOBJECT_POLICIES = 0x0000000000001000;
            ///  Permission to update various group-level security object policies
            ///  including cryptographic policy, key metadata policy and key history
            ///  policy. Implies `GET_GROUP`.
            const UPDATE_GROUP_SOBJECT_POLICIES = 0x0000000000002000;
            ///  Permission to delete various group-level security object policies
            ///  including cryptographic policy, key metadata policy and key history
            ///  policy. Implies `GET_GROUP`.
            const DELETE_GROUP_SOBJECT_POLICIES = 0x0000000000004000;
            ///  Currently implies `CREATE_GROUP_SOBJECT_POLICIES`,
            ///  `UPDATE_GROUP_SOBJECT_POLICIES`, `DELETE_GROUP_SOBJECT_POLICIES`,
            ///  and `GET_GROUP` permissions.
            const MANAGE_GROUP_SOBJECT_POLICIES = 0x0000000000008000;
            ///  Permission to create key custodian policy for the group. Implies
            ///  `GET_GROUP`.
            const CREATE_GROUP_CUSTODIAN_POLICY = 0x0000000000010000;
            ///  Permission to update group's key custodian policy. Implies
            ///  `GET_GROUP`.
            const UPDATE_GROUP_CUSTODIAN_POLICY = 0x0000000000020000;
            ///  Permission to delete group's key custodian policy. Implies
            ///  `GET_GROUP`.
            const DELETE_GROUP_CUSTODIAN_POLICY = 0x0000000000040000;
            ///  Currently implies `CREATE_GROUP_CUSTODIAN_POLICY`,
            ///  `UPDATE_GROUP_CUSTODIAN_POLICY`, `DELETE_GROUP_CUSTODIAN_POLICY`,
            ///  and `GET_GROUP` permissions.
            const MANAGE_GROUP_CUSTODIAN_POLICY = 0x0000000000080000;
            ///  Permission to create cryptographic apps. Implies `GET_APPS`.
            const CREATE_APPS = 0x0000000000100000;
            ///  Permission to update cryptographic apps. Implies `GET_APPS`.
            const UPDATE_APPS = 0x0000000000200000;
            ///  Permission to retrieve cryptographic apps' secrets. Note that not
            ///  all cryptographic app credentials contain secrets. If a
            ///  cryptographic app's credential does not contain any secrets,
            ///  `GET_APPS` permission is sufficient to call the `GetAppCredential`
            ///  API. Implies `GET_APPS`.
            const RETRIEVE_APP_SECRETS = 0x0000000000400000;
            ///  Permission to delete cryptographic apps. Implies `GET_APPS`.
            const DELETE_APPS = 0x0000000000800000;
            ///  Currently implies `CREATE_APPS`, `UPDATE_APPS`,
            ///  `RETRIEVE_APP_SECRETS`, `DELETE_APPS`, and `GET_APPS` permissions.
            const MANAGE_APPS = 0x0000000001000000;
            ///  Permission to create plugins. Implies `GET_PLUGINS`.
            ///  For creating a plugin, following group permissions are also required
            ///  in each group plugin is being added, to prevent privilege escalation:
            ///  `CREATE_SOBJECTS`, `EXPORT_SOBJECTS`, `COPY_SOBJECTS`,
            ///  `WRAP_SOBJECTS`, `UNWRAP_SOBJECTS`, `ENCAPSULATE_SOBJECTS`, `DECAPSULATE_SOBJECTS`,
            ///  `DERIVE_SOBJECTS`, `TRANSFORM_SOBJECTS`, `UPDATE_SOBJECTS_ENABLED_STATE`,
            ///  `ROTATE_SOBJECTS`, `DELETE_SOBJECTS`, `REVOKE_SOBJECTS`, `ACTIVATE_SOBJECTS`,
            ///  `MOVE_SOBJECTS`, `UPDATE_KEY_OPS`, `UPDATE_SOBJECT_POLICIES`, `UPDATE_SOBJECTS_PROFILE`,
            ///  `GET_GROUP`, `GET_SOBJECTS`, `GET_APPS`, `GET_PLUGINS`, `GET_AUDIT_LOGS`
            ///  Following account permissions are required as well:
            ///  `GET_ALL_USERS`
            const CREATE_PLUGINS = 0x0000000002000000;
            ///  Permission to update plugins. Implies `GET_PLUGINS`.
            ///  For updating a plugin, following group permissions are also required
            ///  in each group plugin is being added, to prevent privilege escalation:
            ///  `CREATE_SOBJECTS`, `EXPORT_SOBJECTS`, `COPY_SOBJECTS`, `WRAP_SOBJECTS`, `UNWRAP_SOBJECTS`,
            ///  `ENCAPSULATE_SOBJECTS`, `DECAPSULATE_SOBJECTS`, `UPDATE_SOBJECTS_ENABLED_STATE`,
            ///  `ROTATE_SOBJECTS`, `DELETE_SOBJECTS`, `REVOKE_SOBJECTS`, `ACTIVATE_SOBJECTS`,
            ///  `MOVE_SOBJECTS`, `UPDATE_KEY_OPS`, `UPDATE_SOBJECT_POLICIES`, `UPDATE_SOBJECTS_PROFILE`,
            ///  `GET_GROUP`, `GET_SOBJECTS`, `GET_APPS`, `GET_PLUGINS`, `GET_AUDIT_LOGS`
            ///  Following account permissions are required as well while adding
            ///  new groups:
            ///  `GET_ALL_USERS`
            const UPDATE_PLUGINS = 0x0000000004000000;
            ///  Permission to invoke plugins. Implies `GET_PLUGINS`.
            const INVOKE_PLUGINS = 0x0000000008000000;
            ///  Permission to delete plugins. Implies `GET_PLUGINS`.
            const DELETE_PLUGINS = 0x0000000010000000;
            ///  Currently implies `CREATE_PLUGINS`, `UPDATE_PLUGINS`,
            ///  `INVOKE_PLUGINS`, `DELETE_PLUGINS`, and `GET_PLUGINS` permissions.
            const MANAGE_PLUGINS = 0x0000000020000000;
            ///  Permission to create security objects. This permission is required
            ///  for APIs that result in creation of a new security object including:
            ///  Generate, Import, Unwrap. Also required in destination group when
            ///  moving a key to a different group or when copying a key. Implies
            ///  `GET_SOBJECTS`.
            const CREATE_SOBJECTS = 0x0000000040000000;
            ///  Permission to export security objects. This permission is required
            ///  for Export, ExportByComponents, Copy (depending on destination
            ///  group), Restore, and Wrap (for wrapped security object) APIs.
            ///  Implies `GET_SOBJECTS`.
            const EXPORT_SOBJECTS = 0x0000000080000000;
            ///  Permission to copy security objects. This permission is required in
            ///  the source group when calling the Copy API. Implies `GET_SOBJECTS`.
            const COPY_SOBJECTS = 0x0000000100000000;
            ///  Permission to wrap security objects. This permission is required in
            ///  the wrapping security object's group. Implies `GET_SOBJECTS`.
            const WRAP_SOBJECTS = 0x0000000200000000;
            ///  Permission to unwrap security objects. This permission is required
            ///  in the unwrapping security object's group. Implies `GET_SOBJECTS`.
            const UNWRAP_SOBJECTS = 0x0000000400000000;
            ///  Permission to derive other security objects. Implies `GET_SOBJECTS`.
            const DERIVE_SOBJECTS = 0x0000000800000000;
            ///  Permission to transform security objects. Implies `GET_SOBJECTS`.
            const TRANSFORM_SOBJECTS = 0x0000001000000000;
            ///  Permission to enable/disable security objects. Implies
            ///  `GET_SOBJECTS`.
            const UPDATE_SOBJECTS_ENABLED_STATE = 0x0000002000000000;
            ///  Permission to rotate (a.k.a. "rekey") security objects. Implies
            ///  `GET_SOBJECTS`.
            const ROTATE_SOBJECTS = 0x0000004000000000;
            ///  Permission to delete security objects. Implies `GET_SOBJECTS`.
            const DELETE_SOBJECTS = 0x0000008000000000;
            ///  Permission to destroy security objects. Implies `GET_SOBJECTS`.
            const DESTROY_SOBJECTS = 0x0000010000000000;
            ///  Permission to revoke security objects, i.e. mark security objects as
            ///  deactivated or compromised. Implies `GET_SOBJECTS`.
            const REVOKE_SOBJECTS = 0x0000020000000000;
            ///  Permission to activate security objects. Implies `GET_SOBJECTS`.
            const ACTIVATE_SOBJECTS = 0x0000040000000000;
            ///  Permission to revert changes to security objects. Implies
            ///  `GET_SOBJECTS`.
            const REVERT_SOBJECTS = 0x0000080000000000;
            ///  Permission to delete key material including removing the private key
            ///  part of an asymmetric key pair and removing key material of security
            ///  objects backed by external HSM/KMS. Implies `GET_SOBJECTS`.
            const DELETE_KEY_MATERIAL = 0x0000100000000000;
            ///  Permission to move security objects. This permission is required for
            ///  changing the group of a security object in the source group. Note
            ///  that changing the group of a security object also requires
            ///  `CREATE_SOBJECTS` permission in the destination group. Implies
            ///  `GET_SOBJECTS`.
            const MOVE_SOBJECTS = 0x0000200000000000;
            ///  Permission to update key operations of security objects. Implies
            ///  `GET_SOBJECTS`.
            const UPDATE_KEY_OPS = 0x0000400000000000;
            ///  Permission to update individual security objects' policies. This
            ///  permission allows updating RSA options, as well as Google access
            ///  reason policy (for use with Google EKM APIs) defined on the security
            ///  object itself. Implies `GET_SOBJECTS`.
            const UPDATE_SOBJECT_POLICIES = 0x0000800000000000;
            ///  Permission to update name, description, custom metadata, key links
            ///  (currently only create parent link), and publish public key settings
            ///  of security objects. Implies `GET_SOBJECTS`.
            const UPDATE_SOBJECTS_PROFILE = 0x0001000000000000;
            ///  Permission to scan for security objects in external HSM/KMS. Implies
            ///  `GET_SOBJECTS`.
            const SCAN_EXTERNAL_SOBJECTS = 0x0002000000000000;
            ///  Permission to restore key material of security objects backed by
            ///  external HSM/KMS. Note that calling the Restore API needs this
            ///  permission in the destination group as well as `EXPORT_SOBJECTS`
            ///  permission in the source group (where the object was copied from
            ///  originally). Implies `GET_SOBJECTS`.
            const RESTORE_EXTERNAL_SOBJECTS = 0x0004000000000000;
            ///  Permission to call Workspace CSE Wrap API.
            const WRAP_WORKSPACE_CSE = 0x0008000000000000;
            ///  Permission to call Workspace CSE Unwrap API.
            const UNWRAP_WORKSPACE_CSE = 0x0010000000000000;
            const WORKSPACE_CSE = 0x0020000000000000;
            ///  Permission to get information about the group.
            const GET_GROUP = 0x0040000000000000;
            ///  Permission to get security objects stored in the group.
            const GET_SOBJECTS = 0x0080000000000000;
            ///  Permission to get cryptographic apps in the group.
            const GET_APPS = 0x0100000000000000;
            ///  Permission to get plugin in the group.
            const GET_PLUGINS = 0x0200000000000000;
            ///  Permission to get approval requests related to the group.
            const GET_GROUP_APPROVAL_REQUESTS = 0x0400000000000000;
            ///  Permission to get audit logs related to the group.
            const GET_AUDIT_LOGS = 0x0800000000000000;
            ///  Permission to update or remove wrapping key of the  group
            const MANAGE_GROUP_WRAPPING_KEY = 0x1000000000000000;
            ///  Permission to encapsulate security objects. Implies `CREATE_SOBJECTS`.
            const ENCAPSULATE_SOBJECTS = 0x2000000000000000;
            ///  Permission to decapsulate security objects. Implies `CREATE_SOBJECTS`.
            const DECAPSULATE_SOBJECTS = 0x4000000000000000;
        }
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub enum GroupSort {
    ByGroupId {
        order: Order,
        #[serde(skip_serializing_if = "Option::is_none")]
        previous_id: Option<Uuid>
    },
    ByGroupName {
        order: Order,
        #[serde(skip_serializing_if = "Option::is_none")]
        previous_sort_value: Option<String>
    }
}

impl UrlEncode for GroupSort {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        match *self {
            GroupSort::ByGroupId{ ref order, ref previous_id } => {
                m.insert("sort".to_string(), format!("group_id:{}", order));
                if let Some(v) = previous_id {
                    m.insert("previous_id".to_string(), v.to_string());
                }
            }
            GroupSort::ByGroupName{ ref order, ref previous_sort_value } => {
                m.insert("sort".to_string(), format!("group_name:{}", order));
                if let Some(v) = previous_sort_value {
                    m.insert("previous_sort_value".to_string(), v.to_string());
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct HistoryItem {
    pub id: Uuid,
    pub state: HistoryItemState,
    pub created_at: Time,
    pub expiry: Time
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct HistoryItemState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_date: Option<Time>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activation_undo_window: Option<Secs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_reason: Option<RevocationReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compromise_date: Option<Time>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deactivation_date: Option<Time>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deactivation_undo_window: Option<Secs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destruction_date: Option<Time>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_date: Option<Time>,
    pub state: SobjectState,
    pub key_ops: KeyOperations,
    pub public_only: bool,
    pub has_key: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_policy: Option<RotationPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<Uuid>
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct HmacOptionsPolicy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_key_length: Option<u32>
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct KcdsaOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subgroup_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_alg: Option<DigestAlgorithm>
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct KcdsaOptionsPolicy {

}

/// Methods for calculating a Key Checksum Value.
#[derive(Copy, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub enum KcvMethod {
    /// This is calculated by encrypting an all-zero block using the key
    /// and taking the leftmost 24 bits of the output.
    Encrypt,
    /// This is calculated by computing the CMAC (Cipher-based Message Authentication Code)
    /// on an all-zero block using the key, then taking the leftmost 40 bits
    /// of the output.
    Cmac
}

#[derive(Copy, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct KeyHistoryPolicy {
    pub undo_time_window: Secs
}

/// Linked security objects.
#[derive(PartialEq, Eq, Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KeyLinks {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement: Option<Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replaced: Option<Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copied_from: Option<Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copied_to: Option<Vec<Uuid>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subkeys: Option<Vec<Uuid>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<Uuid>,
    /// Wrapping key used to wrap this security object
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wrapping_key: Option<Uuid>
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct KeyMetadataPolicy {
    /// Applies to all objects.
    pub base: MetadataPolicyItem,
    /// Each entry in this map fully overrides `base` for a particular object type.
    pub for_obj_type: HashMap<ObjectType,MetadataPolicyItem>,
    /// What to do with legacy objects that are not compliant with this policy.
    /// Note that objects are not allowed to be created/updated if the result is
    /// not compliant with the policy. Non-compliant legacy objects can only be
    /// updated to comply with the policy (e.g. by adding missing required metadata).
    pub legacy_objects: LegacyKeyPolicy
}

/// Operations allowed to be performed on a given key.
pub use self::key_operations::KeyOperations;
pub mod key_operations {
    bitflags_set!{
        pub struct KeyOperations: u64 {
            ///  If this is set, the key can be used to for signing.
            const SIGN = 0x0000000000000001;
            ///  If this is set, the key can used for verifying a signature.
            const VERIFY = 0x0000000000000002;
            ///  If this is set, the key can be used for encryption.
            const ENCRYPT = 0x0000000000000004;
            ///  If this is set, the key can be used for decryption.
            const DECRYPT = 0x0000000000000008;
            ///  If this is set, the key can be used wrapping other keys.
            ///  The key being wrapped must have the EXPORT operation enabled.
            const WRAPKEY = 0x0000000000000010;
            ///  If this is set, the key can be used to unwrap a wrapped key.
            const UNWRAPKEY = 0x0000000000000020;
            ///  If this is set, the key can be used to derive another key.
            const DERIVEKEY = 0x0000000000000040;
            ///  If this is set, the key can be transformed.
            const TRANSFORM = 0x0000000000000080;
            ///  If this is set, the key can be used to compute a cryptographic
            ///  Message Authentication Code (MAC) on a message.
            const MACGENERATE = 0x0000000000000100;
            ///  If they is set, the key can be used to verify a MAC.
            const MACVERIFY = 0x0000000000000200;
            ///  If this is set, the value of the key can be retrieved
            ///  with an authenticated request. This shouldn't be set unless
            ///  required. It is more secure to keep the key's value inside DSM only.
            const EXPORT = 0x0000000000000400;
            ///  Without this operation, management operations like delete, destroy,
            ///  rotate, activate, restore, revoke, revert, update, remove_private, etc.
            ///  cannot be performed by a crypto App.
            ///  A user with access or admin app can still perform these operations.
            ///  This option is only relevant for crypto apps.
            const APPMANAGEABLE = 0x0000000000000800;
            ///  If this is set, audit logs will not be recorded for the key.
            ///   High volume here tries to signify a key that is being used a lot
            ///   and will produce lots of logs. Setting this operation disables
            ///   audit logs for the key.
            const HIGHVOLUME = 0x0000000000001000;
            ///  If this is set, the key can be used for key agreement.
            ///  Both the private and public key should have this option enabled
            ///  to perform an agree operation.
            const AGREEKEY = 0x0000000000002000;
            ///  If this is set, the key can be used for key encapsulation. The
            ///  result is a new symmetric key and a ciphertext.
            const ENCAPSULATE = 0x0000000000004000;
            ///  If this is set, the key can be used for key decapsulation. If
            ///  decapsulation succeeds, the result is a new symmetric key.
            const DECAPSULATE = 0x0000000000008000;
        }
    }
}

#[derive(Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, Clone)]
pub struct KeyOpsOverride {
    /// The operations to add to any key creation request (only supported in KMIP).
    ///
    /// The following operations can be specified:
    /// - `EXPORT`
    /// - `APPMANAGEABLE`
    /// - `HIGHVOLUME`
    ///
    /// The operations specified cannot conflict with what's specified in the
    /// `key_ops` field of account and/or group policies (where applicable).
    ///
    /// **Note**: This is only enforced on (KMIP) creation requests since we assume
    /// updates removing key operations are intentional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_key_ops: Option<KeyOperations>
}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct KmipClientConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_unknown_key_ops_for_secrets: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_ops_override: Option<KeyOpsOverride>
}

/// Role of a user or app in an account for the purpose of LDAP configurations.
#[derive(Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum LdapAccountRole {
    Legacy (
        LegacyLdapAccountRole
    ),
    Custom (
        Uuid
    )
}

/// LDAP authorization settings.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct LdapAuthorizationConfig {
    /// Number of seconds after which the authorization should be checked again.
    pub valid_for: u64,
    /// A map from account roles to distinguished names of LDAP groups.
    /// If a DN is specified for an account role, entities with that role
    /// must be a member of the specified LDAP group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_role: Option<HashMap<LdapAccountRole,String>>,
    /// User self-provisioning settings for the LDAP integration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_self_provisioning: Option<LdapUserSelfProvisioningConfig>,
    /// How to resolve group role assignment conflicts for users authorized
    /// through LDAP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_conflict_resolution: Option<LdapRoleConflictResolution>
}

/// Distinguished Name (DN) resolution method. Given a user's email address, a DN resolution method
/// is used to find the user's DN in an LDAP directory.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case", tag = "method")]
pub enum LdapDnResolution {
    /// Transform the user email through a pattern to derive the DN.
    Construct {
        /// For example: "example.com" => "uid={},ou=users,dc=example,dc=com".
        domain_format: HashMap<String,String>
    },
    /// Search the directory using the LDAP `mail` attribute matching user's email.
    SearchByMail,
    /// Use email in place of DN. This method works with Active Directory if the userPrincipalName
    /// attribute is set for the user. https://docs.microsoft.com/en-us/windows/desktop/ad/naming-properties
    #[serde(rename = "upn")]
    UserPrincipalName
}

/// Controls how we resolve conflicting role assignments with LDAP authorization.
///
/// When users are authorized through LDAP, their DSM group memberships are
/// determined by their LDAP groups and the external role mappings created in
/// DSM. For example, if the user belongs to 3 LDAP groups A, B and C, and these
/// LDAP groups are mapped to DSM groups G1 and G2 in the following way:
/// - A -> G1 as "group auditor"
/// - B -> G1 as "group administrator"
/// - C -> G2 as "group administrator"
///   Then which role should be assigned to this user in G1?
///
/// The answer to this question used to be simple before the introduction of
/// custom user roles in DSM: we took the maximum of the roles. Note that the
/// legacy roles (group admin/auditor) formed a strict "more powerful than"
/// relation, i.e. group administrator is strictly more powerful than group
/// auditor (and same is true for legacy account roles). However, custom user
/// roles do not have that relationship anymore. Moreover, the legacy behavior
/// is not quite square with the role exclusivity rules either since the legacy
/// behavior can also be regarded as assigning multiple exclusive roles in the
/// same group.
///
/// After the introduction of custom user roles, we allow a user to have
/// multiple roles in one group as long as none of the roles are marked as
/// exclusive. That rule is easily enforceable in the user Invite API. With LDAP
/// authorization, the group memberships are computed dynamically when the
/// Select Account API is called and it is possible that we run into conflicting
/// role assignments due to user's LDAP group membership and current mappings
/// between external roles (i.e. LDAP groups) and DSM groups.
#[derive(Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum LdapRoleConflictResolution {
    /// In this mode (which cannot be selected for new LDAP integrations and is
    /// only meant for LDAP integrations that existed before custom roles), DSM
    /// rejects any external role mapping involving custom roles and in case of
    /// conflicting role assignments it takes the maximal legacy role.
    BackcompatLegacyRolesOnly,
    /// In case of a role conflict, all role assignments where the role is
    /// marked as exclusive are ignored and the rest are assigned to the user.
    /// Note that legacy roles are all marked as exclusive. For example:
    /// - LDAP group A is mapped to DSM group G1 with role R1
    /// - LDAP group B is mapped to DSM group G1 with role R2
    /// - LDAP group C is mapped to DSM group G1 with role R3
    /// - Role R2 is marked exclusive
    ///
    /// A user that belongs to LDAP groups A, B and C will become a member of
    /// DSM group G1 with role R1 + R3.
    DisregardExclusiveRoles
}

/// Credentials used by the service to authenticate itself to an LDAP server.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct LdapServiceAccount {
    pub dn: String,
    pub password: ZeroizedString
}

/// LDAP user self-provisioning settings. Currently, the only
/// setting available for configuration is the mapping from
/// LDAP users to DSM account roles.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct LdapUserSelfProvisioningConfig {
    /// The mapping that determines which roles will be assigned
    /// to self-provisioned users.
    pub role_assignment: LdapUserSelfProvisioningRole
}

/// A structure indicating how self-provisioned LDAP users will
/// be assigned account roles.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "$type")]
pub enum LdapUserSelfProvisioningRole {
    /// Map all self-provisioned users to a single specified account role.
    /// (Note that this setting only determines the role that a self-
    /// provisioned user starts with; an account admin can change any user's
    /// role at a later time.) A "state enabled" flag will be implicitly added,
    /// and any specified "pending invite" flag will be removed.
    Fixed {
        role: UserAccountFlags
    }
}

#[derive(Copy, Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum LegacyKeyPolicy {
    /// The key can be used for all purposes.
    Allowed,
    /// The key cannot be used for any crypto operations until it becomes compliant.
    Prohibited,
    /// The key can only be used for these crypto operations:
    /// - DECRYPT
    /// - VERIFY
    /// - MACVERIFY
    /// - UNWRAPKEY
    UnprotectOnly
}

/// Role of a user or app in an account for the purpose of LDAP configurations.
#[derive(Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LegacyLdapAccountRole {
    AdminUser,
    MemberUser,
    AuditorUser,
    AdminApp,
    CryptoApp
}

/// Legacy user account role
#[derive(Debug, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum LegacyUserAccountRole {
    AccountAdministrator,
    AccountMember,
    AccountAuditor
}

/// Legacy user group role
#[derive(Debug, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum LegacyUserGroupRole {
    GroupAuditor,
    GroupAdministrator
}

/// Legacy user group role name or custom role id
#[derive(Debug, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum LegacyUserGroupRoleOrRoleId {
    LegacyRole (
        LegacyUserGroupRole
    ),
    RoleId (
        Uuid
    )
}

/// LMS specific options
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct LmsOptions {
    /// The height of the top level tree. This field will be deprecated in v2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l1_height: Option<u32>,
    /// The height of the secondary tree. This field will be deprecated in v2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l2_height: Option<u32>,
    /// The hash function to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<DigestAlgorithm>,
    /// Heights of the trees in each level.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heights: Option<Vec<usize>>,
    /// Amount of bytes associated to each node (the 'm' parameter)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_size: Option<usize>
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct LmsOptionsPolicy {

}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filtered_count: Option<usize>
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MetadataDurationConstraint {
    Forbidden {

    },
    Required {
        /// If specified, the value (typically a date) is restricted to be in a
        /// range expressed in terms of duration with respect to some known point
        /// in time. For example, if we specify min = 30 days and max = 180 days
        /// for `deactivation_date`, then the user must specify a deactivation date
        /// that is within 30 and 180 days of security object's creation time.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        allowed_values: Option<RestrictedDuration>
    }
}

#[derive(Debug, PartialEq, Eq, Default, Serialize, Deserialize, Clone)]
pub struct MetadataPolicyItem {
    pub custom_metadata: HashMap<String,MetadataStringConstraint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<MetadataStringConstraint>,
    /// If a restricted duration is specified, it is enforced w.r.t object creation time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deactivation_date: Option<MetadataDurationConstraint>,
    /// If a restricted duration is specified, it is enforced w.r.t object creation time.
    /// NOTE: Specifying a minimum duration for this field may not be a good
    /// idea since it would not be possible to create a key and start using it
    /// immediately in the affected group(s).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_date: Option<MetadataDurationConstraint>
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MetadataStringConstraint {
    Forbidden {

    },
    Required {
        /// If set to `true`, the value must have a length > 0 after trimming
        /// leading and trailing whitespace characters.
        #[serde(skip_serializing_if = "Option::is_none")]
        non_empty_after_trim: Option<bool>,
        /// If not specified or empty, it will not impose any restrictions on the value.
        #[serde(skip_serializing_if = "Option::is_none")]
        allowed_values: Option<HashSet<String>>
    }
}

/// Params for Mfa challenge.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MfaChallengeParams {
    /// Protocol for the Mfa request. U2f is default
    /// for backcompat.
    pub protocol: MfaProtocol
}

impl UrlEncode for MfaChallengeParams {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        m.insert("protocol".to_string(), self.protocol.to_string());
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum MfaChallengeResponse {
    LegacyU2f (
        U2fMfaChallengeResponse
    ),
    Fido2 (
        Fido2MfaChallengeResponse
    )
}

/// A FIDO device that may be used for second factor authentication.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct MfaDevice {
    /// Name given to the FIDO device.
    pub name: String,
    /// Type of the device, should be either fido2 or u2f
    pub r#type: MfaDeviceType,
    /// Origin of the FIDO device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>
}

/// Type of MFA device
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub enum MfaDeviceType {
    U2f,
    Fido2
}

/// Protocols for MFA.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MfaProtocol {
    /// U2f protocol. (deprecated)
    U2f,
    /// FIDO2 protocol.
    Fido2
}

/// Specifies the Mask Generating Function (MGF) to use.
#[derive(Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Mgf {
    /// MGF1 algorithm
    Mgf1 {
        hash: DigestAlgorithm
    }
}

/// MGF policy.
#[derive(Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MgfPolicy {
    Mgf1 {
        #[serde(skip_serializing_if = "Option::is_none")]
        hash: Option<DigestAlgorithm>
    }
}

/// ML-DSA specific options
#[derive(Copy, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct MlDsaBetaOptions {
    pub param_set: MlDsaParamSet
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct MlDsaBetaOptionsPolicy {

}

/// ML-DSA parameter sets
#[derive(Copy, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub enum MlDsaParamSet {
    MlDsa44,
    MlDsa65,
    MlDsa87
}

/// ML-KEM specific options
#[derive(Copy, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct MlKemBetaOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_set: Option<MlKemParamSet>
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct MlKemBetaOptionsPolicy {

}

/// ML-KEM parameter sets
#[derive(Copy, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub enum MlKemParamSet {
    MlKem512,
    MlKem768,
    MlKem1024
}

/// Corresponds to the `display` parameter in
/// https://openid.net/specs/openid-connect-core-1_0.html#AuthRequest
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum OauthAuthParamDisplay {
    /// The Authorization Server SHOULD display the authentication and consent UI consistent with a full User Agent page view
    Page,
    /// The Authorization Server SHOULD display the authentication and consent UI consistent with a popup User Agent window.
    /// The popup User Agent window should be of an appropriate size for a login-focused dialog and should not obscure the entire window that it is popping up over.
    Popup,
    /// The Authorization Server SHOULD display the authentication and consent UI consistent with a device that leverages a touch interface.
    Touch,
    /// The Authorization Server SHOULD display the authentication and consent UI consistent with a "feature phone" type display.
    Wap
}

/// Corresponds to the `prompt` parameter in
/// https://openid.net/specs/openid-connect-core-1_0.html#AuthRequest
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum OauthAuthParamPrompt {
    /// The Authorization Server SHOULD prompt the End-User for reauthentication.
    /// If it cannot reauthenticate the End-User, it MUST return an error, typically login_required.
    Login,
    /// The Authorization Server MUST NOT display any authentication or consent user interface pages.
    /// An error is returned if an End-User is not already authenticated or the Client does not have pre-configured consent for the requested Claims or does not fulfill other conditions for processing the request.
    /// The error code will typically be login_required, interaction_required, or another code defined in Section 3.1.2.6.
    /// This can be used as a method to check for existing authentication and/or consent.
    None,
    /// The Authorization Server SHOULD prompt the End-User for consent before returning information to the Client.
    /// If it cannot obtain consent, it MUST return an error, typically consent_required.
    Consent,
    /// The Authorization Server SHOULD prompt the End-User to select a user account.
    /// This enables an End-User who has multiple accounts at the Authorization Server to select amongst the multiple accounts that they might have current sessions for.
    /// If it cannot obtain an account selection choice made by the End-User, it MUST return an error, typically account_selection_required.
    SelectAccount
}

/// Parameters for the OpenID Connect Authentication Request
/// https://openid.net/specs/openid-connect-core-1_0.html#AuthRequest
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct OauthAuthenticationParameters {
    /// Specifies whether the Authorization Server prompts the End-User for reauthentication and consent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<Vec<OauthAuthParamPrompt>>,
    /// Specifies how the Authorization Server displays the authentication and consent user interface pages to the End-User
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OauthAuthParamDisplay>,
    /// Specifies the allowable elapsed time in seconds since the last time the End-User was actively authenticated by the OP.
    /// If the elapsed time is greater than this value, the OP MUST attempt to actively re-authenticate the End-User
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_age: Option<u64>
}

/// OAuth scope.
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum OauthScope {
    App,
    OpenID,
    Email,
    Profile
}

#[derive(Copy, Debug, Serialize, Deserialize, Clone)]
pub struct ObjectCount {
    /// Count of the items in the collection matching the request
    pub count: u64,
    /// accuracy of the count returned
    pub count_accuracy: CountAccuracy
}

/// The origin of a security object - where it was created / generated.
#[derive(Copy, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub enum ObjectOrigin {
    FortanixHSM,
    Transient,
    External
}

/// Type of security object.
#[derive(Debug, Eq, PartialEq, Copy, Hash, EnumIter, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum ObjectType {
    Aes,
    Aria,
    Des,
    Des3,
    Seed,
    Rsa,
    Dsa,
    Ec,
    Kcdsa,
    EcKcdsa,
    Bip32,
    Bls,
    Opaque,
    Hmac,
    LedaBeta,
    Round5Beta,
    Secret,
    Lms,
    MlDsaBeta,
    MlKemBeta,
    Certificate,
    Pbe
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct OpaqueOptionsPolicy {

}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Pkcs11ClientConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fake_rsa_x9_31_keygen_support: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signing_aes_key_as_hmac: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact_key_ops: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prevent_duplicate_opaque_objects: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opaque_objects_are_not_certificates: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_concurrent_requests_per_slot: Option<usize>
}

/// Plugin code signing policy.
///
/// When a code signing policy is set, all requests to create new plugins or
/// update existing plugins (if updating the code) would need to provide a valid
/// signature.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct PluginCodeSigningPolicy {
    /// The public key(s) used to verify plugin code signatures.
    pub signing_keys: SigningKeys
}

/// A security principal.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Principal {
    App (
        Uuid
    ),
    User (
        Uuid
    ),
    Plugin (
        Uuid
    ),
    /// UserViaApp signifies a user authorizing some app to act on its behalf through OAuth.
    UserViaApp {
        user_id: Uuid,
        scopes: HashSet<OauthScope>
    },
    /// System signifies DSM itself performing certain actions, like automatic key scans.
    /// This cannot be used for things like approval requests or session creation.
    System,
    /// An unregistered user.
    UnregisteredUser
}

/// <https://www.w3.org/TR/webauthn-2/#dictionary-makecredentialoptions>
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeyCredentialCreationOptions {
    /// Additional relying party's attributes. See type level
    /// doc for more info.
    pub rp: PublicKeyCredentialEntity<PublicKeyCredentialRpEntity>,
    /// Additional user's attributes. See type level doc for
    /// more info.
    pub user: PublicKeyCredentialEntity<PublicKeyCredentialUserEntity>,
    /// A random base64url encoded string. This can be min 16 bytes
    /// and max 64 bytes.
    pub challenge: Base64<UrlSafe>,
    /// This member contains information about the desired properties of the
    /// credential to be created. The sequence is ordered from most preferred
    /// to least preferred.
    pub pub_key_cred_params: Vec<PublicKeyCredentialParameters>,
    /// The time for which response from the authenticator
    /// would be awaited. This should only be a hint as per the spec.
    /// This is in milliseconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,
    /// The existing creds mapped to the current user. This tells
    /// the authenticator to not create multiple creds for the same
    /// user.
    /// NOTE: This isn't for U2F authenticators. For that, `appidExclude`
    /// needs to be set instead.
    pub exclude_credentials: Vec<PublicKeyCredentialDescriptor>,
    /// The selection criteria that should be used for selecting
    /// an authenticator.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authenticator_selection: Option<AuthenticatorSelectionCriteria>,
    /// The way attestation should be conveyed to RP.
    /// See type level doc for more info.
    pub attestation: AttestationConveyancePreference,
    /// Registration extensions returns by DSM and should
    /// be used as inputs to `navigator.credentials.create()`.
    ///
    /// Extensions are optional and can be ignored by clients
    /// or authenticator. But as per the spec, if the extensions
    /// are ignored, response of extensions must be empty and
    /// if not ignored, then, response must not be empty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<AuthenticationExtensionsClientInputs>
}

/// Used to in registration response (telling about existing creds) to prevent
/// creation of duplicate creds on the same authenticator.
/// Used in authentication as the allowed creds.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct PublicKeyCredentialDescriptor {
    /// Type of credential.
    pub r#type: PublicKeyCredentialType,
    /// Credential ID of the public key credential the
    /// caller is referring to.
    pub id: Base64<UrlSafe>,
    /// Hints by relying party on what transport client should
    /// use to communicate with authenticator.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transports: Option<Vec<AuthenticatorTransport>>
}

/// https://www.w3.org/TR/webauthn-2/#dictionary-credential-params
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeyCredentialParameters {
    /// Type of credential.
    pub r#type: PublicKeyCredentialType,
    /// An algorithm from IANA COSE Algorithms registry supported
    /// by DSM as well.Upgrade to use this branch
    pub alg: COSEAlgorithmIdentifier
}

/// <https://www.w3.org/TR/webauthn-2/#dictionary-assertion-options>
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeyCredentialRequestOptions {
    /// This member contains the base64url encoding of the challenge
    /// provided by the Relying Party
    pub challenge: Base64<UrlSafe>,
    /// The time for which response from the authenticator
    /// would be awaited. This should only be a hint as per the spec.
    /// This is in milliseconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,
    /// This optional member specifies the relying party identifier
    /// claimed by the caller. If omitted, its value will be the
    /// CredentialsContainer object’s relevant settings object's
    /// origin's effective domain.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rp_id: Option<String>,
    /// This OPTIONAL member contains a list of [PublicKeyCredentialDescriptor]
    /// objects representing public key credentials acceptable to the caller,
    /// in descending order of the caller’s preference (the first item in the
    /// list is the most preferred credential, and so on down the list).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_credentials: Option<Vec<PublicKeyCredentialDescriptor>>,
    /// Authenticator should support user verification by
    /// ways like pin code, biometrics, etc.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_verification: Option<UserVerificationRequirement>,
    /// Authentication extensions returned by DSM and should
    /// be used as inputs to `navigator.credentials.get()`.
    ///
    /// Extensions are optional and can be ignored by clients
    /// or authenticator. But as per the spec, if the extensions
    /// are ignored, response of extensions must be empty and
    /// if not ignored, then, response must not be empty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<AuthenticationExtensionsClientInputs>
}

/// <https://www.w3.org/TR/webauthn-2/#dictionary-rp-credential-params>
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeyCredentialRpEntity {
    /// A unique identifier for the Relying Party entity, which sets the RP ID.
    ///
    /// <https://www.w3.org/TR/webauthn-2/#CreateCred-DetermineRpId>
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>
}

/// https://www.w3.org/TR/webauthn-2/#enum-credentialType
///
/// This enum defines valid cred types.
#[derive(Debug, Copy, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum PublicKeyCredentialType {
    /// Public key credential.
    PublicKey
}

/// <https://www.w3.org/TR/webauthn-2/#dictionary-user-credential-params>
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeyCredentialUserEntity {
    /// This is uuid of the user in DSM. But here, it is
    /// in base64url format as required by fido server conformance
    /// spec.
    pub id: Base64<UrlSafe>,
    /// Human friendly name intended only for display.
    pub display_name: String
}

/// If enabled, the public key will be available publicly (without authentication) through the GetPublicKey API.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", tag = "state")]
pub enum PublishPublicKeyConfig {
    Enabled {
        /// Additionally list the previous version of the key if not compromised.
        list_previous_version: bool
    },
    Disabled
}

/// Quorum approval policy.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Quorum {
    pub n: usize,
    pub members: Vec<QuorumPolicy>,
    #[serde(flatten)]
    pub config: ApprovalAuthConfig
}

/// Quorum Policy Configurations in clients
#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct QuorumApprovalConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wait_for_quorum_approval: Option<ApprovalWaitConfig>
}

/// Approval policy.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct QuorumPolicy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quorum: Option<Quorum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app: Option<Uuid>
}

/// <https://www.w3.org/TR/webauthn-2/#enum-residentKeyRequirement>
///
/// Tells Relying Party's requirement about client side discoverable
/// creds (formely known as resident keys).
/// If client side discoverable creds are there, it means that the
/// authenticator is self-sufficient in identifying the user. If this
/// isn't the case, the user needs to login first so that the server
/// can identify the user and help send `allowCredentials` to authenticator.
///
/// This is mostly meant for [username-less] authentication (which we don't
/// support in DSM). We support 2FA where we already know about the logged
/// in user.
///
/// [username-less]: <https://groups.google.com/a/fidoalliance.org/g/fido-dev/c/ALQj3JXuyhs>
#[derive(Debug, Copy, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum ResidentKeyRequirement {
    /// Indicates that the relying party "prefers"
    /// that client-side discoverable creds aren't
    /// created.
    Discouraged,
    /// Indicates that relying party prefers resident
    /// keys.
    Preferred,
    /// Indicates that relying party requires resident
    /// keys.
    Required
}

#[derive(Debug, PartialEq, Eq, Default, Serialize, Deserialize, Clone)]
pub struct RestrictedDuration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<TimeSpan>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<TimeSpan>
}

/// Reason for revoking a key.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct RevocationReason {
    pub code: RevocationReasonCode,
    /// Message is used exclusively for audit trail/logging purposes and MAY contain additional
    /// information about why the object was revoked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compromise_occurance_date: Option<Time>
}

/// Reasons to revoke a security object.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub enum RevocationReasonCode {
    Unspecified,
    KeyCompromise,
    CACompromise,
    AffiliationChanged,
    Superseded,
    CessationOfOperation,
    PrivilegeWithdrawn
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RotateCopiedKeys {
    AllExternal,
    Select (
        Vec<Uuid>
    )
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RotationInterval {
    IntervalDays (
        u32
    ),
    IntervalMonths (
        u32
    )
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct RotationPolicy {
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub interval: Option<RotationInterval>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_at: Option<Time>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deactivate_rotated_key: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotate_copied_keys: Option<RotateCopiedKeys>
}

/// Type of padding to use for RSA encryption. The use of PKCS#1 v1.5 padding is strongly
/// discouraged, because of its susceptibility to Bleichenbacher's attack. The padding specified
/// must adhere to the key's encryption policy. If not specified, the default based on the key's
/// policy will be used.
#[derive(Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RsaEncryptionPadding {
    /// Optimal Asymmetric Encryption Padding (PKCS#1 v2.1).
    Oaep {
        mgf: Mgf
    },
    /// PKCS#1 v1.5 padding. This is disallowed in FIPS builds.
    Pkcs1V15 {

    },
    /// RSA decryption without padding. (Raw RSA encryption is not supported)
    RawDecrypt {

    }
}

/// RSA encryption padding policy.
#[derive(Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RsaEncryptionPaddingPolicy {
    /// The Optional Asymmetric Encryption Padding scheme, as defined
    /// in RFC 8017 (PKCS #1 version 2.2)
    Oaep {
        #[serde(skip_serializing_if = "Option::is_none")]
        mgf: Option<MgfPolicy>
    },
    /// PKCS #1 version 1.5 encryption padding. This is disallowed in
    /// FIPS builds
    Pkcs1V15 {

    },
    /// Raw RSA decryption
    RawDecrypt {

    }
}

/// Constraints on RSA encryption parameters. In general, if a constraint is not specified, anything is allowed.
#[derive(Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct RsaEncryptionPolicy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<RsaEncryptionPaddingPolicy>
}

/// RSA-specific options.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct RsaOptions {
    /// Size in bits (not bytes) of the RSA key. Specify on Create only. Returned on Get.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_size: Option<u32>,
    /// Public exponent to use for generating the RSA key. Specify on Create only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_exponent: Option<u32>,
    /// Encryption policy for an RSA key. When doing an encryption or key wrapping operation, the
    /// policies are evaluated against the specified parameters one by one. If one matches, the
    /// operation is allowed. If none match, including if the policy list is empty, the operation
    /// is disallowed. Missing optional parameters will have their defaults specified according to
    /// the matched policy. The default for new keys is `[{"padding":{"OAEP":{}}]`.
    /// If (part of) a constraint is not specified, anything is allowed for that constraint.
    /// To impose no constraints, specify `[{}]`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_policy: Option<Vec<RsaEncryptionPolicy>>,
    /// Signature policy for an RSA key. When doing a signature operation, the policies are
    /// evaluated against the specified parameters one by one. If one matches, the operation is
    /// allowed. If none match, including if the policy list is empty, the operation is disallowed.
    /// Missing optional parameters will have their defaults specified according to the matched
    /// policy. The default for new keys is `[{}]` (no constraints).
    /// If (part of) a constraint is not specified, anything is allowed for that constraint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_policy: Option<Vec<RsaSignaturePolicy>>,
    /// The minimum allowed key length. This is only relevant for group or account cryptographic
    /// policies (and hence has no effect in an RSA policy on a specific key).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_key_length: Option<u32>
}

#[derive(PartialEq, Eq, Debug, Default, Serialize, Deserialize, Clone)]
pub struct RsaOptionsPolicy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_policy: Option<Vec<RsaEncryptionPolicy>>,
    /// Signature policy for an RSA key. When doing a signature operation, the policies are
    /// evaluated against the specified parameters one by one. If one matches, the operation is
    /// allowed. If none match, including if the policy list is empty, the operation is disallowed.
    /// Missing optional parameters will have their defaults specified according to the matched
    /// policy. The default for new keys is `[{}]` (no constraints).
    /// If (part of) a constraint is not specified, anything is allowed for that constraint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_policy: Option<Vec<RsaSignaturePolicy>>,
    /// The minimum allowed key length. This is only relevant for group or account cryptographic
    /// policies (and hence has no effect in an RSA policy on a specific key).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_key_length: Option<u32>
}

/// RSA signature padding policy.
#[derive(Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RsaSignaturePaddingPolicy {
    Pss {
        #[serde(skip_serializing_if = "Option::is_none")]
        mgf: Option<MgfPolicy>
    },
    Pkcs1V15 {

    }
}

/// Constraints on RSA signature parameters. In general, if a constraint is not specified, anything is allowed.
#[derive(Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct RsaSignaturePolicy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<RsaSignaturePaddingPolicy>
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct SecretOptionsPolicy {

}

pub type Secs = u64;

#[derive(PartialEq, Eq, Debug, Default, Serialize, Deserialize, Clone)]
pub struct SeedOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cipher_mode: Option<CipherMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_iv: Option<bool>
}

#[derive(PartialEq, Eq, Debug, Default, Serialize, Deserialize, Clone)]
pub struct SeedOptionsPolicy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_iv: Option<bool>
}

/// Signing keys used to validate JSON Web Signature objects including signed
/// JSON Web Tokens.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase", tag = "kind")]
pub enum SigningKeys {
    Stored {
        /// Mapping key ids to DER-encoded public key.
        keys: HashMap<String,ZeroizedBlob>
    },
    Fetched {
        url: String,
        /// Number of seconds that the service is allowed to cache the fetched keys.
        cache_duration: u64
    }
}

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Sobject {
    /// UUID of the account which the security object belongs to.
    pub acct_id: Uuid,
    /// Activation date of security object in seconds since EPOCH.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activation_date: Option<Time>,
    /// AES specific options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aes: Option<AesOptions>,
    /// Whether the sign operation response contains hash or data as output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sign_hash: Option<bool>,
    /// ARIA specific options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aria: Option<AriaOptions>,
    /// BIP32 specific options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bip32: Option<Bip32Options>,
    /// BLS specific options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bls: Option<BlsOptions>,
    /// Whether this security object is compliant with cryptographic policies or not.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compliant_with_policies: Option<bool>,
    /// Compromise date of security object in seconds since EPOCH.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compromise_date: Option<Time>,
    /// Timestamp at which the security object was created.
    pub created_at: Time,
    /// DSM entity which created the security object.
    pub creator: Principal,
    /// User managed field for adding custom metadata to the security object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_metadata: Option<HashMap<String,String>>,
    /// Deactivation date of security object in seconds since EPOCH.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deactivation_date: Option<Time>,
    /// Deletion date of security object in seconds since EPOCH.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletion_date: Option<Time>,
    /// DES specific options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub des: Option<DesOptions>,
    /// DES3 specific options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub des3: Option<Des3Options>,
    /// Description of the security object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Destruction date of security object in seconds since EPOCH.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destruction_date: Option<Time>,
    /// Optionally get deterministic signatures, if algorithm is EC or RSA.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deterministic_signatures: Option<bool>,
    /// DSA specific options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dsa: Option<DsaOptions>,
    /// ECKCDSA specific options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eckcdsa: Option<EcKcdsaOptions>,
    /// An aggregation of policies and permissions of the session creator for a security object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_key_policy: Option<EffectiveKeyPolicy>,
    /// Identifies a standard elliptic curve.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elliptic_curve: Option<EllipticCurve>,
    /// Whether this security object has cryptographic operations enabled.
    pub enabled: bool,
    /// This export policy determines how exportable keys (ones with the `EXPORT` permission) may be exported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub export_policy: Option<SobjectExportPolicy>,
    /// Information specific to an external KMS. Currently, it only has AWS related information.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external: Option<ExternalSobjectInfo>,
    /// FPE specific options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fpe: Option<FpeOptions>,
    /// Key Access Justifications for GCP EKM.
    /// For more details: https://cloud.google.com/cloud-provider-access-management/key-access-justifications/docs/overview
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub google_access_reason_policy: Option<GoogleAccessReasonPolicy>,
    /// When a Key Undo Policy is in place, a list of (non-expired) history items is returned.
    /// Each history item represents a past key state that can be reverted to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<HistoryItem>>,
    /// KCDSA specific options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kcdsa: Option<KcdsaOptions>,
    /// Key Checksum Value of the security object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kcv: Option<String>,
    /// Cmac Key Checksum Value of the security object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kcv_cmac: Option<String>,
    /// Operations allowed to be performed by a given key.
    pub key_ops: KeyOperations,
    /// Key size of the security object in bits.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_size: Option<u32>,
    /// Unique identifier of the security object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kid: Option<Uuid>,
    /// Linked security objects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<KeyLinks>,
    /// LMS specific options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lms: Option<LmsOptions>,
    /// ML-DSA specific options (beta).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mldsa_beta: Option<MlDsaBetaOptions>,
    /// ML-KEM specific options (beta).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mlkem_beta: Option<MlKemBetaOptions>,
    /// Name of the security object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether the security object was exportable at some point in its lifetime.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub never_exportable: Option<bool>,
    /// Type of security object.
    pub obj_type: ObjectType,
    /// The origin of the security object.
    pub origin: ObjectOrigin,
    /// Public key material of the security object, if it exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pub_key: Option<ZeroizedBlob>,
    /// Whether the security object only consists of public material.
    pub public_only: bool,
    /// If enabled, the public key will be available publicly (without authentication)
    /// through the GetPublicKey API.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish_public_key: Option<PublishPublicKeyConfig>,
    /// Revocation reason for compromised security object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revocation_reason: Option<RevocationReason>,
    /// Rotation policy of security objects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotation_policy: Option<RotationPolicy>,
    /// RSA specific options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsa: Option<RsaOptions>,
    /// Timestamp at which security object will be rotated, if rotation policy exists.
    /// This time will be clamped at 31 December 9999 11:59:59 pm UTC if the calculated
    /// rotation time would exceed that date.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduled_rotation: Option<Time>,
    /// Seed options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seed: Option<SeedOptions>,
    /// Security object operational state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<SobjectState>,
    /// Transient key material.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transient_key: Option<Blob>,
    /// Security object stored as byte array.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<ZeroizedBlob>,
    /// Metadata specific to the virtual key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub virtual_key_info: Option<VirtualSobjectInfo>,
    /// Group ids of groups that use this security object to encrypt the key material of their security objects
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wrapping_key_group_ids: Option<HashSet<Uuid>>,
    /// UUID of the group which the security object belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<Uuid>
}

/// Uniquely identifies a persisted or transient sobject.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SobjectDescriptor {
    Kid (
        Uuid
    ),
    Name (
        String
    ),
    TransientKey (
        Blob
    ),
    Inline {
        value: ZeroizedBlob,
        obj_type: ObjectType
    }
}

/// Uniquely identifies a persisted sobject.
#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SobjectDescriptorPersisted {
    Kid (
        Uuid
    ),
    Name (
        String
    )
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct SobjectExportPolicy {
    #[serde(flatten)]
    pub export_policy: ExportPolicy,
    /// When set to `false`, the `export_policy` for the sobject can never be relaxed,
    /// it can only be made more strict.
    /// This means that
    ///  - If the export policy is set to `Wrapped`, it can never be set back to `Unrestricted`.
    ///  - If the export policy is set to `Wrapped` with a limited set of keys specified,
    ///    it can never bet set back to `Wrapped` with any key. Additionally, no new keys can
    ///    be added to the set of wrapping keys. Also note that if all the wrapping keys (specified
    ///    by key id) in the export policy have been deleted/destroyed/deactivated, the sobject
    ///    becomes effectively unexportable.
    ///
    /// Note: these rules may change in the future.
    pub allow_weakening: bool
}

/// Security object operational state.
#[derive(Debug, Eq, PartialEq, Copy, Serialize, Deserialize, Clone)]
pub enum SobjectState {
    /// The security object exists but can not be used for any cryptographic purpose
    /// until it's activated.
    PreActive,
    /// The security object can be used for any cryptographic purpose.
    Active,
    /// The security object can not be used for applying cryptographic protection,
    /// but can be used for processing cryptographically protected information.
    /// Key must be in the activated state in order to transition to the deactivated state.
    Deactivated,
    /// The security object can not be used for applying cryptographic protection
    /// but can be used for processing cryptographically protected information.
    Compromised,
    /// The security object can not perform any cryptographic operations, as the
    /// key material gets deleted.
    Destroyed,
    /// The security object does not exist in DSM. However, its compromised status
    /// is retained for audit and security purposes.
    Deleted
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct TepClientConfig {
    pub schema: TepSchema,
    pub key_map: TepKeyMapList
}

#[derive(Debug, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TepKeyContext {
    Request,
    Response
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct TepKeyMap {
    pub path: ApiPath,
    pub kid: Uuid,
    pub mode: CipherMode
}

pub type TepKeyMapList = Vec<TepKeyMap>;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "$type")]
pub enum TepSchema {
    OpenAPI (
        Box<OpenAPI>
    )
}

#[derive(Debug, Copy, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TimeSpan {
    Seconds (
        u32
    ),
    Minutes (
        u32
    ),
    Hours (
        u32
    ),
    Days (
        u32
    )
}

/// TLS client settings.
#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", tag = "mode")]
pub enum TlsConfig {
    Disabled,
    Opportunistic,
    Required {
        validate_hostname: bool,
        ca: CaConfig,
        #[serde(skip_serializing_if = "Option::is_none")]
        client_key: Option<ZeroizedBlob>,
        #[serde(skip_serializing_if = "Option::is_none")]
        client_cert: Option<Blob>
    }
}

/// Use of U2F is deprecated, use FIDO2 for second factor authentication.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct U2fAuthRequest {
    pub key_handle: Blob,
    pub signature_data: Blob,
    pub client_data: Blob
}

/// A challenge used for multi-factor authentication.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct U2fMfaChallengeResponse {
    pub u2f_challenge: String,
    pub u2f_keys: Vec<U2fRegisteredKey>
}

/// Description of a registered U2F device.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct U2fRegisteredKey {
    pub key_handle: String,
    pub version: String
}

/// User account flag
#[derive(Debug, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum UserAccountFlag {
    StateEnabled,
    PendingInvite
}

/// User account flag or legacy user account role name or custom role id
#[derive(Debug, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum UserAccountFlagOrRole {
    Flag (
        UserAccountFlag
    ),
    LegacyRole (
        LegacyUserAccountRole
    ),
    RoleId (
        Uuid
    )
}

/// User's role(s) and state in an account.
pub type UserAccountFlags = HashSet<UserAccountFlagOrRole>;

/// User's role(s) in a group.
pub type UserGroupRole = HashSet<LegacyUserGroupRoleOrRoleId>;

/// https://www.w3.org/TR/webauthn-2/#enum-userVerificationRequirement
/// https://www.w3.org/TR/webauthn-2/#user-verification
#[derive(Debug, Copy, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum UserVerificationRequirement {
    /// Indicates the requirement of UV by RP and op
    /// fails if this wasn't satisfied.
    Required,
    /// UV is preferred by the RP but op won't fail
    /// if it isn't satisfied.
    Preferred,
    /// UV isn't "preferred" by RP.
    Discouraged
}

/// Information specific to a virtual key. Currently, this is only relevant
/// for virtual keys backed by DSM.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct VirtualSobjectInfo {
    /// Whether or not the source key material is cached within the key.
    pub cached_key_material: bool
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "$type")]
pub enum WrappingKeys {
    /// Only keys in this list can be used to wrap the sobject
    Only {
        keys: HashSet<SobjectDescriptorPersisted>
    },
    /// Any key can be used to wrap the sobject
    Any
}

