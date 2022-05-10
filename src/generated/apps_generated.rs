/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct App {
    pub acct_id: Uuid,
    pub app_id: Uuid,
    pub app_type: String,
    #[serde(default)]
    pub auth_type: Option<AppAuthType>,
    #[serde(default)]
    pub cert_not_after: Option<Time>,
    pub client_configurations: ClientConfigurations,
    pub created_at: Time,
    pub creator: Principal,
    #[serde(default)]
    pub default_group: Option<Uuid>,
    #[serde(default)]
    pub description: Option<String>,
    pub enabled: bool,
    pub groups: AppGroups,
    #[serde(default)]
    pub interface: Option<String>,
    pub ip_address_policy: IpAddressPolicy,
    pub last_operations: LastAppOperationTimestamp,
    #[serde(default)]
    pub lastused_at: Option<Time>,
    pub legacy_access: bool,
    pub name: String,
    #[serde(default)]
    pub oauth_config: Option<AppOauthConfig>,
    pub role: AppRole
}

/// Authentication method of an app.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum AppAuthType {
    Secret,
    Certificate,
    TrustedCa,
    GoogleServiceAccount,
    SignedJwt,
    Ldap,
    AwsIam
}

/// App authentication mechanisms.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AppCredential {
    Secret (
        String
    ),
    Certificate (
        Blob
    ),
    TrustedCa (
        TrustAnchor
    ),
    GoogleServiceAccount {
        #[serde(default)]
        access_reason_policy: Option<GoogleAccessReasonPolicy>
    },
    SignedJwt {
        valid_issuers: HashSet<String>,
        signing_keys: JwtSigningKeys
    },
    Ldap (
        Uuid
    ),
    AwsIam {

    }
}

/// App credential response.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppCredentialResponse {
    pub app_id: Uuid,
    pub credential: AppCredential,
    #[serde(default)]
    pub previous_credential: Option<PreviousCredential>
}

