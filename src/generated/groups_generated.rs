/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

/// AWS KMS resources are hosted in multiple locations world-wide and
/// each AWS Region is a separate geographic area
/// https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Concepts.RegionsAndAvailabilityZones.html
#[derive(Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone)]
pub enum AwskmsRegion {
    #[serde(rename = "us-east-1")]
    UsEast1,
    #[serde(rename = "us-east-2")]
    UsEast2,
    #[serde(rename = "us-west-1")]
    UsWest1,
    #[serde(rename = "us-west-2")]
    UsWest2,
    #[serde(rename = "af-south-1")]
    AfSouth1,
    #[serde(rename = "ap-east-1")]
    ApEast1,
    #[serde(rename = "ap-southeast-3")]
    ApSoutheast3,
    #[serde(rename = "ap-southeast-4")]
    ApSoutheast4,
    #[serde(rename = "ap-south-1")]
    ApSouth1,
    #[serde(rename = "ap-south-2")]
    ApSouth2,
    #[serde(rename = "ap-northeast-3")]
    ApNortheast3,
    #[serde(rename = "ap-northeast-2")]
    ApNortheast2,
    #[serde(rename = "ap-southeast-1")]
    ApSoutheast1,
    #[serde(rename = "ap-southeast-2")]
    ApSoutheast2,
    #[serde(rename = "ap-northeast-1")]
    ApNortheast1,
    #[serde(rename = "ca-central-1")]
    CaCentral1,
    #[serde(rename = "ca-west-1")]
    CaWest1,
    #[serde(rename = "eu-central-1")]
    EuCentral1,
    #[serde(rename = "eu-central-2")]
    EuCentral2,
    #[serde(rename = "eu-west-1")]
    EuWest1,
    #[serde(rename = "eu-west-2")]
    EuWest2,
    #[serde(rename = "eu-south-1")]
    EuSouth1,
    #[serde(rename = "eu-south-2")]
    EuSouth2,
    #[serde(rename = "eu-west-3")]
    EuWest3,
    #[serde(rename = "eu-north-1")]
    EuNorth1,
    #[serde(rename = "me-south-1")]
    MeSouth1,
    #[serde(rename = "me-central-1")]
    MeCentral1,
    #[serde(rename = "sa-east-1")]
    SaEast1,
    #[serde(rename = "us-gov-east-1")]
    UsGovEast1,
    #[serde(rename = "us-gov-west-1")]
    UsGovWest1,
    #[serde(rename = "il-central-1")]
    IlCentral1
}

/// Specifies the AWS service. Only `kms` is supported for now.
#[derive(Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum AwskmsService {
    Kms,
    KmsFips
}

#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", tag = "mode")]
pub enum AzureAuthConfig {
    ClientSecret {
        client_secret: ZeroizedString
    },
    TokenAuthConfig {
        client_cert: ZeroizedBlob,
        client_key: ZeroizedBlob
    }
}

/// Types of Azure Key Vault based on the protection level.
#[derive(Debug, Eq, PartialEq, Copy, Hash, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum AzureKeyVaultType {
    /// Software-protected
    Standard,
    /// HSM-protected (with Premium SKU)
    Premium,
    /// Azure Managed HSM
    Managed
}

