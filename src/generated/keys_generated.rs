/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

/// Request to copy a security object
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CopySobjectRequest {
    pub key: SobjectDescriptor,
    #[serde(flatten)]
    pub dest: SobjectRequest
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ExportComponentsResponse {
    pub components: Vec<SobjectComponent>,
    pub iv: Option<Blob>,
    pub tag: Option<Blob>,
    pub key_kcv: Option<String>,
    pub description: Option<String>
}

/// Request to Export a Sobject by components
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct ExportSobjectComponentsRequest {
    pub key: SobjectDescriptor,
    pub wrap_key_params: Option<WrapKeyParams>,
    pub custodians: Vec<Principal>,
    #[serde(default)]
    pub method: Option<SplittingMethod>,
    #[serde(default)]
    pub description: Option<String>
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, )]
pub struct FilterList {
    #[serde(flatten)]
    pub head: Box<CustomMetadata>
}

impl UrlEncode for FilterList {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        self.head.url_encode(m);
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GetSobjectParams {
    pub view: SobjectEncoding,
    pub show_destroyed: bool,
    pub show_deleted: bool,
    pub show_value: bool,
    pub show_pub_key: bool
}

impl UrlEncode for GetSobjectParams {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        m.insert("view".to_string(), self.view.to_string());
        m.insert("show_destroyed".to_string(), self.show_destroyed.to_string());
        m.insert("show_deleted".to_string(), self.show_deleted.to_string());
        m.insert("show_value".to_string(), self.show_value.to_string());
        m.insert("show_pub_key".to_string(), self.show_pub_key.to_string());
    }
}

/// Request to Import a Sobject by components
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImportSobjectComponentsRequest {
    pub key: SobjectRequest,
    pub unwrap_key_params: Option<UnwrapKeyParams>,
    pub custodians: Vec<Principal>,
    pub components: Option<Vec<SobjectComponent>>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub method: Option<SplittingMethod>,
    #[serde(default)]
    pub auth_config: Option<ApprovalAuthConfig>
}

/// KCV of a key
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyCheckValueResponse {
    #[serde(default)]
    pub kid: Option<Uuid>,
    pub kcv: String
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListSobjectsParams {
    pub group_id: Option<Uuid>,
    pub creator: Option<Uuid>,
    pub name: Option<String>,
    pub pkcs11_label: Option<String>,
    pub pkcs11_id: Option<Blob>,
    pub obj_type: Option<ObjectType>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    #[serde(flatten)]
    pub sort: Option<SobjectSort>,
    pub compliant_with_policies: Option<bool>,
    #[serde(flatten)]
    pub custom_metadata: Option<CustomMetadata>,
    pub with_metadata: Option<bool>,
    pub show_destroyed: bool,
    pub show_deleted: bool,
    pub show_value: bool,
    pub show_pub_key: bool,
    #[serde(default)]
    pub filter: Option<FilterList>
}

impl UrlEncode for ListSobjectsParams {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        if let Some(ref v) = self.group_id {
            m.insert("group_id".to_string(), v.to_string());
        }
        if let Some(ref v) = self.creator {
            m.insert("creator".to_string(), v.to_string());
        }
        if let Some(ref v) = self.name {
            m.insert("name".to_string(), v.to_string());
        }
        if let Some(ref v) = self.pkcs11_label {
            m.insert("pkcs11_label".to_string(), v.to_string());
        }
        if let Some(ref v) = self.pkcs11_id {
            m.insert("pkcs11_id".to_string(), v.to_string());
        }
        if let Some(ref v) = self.obj_type {
            m.insert("obj_type".to_string(), v.to_string());
        }
        if let Some(ref v) = self.limit {
            m.insert("limit".to_string(), v.to_string());
        }
        if let Some(ref v) = self.offset {
            m.insert("offset".to_string(), v.to_string());
        }
        self.sort.url_encode(m);
        if let Some(ref v) = self.compliant_with_policies {
            m.insert("compliant_with_policies".to_string(), v.to_string());
        }
        self.custom_metadata.url_encode(m);
        if let Some(ref v) = self.with_metadata {
            m.insert("with_metadata".to_string(), v.to_string());
        }
        m.insert("show_destroyed".to_string(), self.show_destroyed.to_string());
        m.insert("show_deleted".to_string(), self.show_deleted.to_string());
        m.insert("show_value".to_string(), self.show_value.to_string());
        m.insert("show_pub_key".to_string(), self.show_pub_key.to_string());
        if let Some(ref v) = self.filter {
            m.insert("filter".to_string(), v.head.encode());
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub total_count: usize,
    pub filtered_count: usize
}

/// Request to compute digest of a key.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ObjectDigestRequest {
    pub key: SobjectDescriptor,
    pub alg: DigestAlgorithm
}

/// Digest of a key.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct ObjectDigestResponse {
    #[serde(default)]
    pub kid: Option<Uuid>,
    pub digest: Blob
}