/// OAuth settings for an app. If enabled, an app can request to act on behalf of a user.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", tag = "state")]
pub enum AppOauthConfig {
    Enabled {
        redirect_uris: Vec<String>
    },
    Disabled
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct AppRequest {
    #[serde(default)]
    pub add_groups: Option<AppGroups>,
    #[serde(default)]
    pub app_type: Option<String>,
    #[serde(default)]
    pub client_configurations: Option<ClientConfigurationsRequest>,
    #[serde(default)]
    pub credential: Option<AppCredential>,
    #[serde(default)]
    pub credential_migration_period: Option<u32>,
    #[serde(default)]
    pub default_group: Option<Uuid>,
    #[serde(default)]
    pub del_groups: Option<HashSet<Uuid>>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub interface: Option<String>,
    #[serde(default)]
    pub ip_address_policy: Option<IpAddressPolicy>,
    #[serde(default)]
    pub mod_groups: Option<AppGroups>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub oauth_config: Option<AppOauthConfig>,
    #[serde(default)]
    pub role: Option<AppRole>,
    #[serde(default)]
    pub secret_size: Option<u32>
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct AppResetSecretRequest {
    /// Size of app's secret in bytes.
    #[serde(default)]
    pub secret_size: Option<u32>,
    #[serde(default)]
    pub credential_migration_period: Option<u32>
}

/// App's role.
#[derive(Debug, Eq, PartialEq, Copy, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AppRole {
    Admin,
    Crypto
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AppSort {
    ByAppId {
        order: Order,
        start: Option<Uuid>
    }
}

impl UrlEncode for AppSort {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        match *self {
            AppSort::ByAppId{ ref order, ref start } => {
                m.insert("sort".to_string(), format!("app_id:{}", order));
                if let Some(v) = start {
                    m.insert("start".to_string(), v.to_string());
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GetAppParams {
    pub group_permissions: bool,
    pub role: Option<String>
}

impl UrlEncode for GetAppParams {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        m.insert("group_permissions".to_string(), self.group_permissions.to_string());
        if let Some(ref v) = self.role {
            m.insert("role".to_string(), v.to_string());
        }
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum IpAddressPolicy {
    AllowAll,
    Whitelist (
        HashSet<String>
    )
}

#[derive(Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct LastAppOperationTimestamp {
    #[serde(default)]
    pub generic: Option<u64>,
    #[serde(default)]
    pub tokenization: Option<u64>,
    #[serde(default)]
    pub tep: Option<u64>,
    #[serde(default)]
    pub accelerator: Option<u64>,
    #[serde(default)]
    pub secrets_management: Option<u64>
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListAppsParams {
    pub group_id: Option<Uuid>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    #[serde(flatten)]
    pub sort: AppSort,
    pub group_permissions: bool,
    pub role: Option<AppRole>
}

impl UrlEncode for ListAppsParams {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        if let Some(ref v) = self.group_id {
            m.insert("group_id".to_string(), v.to_string());
        }
        if let Some(ref v) = self.limit {
            m.insert("limit".to_string(), v.to_string());
        }
        if let Some(ref v) = self.offset {
            m.insert("offset".to_string(), v.to_string());
        }
        self.sort.url_encode(m);
        m.insert("group_permissions".to_string(), self.group_permissions.to_string());
        if let Some(ref v) = self.role {
            m.insert("role".to_string(), v.to_string());
        }
    }
}

/// Expired credentials that are still valid for a transitional period
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct PreviousCredential {
    pub credential: AppCredential,
    pub valid_until: Time
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SubjectGeneral {
    DirectoryName (
        Vec<[String; 2]>
    ),
    DnsName (
        String
    ),
    IpAddress (
        IpAddr
    )
}

/// A trusted CA for app authentication.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct TrustAnchor {
    #[serde(flatten)]
    pub subject: TrustAnchorSubject,
    pub ca_certificate: Blob
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TrustAnchorSubject {
    Subject (
        Vec<[String; 2]>
    ),
    SubjectGeneral (
        SubjectGeneral
    )
}

pub struct OperationCreateApp;
#[allow(unused)]
impl Operation for OperationCreateApp {
    type PathParams = ();
    type QueryParams = GetAppParams;
    type Body = AppRequest;
    type Output = App;

    fn method() -> Method {
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/apps?{q}", q = q.encode())
    }
}

impl SdkmsClient {
    pub fn create_app(&self, query_params: Option<&GetAppParams>, req: &AppRequest) -> Result<App> {
        self.execute::<OperationCreateApp>(req, (), query_params)
    }
}

pub struct OperationDeleteApp;
#[allow(unused)]
impl Operation for OperationDeleteApp {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = ();

    fn method() -> Method {
        Method::DELETE
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/apps/{id}", id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn delete_app(&self, id: &Uuid) -> Result<()> {
        self.execute::<OperationDeleteApp>(&(), (id,), None)
    }
}

pub struct OperationGetApp;
#[allow(unused)]
impl Operation for OperationGetApp {
    type PathParams = (Uuid,);
    type QueryParams = GetAppParams;
    type Body = ();
    type Output = App;

    fn method() -> Method {
        Method::GET
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/apps/{id}?{q}", id = p.0, q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_app(&self, id: &Uuid, query_params: Option<&GetAppParams>) -> Result<App> {
        self.execute::<OperationGetApp>(&(), (id,), query_params)
    }
}

pub struct OperationGetAppCredential;
#[allow(unused)]
impl Operation for OperationGetAppCredential {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = AppCredentialResponse;

    fn method() -> Method {
        Method::GET
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/apps/{id}/credential", id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_app_credential(&self, id: &Uuid) -> Result<AppCredentialResponse> {
        self.execute::<OperationGetAppCredential>(&(), (id,), None)
    }
    pub fn request_approval_to_get_app_credential(
        &self, id: &Uuid,
        description: Option<String>) -> Result<PendingApproval<OperationGetAppCredential>> {
        self.request_approval::<OperationGetAppCredential>(&(), (id,), None, description)
    }
}

pub struct OperationGetClientConfigs;
#[allow(unused)]
impl Operation for OperationGetClientConfigs {
    type PathParams = ();
    type QueryParams = ();
    type Body = ();
    type Output = ClientConfigurations;

    fn method() -> Method {
        Method::GET
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/apps/client_configs")
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_client_configs(&self) -> Result<ClientConfigurations> {
        self.execute::<OperationGetClientConfigs>(&(), (), None)
    }
}

pub struct OperationListApps;
#[allow(unused)]
impl Operation for OperationListApps {
    type PathParams = ();
    type QueryParams = ListAppsParams;
    type Body = ();
    type Output = Vec<App>;

    fn method() -> Method {
        Method::GET
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/apps?{q}", q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn list_apps(&self, query_params: Option<&ListAppsParams>) -> Result<Vec<App>> {
        self.execute::<OperationListApps>(&(), (), query_params)
    }
}

pub struct OperationResetAppSecret;
#[allow(unused)]
impl Operation for OperationResetAppSecret {
    type PathParams = (Uuid,);
    type QueryParams = GetAppParams;
    type Body = AppResetSecretRequest;
    type Output = App;

    fn method() -> Method {
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/apps/{id}/reset_secret?{q}", id = p.0, q = q.encode())
    }
}

impl SdkmsClient {
    pub fn reset_app_secret(&self, id: &Uuid, query_params: Option<&GetAppParams>, req: &AppResetSecretRequest) -> Result<App> {
        self.execute::<OperationResetAppSecret>(req, (id,), query_params)
    }
    pub fn request_approval_to_reset_app_secret(
        &self, id: &Uuid, query_params: Option<&GetAppParams>, req: &AppResetSecretRequest,
        description: Option<String>) -> Result<PendingApproval<OperationResetAppSecret>> {
        self.request_approval::<OperationResetAppSecret>(req, (id,), query_params, description)
    }
}

pub struct OperationUpdateApp;
#[allow(unused)]
impl Operation for OperationUpdateApp {
    type PathParams = (Uuid,);
    type QueryParams = GetAppParams;
    type Body = AppRequest;
    type Output = App;

    fn method() -> Method {
        Method::PATCH
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/apps/{id}?{q}", id = p.0, q = q.encode())
    }
}

impl SdkmsClient {
    pub fn update_app(&self, id: &Uuid, query_params: Option<&GetAppParams>, req: &AppRequest) -> Result<App> {
        self.execute::<OperationUpdateApp>(req, (id,), query_params)
    }
    pub fn request_approval_to_update_app(
        &self, id: &Uuid, query_params: Option<&GetAppParams>, req: &AppRequest,
        description: Option<String>) -> Result<PendingApproval<OperationUpdateApp>> {
        self.request_approval::<OperationUpdateApp>(req, (id,), query_params, description)
    }
}