/// The set of endpoints to use when connecting with Azure cloud.
///
/// Today, only Azure global and Azure Government cloud endpoints are supported,
/// and they cannot be mixed together. The Azure global endpoints are
/// - `management`: management.azure.com
/// - `key_vault`: vault.azure.net
/// - `key_vault_managed_hsm`: managedhsm.azure.net
/// - `iam`: login.microsoftonline.com
///
/// and the Azure Government endpoints are
/// - `management`: management.usgovcloudapi.net
/// - `key_vault`: vault.usgovcloudapi.net
/// - `key_vault_managed_hsm`: managedhsm.usgovcloudapi.net
/// - `iam`: login.microsoftonline.us
///
/// (In the future, this restriction may be relaxed to support custom clouds.)
#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Clone)]
pub struct AzureServiceEndpoints {
    /// The API endpoint for managing Azure APIs and resources.
    pub management: String,
    /// The API endpoint for Azure Key Vault (for Standard and Premium SKUs).
    pub key_vault: String,
    /// The API endpoint for Azure Key Vault Managed HSM.
    pub key_vault_managed_hsm: String,
    /// The API endpoint for Azure AD (and authentication).
    pub iam: String
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CheckHmgRequest {
    /// The ID of the hmg configuration in the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<HmgConfig>
}

#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone)]
pub struct GcpKeyRingConfig {
    /// Email for the service account to be used.
    pub service_account_email: String,
    /// The project ID is a unique identifier for a project
    pub project_id: String,
    /// For a given project in GCP KMS, resources can be created in one of many locations.
    /// These represent the geographical regions where a resource is stored and can be accessed.
    /// A key's location impacts the performance of applications using the key.
    /// https://cloud.google.com/kms/docs/locations
    pub location: String,
    /// A key ring organizes keys in a specific GCP location and allows you to manage
    /// access control on groups of keys.
    /// https://cloud.google.com/kms/docs/resource-hierarchy#key_rings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_ring: Option<String>,
    /// Private component of the service account key pair that can be
    /// obtained from the GCP cloud console. It is used to authenticate
    /// the requests made by DSM to the GCP cloud.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<ZeroizedBlob>
}

/// Information about a group's recent scans.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct GetAllHmgScansResponse {
    /// List of all tracked scans, from newest to oldest.
    pub items: Vec<Scan>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Group {
    pub acct_id: Uuid,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_policy: Option<GroupApprovalPolicy>,
    /// Settings for automatic key scanning. For now, this is only available for DSM-backed groups.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_scan: Option<AutoScanSettings>,
    pub client_configurations: ClientConfigurations,
    pub created_at: Time,
    pub creator: Principal,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cryptographic_policy: Option<CryptographicPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custodian_policy: Option<QuorumPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_metadata: Option<HashMap<String,String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Export policy that applies to exportable sobjects (ones with `EXPORT` key op) in the group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub export_policy: Option<ExportPolicy>,
    /// Key Access Justifications for GCP EKM.
    /// For more details: https://cloud.google.com/cloud-provider-access-management/key-access-justifications/docs/overview
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub google_access_reason_policy: Option<GoogleAccessReasonPolicy>,
    pub group_id: Uuid,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hmg: Option<HashMap<Uuid,HmgConfig>>,
    /// The `HmgRedundancyScheme` to set for the group. If unset, the backend will assign no particular meaning to the `hsm_order` fields of the group's `HmgConfig`s, and may error if it cannot connect to the external HSMs or DSM nodes specified in one of the `HmgConfig`s.
    /// 
    /// When creating the group, the value should either be an `HmgRedundancyScheme`, or omitted entirely. When updating the group, there are three choices:
    /// - A new value can be set by providing an `HmgRedundancyScheme`.
    /// - The string "remove" can be specified to unset the field.
    /// - Simply leaving the field blank leaves the field unchanged.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hmg_redundancy: Option<HmgRedundancyScheme>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hmg_segregation: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hmg_sync: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_history_policy: Option<KeyHistoryPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_metadata_policy: Option<KeyMetadataPolicy>,
    pub name: String,
    /// Name of an AES key from another group. The key will be used to encrypt the key material of all keys in this group
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wrapping_key_name: Option<WrappingKeyName>
}

/// Group approval policy.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct GroupApprovalPolicy {
    #[serde(flatten)]
    pub policy: QuorumPolicy,
    /// Deprecated, left this for backward compatibility.
    /// When this is true, manage operations on security objects require approval.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protect_manage_operations: Option<bool>,
    /// Use QuorumGroupPermissions to represent operations that require approval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_permissions: Option<QuorumGroupPermissions>,
    /// When this is true, cryptographic operations on security objects require approval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_crypto_operations: Option<bool>
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct GroupRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_hmg: Option<Vec<HmgConfig>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_policy: Option<GroupApprovalPolicy>,
    /// Settings for automatic key scanning. For now, this is only available for DSM-backed groups.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_scan: Option<Removable<AutoScanSettings>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_configurations: Option<ClientConfigurationsRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cryptographic_policy: Option<Removable<CryptographicPolicy>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custodian_policy: Option<QuorumPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_metadata: Option<HashMap<String,String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub del_hmg: Option<HashSet<Uuid>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Export policy that applies to exportable sobjects (ones with `EXPORT` key op) in the group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub export_policy: Option<ExportPolicy>,
    /// Key Access Justifications for GCP EKM.
    /// For more details: https://cloud.google.com/cloud-provider-access-management/key-access-justifications/docs/overview
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub google_access_reason_policy: Option<Removable<GoogleAccessReasonPolicy>>,
    /// The `HmgRedundancyScheme` to set for the group. If unset, the backend will assign no particular meaning to the `hsm_order` fields of the group's `HmgConfig`s, and may error if it cannot connect to the external HSMs or DSM nodes specified in one of the `HmgConfig`s.
    /// 
    /// When creating the group, the value should either be an `HmgRedundancyScheme`, or omitted entirely. When updating the group, there are three choices:
    /// - A new value can be set by providing an `HmgRedundancyScheme`.
    /// - The string "remove" can be specified to unset the field.
    /// - Simply leaving the field blank leaves the field unchanged.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hmg_redundancy: Option<Removable<HmgRedundancyScheme>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hmg_segregation: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hmg_sync: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_history_policy: Option<Removable<KeyHistoryPolicy>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_metadata_policy: Option<Removable<KeyMetadataPolicy>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mod_hmg: Option<HashMap<Uuid,HmgConfig>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Name of an AES key from another group. The key will be used to encrypt the key material of all keys in this group
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wrapping_key_name: Option<WrappingKeyName>
}

