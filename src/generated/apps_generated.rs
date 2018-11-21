/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

/// Operations allowed to be performed by an app.
pub use self::app_permissions::AppPermissions;
pub mod app_permissions {
    bitflags_set! {
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
        }
    }
}

/// OAuth settings for an app. If enabled, an app can request to act on behalf of a user.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", tag = "state")]
pub enum AppOauthConfig {
    Enabled { redirect_uris: Vec<String> },
    Disabled,
}

/// A trusted CA for app authentication.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct TrustAnchor {
    pub subject: Vec<[String; 2]>,
    pub ca_certificate: Blob,
}

/// App authentication mechanisms.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AppCredential {
    Secret(String),
    Certificate(Blob),
    TrustedCa(TrustAnchor),
    SignedJwt {
        valid_issuers: HashSet<String>,
        signing_keys: JwtSigningKeys,
    },
}

/// Authentication method of an app.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AppAuthType {
    Secret,
    Certificate,
    TrustedCa,
    SignedJwt,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct App {
    pub acct_id: Uuid,
    pub app_id: Uuid,
    pub app_type: String,
    #[serde(default)]
    pub auth_type: Option<AppAuthType>,
    #[serde(default)]
    pub cert_not_after: Option<Time>,
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
    #[serde(default)]
    pub lastused_at: Option<Time>,
    pub name: String,
    #[serde(default)]
    pub oauth_config: Option<AppOauthConfig>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct AppRequest {
    #[serde(default)]
    pub add_groups: Option<AppGroups>,
    #[serde(default)]
    pub app_type: Option<String>,
    #[serde(default)]
    pub credential: Option<AppCredential>,
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
    pub mod_groups: Option<AppGroups>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub oauth_config: Option<AppOauthConfig>,
    #[serde(default)]
    pub secret_size: Option<u32>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct AppResetSecretRequest {
    /// Size of app's secret in bytes.
    #[serde(default)]
    pub secret_size: Option<u32>,
}

/// App credential response.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppCredentialResponse {
    pub app_id: Uuid,
    pub credential: AppCredential,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GetAppParams {
    pub group_permissions: bool,
}

impl UrlEncode for GetAppParams {
    fn url_encode(&self, m: &mut HashMap<&'static str, String>) {
        m.insert("group_permissions", self.group_permissions.to_string());
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ListAppsParams {
    pub group_id: Option<Uuid>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    #[serde(flatten)]
    pub sort: AppSort,
    pub group_permissions: bool,
}

impl UrlEncode for ListAppsParams {
    fn url_encode(&self, m: &mut HashMap<&'static str, String>) {
        if let Some(ref v) = self.group_id {
            m.insert("group_id", v.to_string());
        }
        if let Some(ref v) = self.limit {
            m.insert("limit", v.to_string());
        }
        if let Some(ref v) = self.offset {
            m.insert("offset", v.to_string());
        }
        self.sort.url_encode(m);
        m.insert("group_permissions", self.group_permissions.to_string());
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AppSort {
    ByAppId { order: Order, start: Option<Uuid> },
}

impl UrlEncode for AppSort {
    fn url_encode(&self, m: &mut HashMap<&'static str, String>) {
        match *self {
            AppSort::ByAppId {
                ref order,
                ref start,
            } => {
                m.insert("sort", format!("app_id:{}", order));
                if let Some(v) = start {
                    m.insert("start", v.to_string());
                }
            }
        }
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
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/sys/v1/apps?{q}", q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

impl SdkmsClient {
    pub fn list_apps(&self, query_params: &ListAppsParams) -> Result<Vec<App>> {
        self.execute::<OperationListApps>(&(), (), query_params)
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
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/sys/v1/apps/{id}?{q}", id = p.0, q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

impl SdkmsClient {
    pub fn get_app(&self, id: &Uuid, query_params: &GetAppParams) -> Result<App> {
        self.execute::<OperationGetApp>(&(), (id,), query_params)
    }
}

pub struct OperationCreateApp;
#[allow(unused)]
impl Operation for OperationCreateApp {
    type PathParams = ();
    type QueryParams = GetAppParams;
    type Body = AppRequest;
    type Output = App;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/sys/v1/apps?{q}", q = q.encode())
    }
}

impl SdkmsClient {
    pub fn create_app(&self, query_params: &GetAppParams, req: &AppRequest) -> Result<App> {
        self.execute::<OperationCreateApp>(req, (), query_params)
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
        Method::Patch
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/sys/v1/apps/{id}?{q}", id = p.0, q = q.encode())
    }
}

impl SdkmsClient {
    pub fn update_app(
        &self,
        id: &Uuid,
        query_params: &GetAppParams,
        req: &AppRequest,
    ) -> Result<App> {
        self.execute::<OperationUpdateApp>(req, (id,), query_params)
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
        Method::Delete
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/sys/v1/apps/{id}", id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

impl SdkmsClient {
    pub fn delete_app(&self, id: &Uuid) -> Result<()> {
        self.execute::<OperationDeleteApp>(&(), (id,), &())
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
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!(
            "/sys/v1/apps/{id}/reset_secret?{q}",
            id = p.0,
            q = q.encode()
        )
    }
}

impl SdkmsClient {
    pub fn reset_app_secret(
        &self,
        id: &Uuid,
        query_params: &GetAppParams,
        req: &AppResetSecretRequest,
    ) -> Result<App> {
        self.execute::<OperationResetAppSecret>(req, (id,), query_params)
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
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/sys/v1/apps/{id}/credential", id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

impl SdkmsClient {
    pub fn get_app_credential(&self, id: &Uuid) -> Result<AppCredentialResponse> {
        self.execute::<OperationGetAppCredential>(&(), (id,), &())
    }
    pub fn request_approval_to_get_app_credential(
        &self,
        id: &Uuid,
        description: Option<String>,
    ) -> Result<PendingApproval<OperationGetAppCredential>> {
        self.request_approval::<OperationGetAppCredential>(&(), (id,), &(), description)
    }
}