/// Request to persist a transient key.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct PersistTransientKeyRequest {
    #[serde(default)]
    pub activation_date: Option<Time>,
    #[serde(default)]
    pub deactivation_date: Option<Time>,
    /// Name of the persisted security object. Security object names must be unique within an account.
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    /// User-defined metadata for the persisted key stored as key-value pairs.
    #[serde(default)]
    pub custom_metadata: Option<HashMap<String,String>>,
    /// Whether the new security object should be enabled. Disabled security objects may not perform cryptographic operations.
    #[serde(default)]
    pub enabled: Option<bool>,
    /// Group ID of the security group that the persisted key should belong to. The user or
    /// application creating this security object must be a member of this group. If no group is
    /// specified, the default group for the requesting application will be used.
    #[serde(default)]
    pub group_id: Option<Uuid>,
    #[serde(default)]
    pub state: Option<SobjectState>,
    /// Transient key to persist.
    pub transient_key: Blob
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct RevertRequest {
    pub ids: Vec<Uuid>
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct SobjectComponent {
    pub component: Blob,
    pub component_kcv: Option<String>,
    pub custodian: Principal
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SobjectEncoding {
    Json,
    Value
}

/// Request to rekey a security object
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SobjectRekeyRequest {
    pub deactivate_rotated_key: bool,
    #[serde(flatten)]
    pub dest: SobjectRequest
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct SobjectRequest {
    #[serde(default)]
    pub activation_date: Option<Time>,
    #[serde(default)]
    pub aes: Option<AesOptions>,
    #[serde(default)]
    pub custom_metadata: Option<HashMap<String,String>>,
    #[serde(default)]
    pub deactivation_date: Option<Time>,
    #[serde(default)]
    pub des: Option<DesOptions>,
    #[serde(default)]
    pub des3: Option<Des3Options>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub deterministic_signatures: Option<bool>,
    #[serde(default)]
    pub dsa: Option<DsaOptions>,
    #[serde(default)]
    pub elliptic_curve: Option<EllipticCurve>,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub fpe: Option<FpeOptions>,
    #[serde(default)]
    pub kcv: Option<String>,
    #[serde(default)]
    pub key_ops: Option<KeyOperations>,
    #[serde(default)]
    pub key_size: Option<u32>,
    #[serde(default)]
    pub links: Option<KeyLinks>,
    #[serde(default)]
    pub lms: Option<LmsOptions>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub obj_type: Option<ObjectType>,
    #[serde(default)]
    pub pub_exponent: Option<u32>,
    #[serde(default)]
    pub publish_public_key: Option<PublishPublicKeyConfig>,
    #[serde(default)]
    pub rotation_policy: Option<RotationPolicy>,
    #[serde(default)]
    pub rsa: Option<RsaOptions>,
    #[serde(default)]
    pub seed: Option<SeedOptions>,
    #[serde(default)]
    pub state: Option<SobjectState>,
    #[serde(default)]
    pub transient: Option<bool>,
    #[serde(default)]
    pub value: Option<Blob>,
    #[serde(default)]
    pub group_id: Option<Uuid>
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub enum SobjectSort {
    ByKid {
        order: Order,
        start: Option<Uuid>
    },
    ByName {
        order: Order,
        start: Option<String>
    }
}

impl UrlEncode for SobjectSort {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        match *self {
            SobjectSort::ByKid{ ref order, ref start } => {
                m.insert("sort".to_string(), format!("kid:{}", order));
                if let Some(v) = start {
                    m.insert("start".to_string(), v.to_string());
                }
            }
            SobjectSort::ByName{ ref order, ref start } => {
                m.insert("sort".to_string(), format!("name:{}", order));
                if let Some(v) = start {
                    m.insert("start".to_string(), v.to_string());
                }
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub enum SplittingMethod {
    XOR
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct UnwrapKeyParams {
    pub key: SobjectDescriptor,
    pub alg: Algorithm,
    /// Mode is required for symmetric algorithms.
    #[serde(default)]
    pub mode: Option<CryptMode>,
    /// Initialization vector is required for symmetric algorithms.
    #[serde(default)]
    pub iv: Option<Blob>,
    /// Authenticated data is only applicable if mode is GCM.
    #[serde(default)]
    pub ad: Option<Blob>,
    /// Tag is required if mode is GCM.
    #[serde(default)]
    pub tag: Option<Blob>
}

/// Verify KCV of a key
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VerifyKcvRequest {
    pub kcv: String,
    pub value: Blob,
    pub obj_type: ObjectType
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VerifyKcvResponse {
    pub verified: bool
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct WrapKeyParams {
    /// The wrapping key.
    pub key: SobjectDescriptor,
    pub alg: Algorithm,
    /// Mode is required for symmetric algorithms.
    #[serde(default)]
    pub mode: Option<CryptMode>,
    #[serde(default)]
    pub iv: Option<Blob>,
    /// Authenticated data is only applicable if mode is GCM.
    #[serde(default)]
    pub ad: Option<Blob>,
    /// Tag length is required when mode is GCM.
    #[serde(default)]
    pub tag_len: Option<usize>
}

pub struct OperationActivateSobject;
#[allow(unused)]
impl Operation for OperationActivateSobject {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = ();

    fn method() -> Method {
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/crypto/v1/keys/{id}/activate", id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn activate_sobject(&self, id: &Uuid) -> Result<()> {
        self.execute::<OperationActivateSobject>(&(), (id,), None)
    }
}

pub struct OperationBatchSign;
#[allow(unused)]
impl Operation for OperationBatchSign {
    type PathParams = ();
    type QueryParams = ();
    type Body = Vec<SignRequest>;
    type Output = Vec<BatchResponseItem<SignResponse>>;

    fn method() -> Method {
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/crypto/v1/keys/batch/sign")
    }
}

impl SdkmsClient {
    pub fn batch_sign(&self, req: &Vec<SignRequest>) -> Result<Vec<BatchResponseItem<SignResponse>>> {
        self.execute::<OperationBatchSign>(req, (), None)
    }
    pub fn request_approval_to_batch_sign(
        &self, req: &Vec<SignRequest>,
        description: Option<String>) -> Result<PendingApproval<OperationBatchSign>> {
        self.request_approval::<OperationBatchSign>(req, (), None, description)
    }
}

pub struct OperationBatchVerify;
#[allow(unused)]
impl Operation for OperationBatchVerify {
    type PathParams = ();
    type QueryParams = ();
    type Body = Vec<VerifyRequest>;
    type Output = Vec<BatchResponseItem<VerifyResponse>>;

    fn method() -> Method {
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/crypto/v1/keys/batch/verify")
    }
}

impl SdkmsClient {
    pub fn batch_verify(&self, req: &Vec<VerifyRequest>) -> Result<Vec<BatchResponseItem<VerifyResponse>>> {
        self.execute::<OperationBatchVerify>(req, (), None)
    }
}

pub struct OperationCopySobject;
#[allow(unused)]
impl Operation for OperationCopySobject {
    type PathParams = ();
    type QueryParams = ();
    type Body = CopySobjectRequest;
    type Output = Sobject;

    fn method() -> Method {
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/crypto/v1/keys/copy")
    }
}

impl SdkmsClient {
    pub fn copy_sobject(&self, req: &CopySobjectRequest) -> Result<Sobject> {
        self.execute::<OperationCopySobject>(req, (), None)
    }
    pub fn request_approval_to_copy_sobject(
        &self, req: &CopySobjectRequest,
        description: Option<String>) -> Result<PendingApproval<OperationCopySobject>> {
        self.request_approval::<OperationCopySobject>(req, (), None, description)
    }
}

pub struct OperationCreateSobject;
#[allow(unused)]
impl Operation for OperationCreateSobject {
    type PathParams = ();
    type QueryParams = ();
    type Body = SobjectRequest;
    type Output = Sobject;

    fn method() -> Method {
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/crypto/v1/keys")
    }
}

impl SdkmsClient {
    pub fn create_sobject(&self, req: &SobjectRequest) -> Result<Sobject> {
        self.execute::<OperationCreateSobject>(req, (), None)
    }
}

pub struct OperationDeleteSobject;
#[allow(unused)]
impl Operation for OperationDeleteSobject {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = ();

    fn method() -> Method {
        Method::DELETE
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/crypto/v1/keys/{id}", id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn delete_sobject(&self, id: &Uuid) -> Result<()> {
        self.execute::<OperationDeleteSobject>(&(), (id,), None)
    }
    pub fn request_approval_to_delete_sobject(
        &self, id: &Uuid,
        description: Option<String>) -> Result<PendingApproval<OperationDeleteSobject>> {
        self.request_approval::<OperationDeleteSobject>(&(), (id,), None, description)
    }
}

pub struct OperationDestroySobject;
#[allow(unused)]
impl Operation for OperationDestroySobject {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = ();

    fn method() -> Method {
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/crypto/v1/keys/{id}/destroy", id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn destroy_sobject(&self, id: &Uuid) -> Result<()> {
        self.execute::<OperationDestroySobject>(&(), (id,), None)
    }
    pub fn request_approval_to_destroy_sobject(
        &self, id: &Uuid,
        description: Option<String>) -> Result<PendingApproval<OperationDestroySobject>> {
        self.request_approval::<OperationDestroySobject>(&(), (id,), None, description)
    }
}

pub struct OperationDigestSobject;
#[allow(unused)]
impl Operation for OperationDigestSobject {
    type PathParams = ();
    type QueryParams = ();
    type Body = ObjectDigestRequest;
    type Output = ObjectDigestResponse;

    fn method() -> Method {
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/crypto/v1/keys/digest")
    }
}

impl SdkmsClient {
    pub fn digest_sobject(&self, req: &ObjectDigestRequest) -> Result<ObjectDigestResponse> {
        self.execute::<OperationDigestSobject>(req, (), None)
    }
}

pub struct OperationExportSobject;
#[allow(unused)]
impl Operation for OperationExportSobject {
    type PathParams = ();
    type QueryParams = ();
    type Body = SobjectDescriptor;
    type Output = Sobject;

    fn method() -> Method {
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/crypto/v1/keys/export")
    }
}

impl SdkmsClient {
    pub fn export_sobject(&self, req: &SobjectDescriptor) -> Result<Sobject> {
        self.execute::<OperationExportSobject>(req, (), None)
    }
    pub fn request_approval_to_export_sobject(
        &self, req: &SobjectDescriptor,
        description: Option<String>) -> Result<PendingApproval<OperationExportSobject>> {
        self.request_approval::<OperationExportSobject>(req, (), None, description)
    }
}

pub struct OperationExportSobjectComponents;
#[allow(unused)]
impl Operation for OperationExportSobjectComponents {
    type PathParams = ();
    type QueryParams = ();
    type Body = ExportSobjectComponentsRequest;
    type Output = ExportComponentsResponse;

    fn method() -> Method {
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/crypto/v1/keys/components/export")
    }
}

impl SdkmsClient {
    pub fn export_sobject_components(&self, req: &ExportSobjectComponentsRequest) -> Result<ExportComponentsResponse> {
        self.execute::<OperationExportSobjectComponents>(req, (), None)
    }
    pub fn request_approval_to_export_sobject_components(
        &self, req: &ExportSobjectComponentsRequest,
        description: Option<String>) -> Result<PendingApproval<OperationExportSobjectComponents>> {
        self.request_approval::<OperationExportSobjectComponents>(req, (), None, description)
    }
}

pub struct OperationGetKcv;
#[allow(unused)]
impl Operation for OperationGetKcv {
    type PathParams = ();
    type QueryParams = ();
    type Body = SobjectDescriptor;
    type Output = KeyCheckValueResponse;

    fn method() -> Method {
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/crypto/v1/keys/kcv")
    }
}

impl SdkmsClient {
    pub fn get_kcv(&self, req: &SobjectDescriptor) -> Result<KeyCheckValueResponse> {
        self.execute::<OperationGetKcv>(req, (), None)
    }
}

pub struct OperationGetPubkey;
#[allow(unused)]
impl Operation for OperationGetPubkey {
    type PathParams = (Uuid, String,);
    type QueryParams = ();
    type Body = ();
    type Output = HashMap<String,Blob>;

    fn method() -> Method {
        Method::GET
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/crypto/v1/pubkey/{id}/{name}", id = p.0, name = p.1)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_pubkey(&self, id: &Uuid, name: &String) -> Result<HashMap<String,Blob>> {
        self.execute::<OperationGetPubkey>(&(), (id, name,), None)
    }
}

pub struct OperationGetSobject;
#[allow(unused)]
impl Operation for OperationGetSobject {
    type PathParams = ();
    type QueryParams = GetSobjectParams;
    type Body = SobjectDescriptor;
    type Output = Sobject;

    fn method() -> Method {
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/crypto/v1/keys/info?{q}", q = q.encode())
    }
}

impl SdkmsClient {
    pub fn get_sobject(&self, query_params: Option<&GetSobjectParams>, req: &SobjectDescriptor) -> Result<Sobject> {
        self.execute::<OperationGetSobject>(req, (), query_params)
    }
}

pub struct OperationImportSobject;
#[allow(unused)]
impl Operation for OperationImportSobject {
    type PathParams = ();
    type QueryParams = ();
    type Body = SobjectRequest;
    type Output = Sobject;

    fn method() -> Method {
        Method::PUT
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/crypto/v1/keys")
    }
}

impl SdkmsClient {
    pub fn import_sobject(&self, req: &SobjectRequest) -> Result<Sobject> {
        self.execute::<OperationImportSobject>(req, (), None)
    }
}

pub struct OperationImportSobjectByComponents;
#[allow(unused)]
impl Operation for OperationImportSobjectByComponents {
    type PathParams = ();
    type QueryParams = ();
    type Body = ImportSobjectComponentsRequest;
    type Output = Sobject;

    fn method() -> Method {
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/crypto/v1/keys/components/import")
    }
}

impl SdkmsClient {
    pub fn import_sobject_by_components(&self, req: &ImportSobjectComponentsRequest) -> Result<Sobject> {
        self.execute::<OperationImportSobjectByComponents>(req, (), None)
    }
    pub fn request_approval_to_import_sobject_by_components(
        &self, req: &ImportSobjectComponentsRequest,
        description: Option<String>) -> Result<PendingApproval<OperationImportSobjectByComponents>> {
        self.request_approval::<OperationImportSobjectByComponents>(req, (), None, description)
    }
}

pub struct OperationListSobjects;
#[allow(unused)]
impl Operation for OperationListSobjects {
    type PathParams = ();
    type QueryParams = ListSobjectsParams;
    type Body = ();
    type Output = GetAllResponse;

    fn method() -> Method {
        Method::GET
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/crypto/v1/keys?{q}", q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn list_sobjects(&self, query_params: Option<&ListSobjectsParams>) -> Result<GetAllResponse> {
        self.execute::<OperationListSobjects>(&(), (), query_params)
    }
}

pub struct OperationPersistTransientKey;
#[allow(unused)]
impl Operation for OperationPersistTransientKey {
    type PathParams = ();
    type QueryParams = ();
    type Body = PersistTransientKeyRequest;
    type Output = Sobject;

    fn method() -> Method {
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/crypto/v1/keys/persist")
    }
}

impl SdkmsClient {
    pub fn persist_transient_key(&self, req: &PersistTransientKeyRequest) -> Result<Sobject> {
        self.execute::<OperationPersistTransientKey>(req, (), None)
    }
}

pub struct OperationRemovePrivate;
#[allow(unused)]
impl Operation for OperationRemovePrivate {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = ();

    fn method() -> Method {
        Method::DELETE
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/crypto/v1/keys/{id}/private", id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn remove_private(&self, id: &Uuid) -> Result<()> {
        self.execute::<OperationRemovePrivate>(&(), (id,), None)
    }
    pub fn request_approval_to_remove_private(
        &self, id: &Uuid,
        description: Option<String>) -> Result<PendingApproval<OperationRemovePrivate>> {
        self.request_approval::<OperationRemovePrivate>(&(), (id,), None, description)
    }
}

pub struct OperationRevertPrevKeyOp;
#[allow(unused)]
impl Operation for OperationRevertPrevKeyOp {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = RevertRequest;
    type Output = ();

    fn method() -> Method {
        Method::PUT
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/crypto/v1/keys/{id}/revert", id = p.0)
    }
}

impl SdkmsClient {
    pub fn revert_prev_key_op(&self, id: &Uuid, req: &RevertRequest) -> Result<()> {
        self.execute::<OperationRevertPrevKeyOp>(req, (id,), None)
    }
    pub fn request_approval_to_revert_prev_key_op(
        &self, id: &Uuid, req: &RevertRequest,
        description: Option<String>) -> Result<PendingApproval<OperationRevertPrevKeyOp>> {
        self.request_approval::<OperationRevertPrevKeyOp>(req, (id,), None, description)
    }
}

pub struct OperationRevokeSobject;
#[allow(unused)]
impl Operation for OperationRevokeSobject {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = RevocationReason;
    type Output = ();

    fn method() -> Method {
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/crypto/v1/keys/{id}/revoke", id = p.0)
    }
}

impl SdkmsClient {
    pub fn revoke_sobject(&self, id: &Uuid, req: &RevocationReason) -> Result<()> {
        self.execute::<OperationRevokeSobject>(req, (id,), None)
    }
    pub fn request_approval_to_revoke_sobject(
        &self, id: &Uuid, req: &RevocationReason,
        description: Option<String>) -> Result<PendingApproval<OperationRevokeSobject>> {
        self.request_approval::<OperationRevokeSobject>(req, (id,), None, description)
    }
}

pub struct OperationRotateSobject;
#[allow(unused)]
impl Operation for OperationRotateSobject {
    type PathParams = ();
    type QueryParams = ();
    type Body = SobjectRekeyRequest;
    type Output = Sobject;

    fn method() -> Method {
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/crypto/v1/keys/rekey")
    }
}

impl SdkmsClient {
    pub fn rotate_sobject(&self, req: &SobjectRekeyRequest) -> Result<Sobject> {
        self.execute::<OperationRotateSobject>(req, (), None)
    }
    pub fn request_approval_to_rotate_sobject(
        &self, req: &SobjectRekeyRequest,
        description: Option<String>) -> Result<PendingApproval<OperationRotateSobject>> {
        self.request_approval::<OperationRotateSobject>(req, (), None, description)
    }
}

pub struct OperationUpdateSobject;
#[allow(unused)]
impl Operation for OperationUpdateSobject {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = SobjectRequest;
    type Output = Sobject;

    fn method() -> Method {
        Method::PATCH
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/crypto/v1/keys/{id}", id = p.0)
    }
}

impl SdkmsClient {
    pub fn update_sobject(&self, id: &Uuid, req: &SobjectRequest) -> Result<Sobject> {
        self.execute::<OperationUpdateSobject>(req, (id,), None)
    }
    pub fn request_approval_to_update_sobject(
        &self, id: &Uuid, req: &SobjectRequest,
        description: Option<String>) -> Result<PendingApproval<OperationUpdateSobject>> {
        self.request_approval::<OperationUpdateSobject>(req, (id,), None, description)
    }
}

pub struct OperationVerifyKcv;
#[allow(unused)]
impl Operation for OperationVerifyKcv {
    type PathParams = ();
    type QueryParams = ();
    type Body = VerifyKcvRequest;
    type Output = VerifyKcvResponse;

    fn method() -> Method {
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/crypto/v1/keys/kcv/verify")
    }
}

impl SdkmsClient {
    pub fn verify_kcv(&self, req: &VerifyKcvRequest) -> Result<VerifyKcvResponse> {
        self.execute::<OperationVerifyKcv>(req, (), None)
    }
}