#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind")]
pub enum HmgConfig {
    Ncipher {
        url: String,
        tls: TlsConfig,
        slot: usize,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pin: Option<ZeroizedString>,
        /// The priority of this `HmgConfig`. This is used when a group is
        /// configured with an `HmgRedundancyScheme`, and is otherwise
        /// unused. (See the docs for `HmgRedundancyScheme` for more
        /// information about the interpretation of this field.)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        hsm_order: Option<i32>
    },
    Safenet {
        url: String,
        tls: TlsConfig,
        slot: usize,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pin: Option<ZeroizedString>,
        /// The priority of this `HmgConfig`. This is used when a group is
        /// configured with an `HmgRedundancyScheme`, and is otherwise
        /// unused. (See the docs for `HmgRedundancyScheme` for more
        /// information about the interpretation of this field.)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        hsm_order: Option<i32>
    },
    AwsCloudHsm {
        url: String,
        tls: TlsConfig,
        slot: usize,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pin: Option<ZeroizedString>,
        /// The priority of this `HmgConfig`. This is used when a group is
        /// configured with an `HmgRedundancyScheme`, and is otherwise
        /// unused. (See the docs for `HmgRedundancyScheme` for more
        /// information about the interpretation of this field.)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        hsm_order: Option<i32>
    },
    AwsKms {
        url: String,
        tls: TlsConfig,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        access_key: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        secret_key: Option<ZeroizedString>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        region: Option<AwskmsRegion>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        service: Option<AwskmsService>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        account_id: Option<String>
    },
    Fortanix {
        url: String,
        tls: TlsConfig,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pin: Option<ZeroizedString>
    },
    FortanixFipsCluster {
        url: String,
        tls: TlsConfig,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pin: Option<ZeroizedString>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        credentials: Option<Vec<ZeroizedString>>,
        /// The priority of this `HmgConfig`. This is used when a group is
        /// configured with an `HmgRedundancyScheme`, and is otherwise
        /// unused. (See the docs for `HmgRedundancyScheme` for more
        /// information about the interpretation of this field.)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        hsm_order: Option<i32>
    },
    AzureKeyVault {
        url: String,
        tls: TlsConfig,
        auth_config: AzureAuthConfig,
        /// Deprecated, left this for backward compatibility. Should use auth_config.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        secret_key: Option<ZeroizedString>,
        /// A tenant ID is a unique way to identify an Azure AD instance
        /// within an Azure subscription.
        tenant_id: Uuid,
        /// The client ID is the unique Application ID assigned
        /// to your app by Azure AD when the app was registered.
        client_id: Uuid,
        /// A subscription ID is a unique alphanumeric string
        /// that identifies your Azure subscription.
        subscription_id: Uuid,
        /// Specifies the type of key vault to be configured.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        key_vault_type: Option<AzureKeyVaultType>,
        /// Which Azure endpoints to use. If not specified upon group creation or
        /// update, endpoints for (ordinary) Azure global cloud will be used.
        #[serde(skip_serializing_if = "Option::is_none")]
        endpoints: Option<AzureServiceEndpoints>
    },
    GcpKeyRing (
        GcpKeyRingConfig
    )
}

