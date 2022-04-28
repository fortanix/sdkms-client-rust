/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone)]
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
    #[serde(rename = "ap-south-1")]
    ApSouth1,
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
    #[serde(rename = "eu-central-1")]
    EuCentral1,
    #[serde(rename = "eu-west-1")]
    EuWest1,
    #[serde(rename = "eu-west-2")]
    EuWest2,
    #[serde(rename = "eu-south-1")]
    EuSouth1,
    #[serde(rename = "eu-west-3")]
    EuWest3,
    #[serde(rename = "eu-north-1")]
    EuNorth1,
    #[serde(rename = "me-south-1")]
    MeSouth1,
    #[serde(rename = "sa-east-1")]
    SaEast1,
    #[serde(rename = "us-gov-east-1")]
    UsGovEast1,
    #[serde(rename = "us-gov-west-1")]
    UsGovWest1
}

#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum AwskmsService {
    Kms,
    KmsFips
}

#[derive(Debug, Eq, PartialEq, Copy, Hash, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum AzureKeyVaultType {
    Standard,
    Premium,
    Managed
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct CheckHmgRequest {
    /// The ID of the hmg configuration in the group.
    pub id: Option<Uuid>,
    pub config: Option<HmgConfig>
}

#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone)]
pub struct GcpKeyRingConfig {
    pub service_account_email: String,
    pub project_id: String,
    pub location: String,
    pub key_ring: Option<String>,
    pub private_key: Option<Blob>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Group {
    pub acct_id: Uuid,
    #[serde(default)]
    pub approval_policy: Option<GroupApprovalPolicy>,
    pub client_configurations: ClientConfigurations,
    pub created_at: Time,
    pub creator: Principal,
    #[serde(default)]
    pub cryptographic_policy: Option<CryptographicPolicy>,
    #[serde(default)]
    pub custodian_policy: Option<QuorumPolicy>,
    #[serde(default)]
    pub custom_metadata: Option<HashMap<String,String>>,
    #[serde(default)]
    pub description: Option<String>,
    pub group_id: Uuid,
    #[serde(default)]
    pub hmg: Option<HashMap<Uuid,HmgConfig>>,
    #[serde(default)]
    pub hmg_redundancy: Option<HmgRedundancyScheme>,
    #[serde(default)]
    pub hmg_segregation: Option<bool>,
    #[serde(default)]
    pub hmg_sync: Option<bool>,
    #[serde(default)]
    pub key_history_policy: Option<KeyHistoryPolicy>,
    #[serde(default)]
    pub key_metadata_policy: Option<KeyMetadataPolicy>,
    pub name: String
}

/// Group approval policy.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct GroupApprovalPolicy {
    #[serde(flatten)]
    pub policy: QuorumPolicy,
    /// When this is true, manage operations on security objects require approval.
    pub protect_manage_operations: Option<bool>,
    /// When this is true, cryptographic operations on security objects require approval.
    pub protect_crypto_operations: Option<bool>
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct GroupRequest {
    #[serde(default)]
    pub add_hmg: Option<Vec<HmgConfig>>,
    #[serde(default)]
    pub approval_policy: Option<GroupApprovalPolicy>,
    #[serde(default)]
    pub client_configurations: Option<ClientConfigurationsRequest>,
    #[serde(default)]
    pub cryptographic_policy: Option<Option<CryptographicPolicy>>,
    #[serde(default)]
    pub custodian_policy: Option<QuorumPolicy>,
    #[serde(default)]
    pub custom_metadata: Option<HashMap<String,String>>,
    #[serde(default)]
    pub del_hmg: Option<HashSet<Uuid>>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub hmg_redundancy: Option<HmgRedundancyScheme>,
    #[serde(default)]
    pub hmg_segregation: Option<bool>,
    #[serde(default)]
    pub hmg_sync: Option<bool>,
    #[serde(default)]
    pub key_history_policy: Option<Option<KeyHistoryPolicy>>,
    #[serde(default)]
    pub key_metadata_policy: Option<Option<KeyMetadataPolicy>>,
    #[serde(default)]
    pub mod_hmg: Option<HashMap<Uuid,HmgConfig>>,
    #[serde(default)]
    pub name: Option<String>
}

#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind")]
pub enum HmgConfig {
    Ncipher {
        url: String,
        tls: TlsConfig,
        slot: usize,
        #[serde(default)]
        pin: Option<String>,
        #[serde(default)]
        hsm_order: Option<i32>
    },
    Safenet {
        url: String,
        tls: TlsConfig,
        slot: usize,
        #[serde(default)]
        pin: Option<String>,
        #[serde(default)]
        hsm_order: Option<i32>
    },
    AwsCloudHsm {
        url: String,
        tls: TlsConfig,
        slot: usize,
        #[serde(default)]
        pin: Option<String>,
        #[serde(default)]
        hsm_order: Option<i32>
    },
    AwsKms {
        url: String,
        tls: TlsConfig,
        #[serde(default)]
        access_key: Option<String>,
        #[serde(default)]
        secret_key: Option<String>,
        #[serde(default)]
        region: Option<AwskmsRegion>,
        #[serde(default)]
        service: Option<AwskmsService>
    },
    Fortanix {
        url: String,
        tls: TlsConfig,
        #[serde(default)]
        pin: Option<String>
    },
    FortanixFipsCluster {
        url: String,
        tls: TlsConfig,
        #[serde(default)]
        pin: Option<String>
    },
    AzureKeyVault {
        url: String,
        tls: TlsConfig,
        #[serde(default)]
        secret_key: Option<String>,
        tenant_id: Uuid,
        client_id: Uuid,
        subscription_id: Uuid,
        #[serde(default)]
        key_vault_type: Option<AzureKeyVaultType>
    },
    GcpKeyRing (
        GcpKeyRingConfig
    )
}

#[derive(Eq, Debug, PartialEq, Hash, Copy, Serialize, Deserialize, Clone)]
pub enum HmgRedundancyScheme {
    PriorityFailover
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyVault {
    pub id: String,
    pub name: String,
    pub vault_type: AzureKeyVaultType,
    pub location: String,
    #[serde(default)]
    pub tags: Option<HashMap<String,String>>,
    #[serde(default)]
    pub retention: Option<u32>,
    pub uri: String
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ScanHmgRequest {
    pub config: Option<HmgConfig>
}

pub struct OperationCheckHmg;
#[allow(unused)]
impl Operation for OperationCheckHmg {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = CheckHmgRequest;
    type Output = ();

    fn method() -> Method {
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/groups/{id}/hmg/check", id = p.0)
    }
}

impl SdkmsClient {
    pub fn check_hmg(&self, id: &Uuid, req: &CheckHmgRequest) -> Result<()> {
        self.execute::<OperationCheckHmg>(req, (id,), None)
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
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/groups/hmg/check")
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
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/groups")
    }
}

impl SdkmsClient {
    pub fn create_group(&self, req: &GroupRequest) -> Result<Group> {
        self.execute::<OperationCreateGroup>(req, (), None)
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
        Method::DELETE
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/groups/{id}", id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn delete_group(&self, id: &Uuid) -> Result<()> {
        self.execute::<OperationDeleteGroup>(&(), (id,), None)
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
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/groups/hmg/gcp_key_rings")
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
        Method::GET
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/groups/{id}", id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_group(&self, id: &Uuid) -> Result<Group> {
        self.execute::<OperationGetGroup>(&(), (id,), None)
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
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/groups/hmg/azure_vaults")
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
    type QueryParams = ();
    type Body = ();
    type Output = Vec<Group>;

    fn method() -> Method {
        Method::GET
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/groups")
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn list_groups(&self) -> Result<Vec<Group>> {
        self.execute::<OperationListGroups>(&(), (), None)
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
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/groups/{id}/hmg/scan", id = p.0)
    }
}

impl SdkmsClient {
    pub fn scan_hmg(&self, id: &Uuid, req: &ScanHmgRequest) -> Result<Vec<Sobject>> {
        self.execute::<OperationScanHmg>(req, (id,), None)
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
        Method::PATCH
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/groups/{id}", id = p.0)
    }
}

impl SdkmsClient {
    pub fn update_group(&self, id: &Uuid, req: &GroupRequest) -> Result<Group> {
        self.execute::<OperationUpdateGroup>(req, (id,), None)
    }
    pub fn request_approval_to_update_group(
        &self, id: &Uuid, req: &GroupRequest,
        description: Option<String>) -> Result<PendingApproval<OperationUpdateGroup>> {
        self.request_approval::<OperationUpdateGroup>(req, (id,), None, description)
    }
}