/// The scheme for determining how multiple `HmgConfig`s on a group
/// should behave. If not specified, the backend will go through
/// the list in random order, and use the first `HmgConfig` that works.
#[derive(Eq, Debug, PartialEq, Hash, Copy, Serialize, Deserialize, Clone)]
pub enum HmgRedundancyScheme {
    /// Go through the list of `HmgConfig`s in the order specified in
    /// each one's `hsm_order` field. Smaller numbers indicate higher
    /// priority; e.g., "1" takes precedence over "3", and "-4" takes
    /// precedence over "-1".
    PriorityFailover
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyVault {
    pub id: String,
    pub name: String,
    pub vault_type: AzureKeyVaultType,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<HashMap<String,String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention: Option<u32>,
    pub uri: String
}

/// The response of the get all groups API
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ListGroupsResponse {
    /// A response that includes metadata
    WithMetadata {
        /// The list of groups satisfying the request
        items: Vec<Group>,
        /// The metadata associated with the response
        metadata: CollectionMetadata
    },
    /// A response that omits metadata
    WithoutMetadata (
        Vec<Group>
    )
}

/// Subset of GroupPermissions to represent GroupPermissions flags in use
pub use self::quorum_group_permissions::QuorumGroupPermissions;
pub mod quorum_group_permissions {
    bitflags_set!{
        pub struct QuorumGroupPermissions: u64 {
            const GET_SOBJECTS = 0x0000000000000001;
            const ROTATE_SOBJECTS = 0x0000000000000002;
            const REVOKE_SOBJECTS = 0x0000000000000004;
            const REVERT_SOBJECTS = 0x0000000000000008;
            const DELETE_KEY_MATERIAL = 0x0000000000000010;
            const DELETE_SOBJECTS = 0x0000000000000020;
            const DESTROY_SOBJECTS = 0x0000000000000040;
            const MOVE_SOBJECTS = 0x0000000000000080;
            const CREATE_SOBJECTS = 0x0000000000000100;
            const UPDATE_SOBJECTS_PROFILE = 0x0000000000000200;
            const UPDATE_SOBJECTS_ENABLED_STATE = 0x0000000000000400;
            const UPDATE_SOBJECT_POLICIES = 0x0000000000000800;
            const ACTIVATE_SOBJECTS = 0x0000000000001000;
            const UPDATE_KEY_OPS = 0x0000000000002000;
        }
    }
}

/// An object for representing a scan of objects from a source HSM,
/// DSM cluster, or cloud KMS.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Scan {
    /// The ID of the scan.
    pub scan_id: Uuid,
    /// Whether the scan is async or not.
    pub is_async: bool,
    /// The time the scan began.
    pub started_at: Time,
    /// The time the scan finished.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<Time>,
    /// The "return status" of the scan.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scan_result: Option<ScanResult>,
    /// Any warnings thrown during the scan.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<ScanWarning>>
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ScanHmgRequest {

}

/// The result of a scan.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename = "snake_case", tag = "$type")]
pub enum ScanResult {
    /// Indicates that a scan completed successfully.
    Success,
    /// Indicates that a scan has failed. The most recent error is included
    /// (taken from the last retry).
    Failed {
        message: String
    }
}

/// A warning "thrown" by a scan.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct ScanWarning {
    /// The ID of the source key for which the warning applies to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_key_id: Option<Uuid>,
    /// The ID of the virtual key for which the warning applies to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub virtual_key_id: Option<Uuid>,
    /// The warning message associated with the warning.
    pub message: String
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum WrappingKeyName {
    Null,
    Value (
        String
    )
}

pub struct OperationAsyncScanHmg;
#[allow(unused)]
impl Operation for OperationAsyncScanHmg {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = Scan;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/groups/{group_id}/hmg/scans", group_id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn async_scan_hmg(&self, group_id: &Uuid) -> Result<Scan> {
        self.execute::<OperationAsyncScanHmg>(&(), (group_id,), None)
    }
}

pub struct OperationCheckHmg;
#[allow(unused)]
impl Operation for OperationCheckHmg {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = CheckHmgRequest;
    type Output = ();

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/groups/{group_id}/hmg/check", group_id = p.0)
    }
}

impl SdkmsClient {
    pub fn check_hmg(&self, group_id: &Uuid, req: &CheckHmgRequest) -> Result<()> {
        self.execute::<OperationCheckHmg>(req, (group_id,), None)
    }
}

pub struct OperationCheckHmgConfig;
#[allow(unused)]
impl Operation for OperationCheckHmgConfig {
    type PathParams = ();
    type QueryParams = ();
    type Body = HmgConfig;
    type Output = ();

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/sys/v1/groups/hmg/check".to_string()
    }
}

impl SdkmsClient {
    pub fn check_hmg_config(&self, req: &HmgConfig) -> Result<()> {
        self.execute::<OperationCheckHmgConfig>(req, (), None)
    }
}

pub struct OperationCreateGroup;
#[allow(unused)]
impl Operation for OperationCreateGroup {
    type PathParams = ();
    type QueryParams = ();
    type Body = GroupRequest;
    type Output = Group;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/sys/v1/groups".to_string()
    }
}

impl SdkmsClient {
    pub fn create_group(&self, req: &GroupRequest) -> Result<Group> {
        self.execute::<OperationCreateGroup>(req, (), None)
    }
    pub fn request_approval_to_create_group(
        &self, req: &GroupRequest,
        description: Option<String>) -> Result<PendingApproval<OperationCreateGroup>> {
        self.request_approval::<OperationCreateGroup>(req, (), None, description)
    }
}

pub struct OperationDeleteGroup;
#[allow(unused)]
impl Operation for OperationDeleteGroup {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = ();

    fn method() -> Method {
        Method::Delete
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/groups/{group_id}", group_id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn delete_group(&self, group_id: &Uuid) -> Result<()> {
        self.execute::<OperationDeleteGroup>(&(), (group_id,), None)
    }
}

pub struct OperationGetAllHmgScans;
#[allow(unused)]
impl Operation for OperationGetAllHmgScans {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = GetAllHmgScansResponse;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/groups/{group_id}/hmg/scans", group_id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_all_hmg_scans(&self, group_id: &Uuid) -> Result<GetAllHmgScansResponse> {
        self.execute::<OperationGetAllHmgScans>(&(), (group_id,), None)
    }
}

pub struct OperationGetGcpKeyRings;
#[allow(unused)]
impl Operation for OperationGetGcpKeyRings {
    type PathParams = ();
    type QueryParams = ();
    type Body = GcpKeyRingConfig;
    type Output = Vec<String>;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/sys/v1/groups/hmg/gcp_key_rings".to_string()
    }
}

impl SdkmsClient {
    pub fn get_gcp_key_rings(&self, req: &GcpKeyRingConfig) -> Result<Vec<String>> {
        self.execute::<OperationGetGcpKeyRings>(req, (), None)
    }
}

pub struct OperationGetGroup;
#[allow(unused)]
impl Operation for OperationGetGroup {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = Group;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/groups/{group_id}", group_id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_group(&self, group_id: &Uuid) -> Result<Group> {
        self.execute::<OperationGetGroup>(&(), (group_id,), None)
    }
}

pub struct OperationGetScan;
#[allow(unused)]
impl Operation for OperationGetScan {
    type PathParams = (Uuid, Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = Scan;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/groups/{group_id}/hmg/scans/{scan_id}", group_id = p.0, scan_id = p.1)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_scan(&self, group_id: &Uuid, scan_id: &Uuid) -> Result<Scan> {
        self.execute::<OperationGetScan>(&(), (group_id, scan_id,), None)
    }
}

pub struct OperationGetVaults;
#[allow(unused)]
impl Operation for OperationGetVaults {
    type PathParams = ();
    type QueryParams = ();
    type Body = HmgConfig;
    type Output = Vec<KeyVault>;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/sys/v1/groups/hmg/azure_vaults".to_string()
    }
}

impl SdkmsClient {
    pub fn get_vaults(&self, req: &HmgConfig) -> Result<Vec<KeyVault>> {
        self.execute::<OperationGetVaults>(req, (), None)
    }
}

pub struct OperationListGroups;
#[allow(unused)]
impl Operation for OperationListGroups {
    type PathParams = ();
    type QueryParams = GetGroupsParams;
    type Body = ();
    type Output = ListGroupsResponse;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/groups?{q}", q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn list_groups(&self, query_params: Option<&GetGroupsParams>) -> Result<ListGroupsResponse> {
        self.execute::<OperationListGroups>(&(), (), query_params)
    }
}

pub struct OperationScanHmg;
#[allow(unused)]
impl Operation for OperationScanHmg {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ScanHmgRequest;
    type Output = Vec<Sobject>;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/groups/{group_id}/hmg/scan", group_id = p.0)
    }
}

impl SdkmsClient {
    pub fn scan_hmg(&self, group_id: &Uuid, req: &ScanHmgRequest) -> Result<Vec<Sobject>> {
        self.execute::<OperationScanHmg>(req, (group_id,), None)
    }
}

pub struct OperationUpdateGroup;
#[allow(unused)]
impl Operation for OperationUpdateGroup {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = GroupRequest;
    type Output = Group;

    fn method() -> Method {
        Method::Patch
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/groups/{group_id}", group_id = p.0)
    }
}

impl SdkmsClient {
    pub fn update_group(&self, group_id: &Uuid, req: &GroupRequest) -> Result<Group> {
        self.execute::<OperationUpdateGroup>(req, (group_id,), None)
    }
    pub fn request_approval_to_update_group(
        &self, group_id: &Uuid, req: &GroupRequest,
        description: Option<String>) -> Result<PendingApproval<OperationUpdateGroup>> {
        self.request_approval::<OperationUpdateGroup>(req, (group_id,), None, description)
    }
}

