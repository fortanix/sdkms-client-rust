/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct App {
    /// The type used to create, modify, or view the assigned account roles.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_membership: Option<AppAccountMembership>,
    /// The id of the Account that this application belongs to.
    pub acct_id: Uuid,
    /// Unique id to identify the application.
    pub app_id: Uuid,
    /// The user-defined type of this application.
    pub app_type: String,
    /// The authentication mechanisms for an application.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<AppAuthType>,
    /// Certificate expiration date.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cert_not_after: Option<Time>,
    /// Client configurations that are set on the application level.
    /// App level client configs override those set at group or account level.
    pub client_configurations: ClientConfigurations,
    /// Timestamp when the application was created.
    pub created_at: Time,
    /// Creator of this application.
    pub creator: Principal,
    /// The default group an application belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_group: Option<Uuid>,
    /// Description of this application.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether this application is enabled.
    pub enabled: bool,
    /// Mapping for all groups an application is part of and the permissions it has within each of those groups.
    pub groups: AppGroups,
    /// Interface used with this application (PKCS11, CNG, JCE, KMIP, etc).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interface: Option<String>,
    /// The IPs that are allowed for an application. ipv4 or ipv6 both are acceptable types.
    pub ip_address_policy: IpAddressPolicy,
    pub last_operations: LastAppOperationTimestamp,
    /// Timestamp when the application was most recently used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lastused_at: Option<Time>,
    /// If a requester is updating an App or retrieving its credentials,
    /// they must have the relevant permissions in all Groups that App has access to.
    /// But for legacy Apps, requester is required to have relevant permissions
    /// in any of the groups that App has access to.
    pub legacy_access: bool,
    /// Name of this application, which must be unique within an account.
    pub name: String,
    /// OAuth settings for an app. If enabled, an app can request to act on behalf of a user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth_config: Option<AppOauthConfig>,
    /// Application's role.
    pub role: AppRole
}

#[derive(Debug, Eq, PartialEq, Default, Serialize, Deserialize, Clone)]
pub struct AppAccountMembership {
    pub roles: HashSet<AppAccountRoleDescriptor>
}

#[derive(Debug, Eq, PartialEq, Copy, Hash, Serialize, Deserialize, Clone)]
pub enum AppAccountRole {
    AccountAdministrator,
    AccountMember,
    AccountAuditor
}

#[derive(Debug, Eq, PartialEq, Copy, Hash, Serialize, Deserialize, Clone)]
#[serde(content = "value", tag = "$type")]
pub enum AppAccountRoleDescriptor {
    SystemDefined (
        AppAccountRole
    ),
    Custom (
        Uuid
    )
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
    AwsIam,
    AwsXks,
    GoogleWorkspaceCSE
}

/// App authentication mechanisms.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AppCredential {
    /// Authenticating credentials of an App.
    Secret (
        ZeroizedString
    ),
    /// PKI Certificate based authentication.
    Certificate (
        ZeroizedBlob
    ),
    /// PKI certificate with Trusted CA based authentication.
    TrustedCa (
        TrustedCaCredential
    ),
    /// An App's service account for communicating with Google APIs and Cloud. Google OAuth 2.0
    GoogleServiceAccount {
        /// Policy specifying acceptable access reasons.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        access_reason_policy: Option<GoogleAccessReasonPolicy>,
        /// Mapping for all groups an application is part of and the Gcp specific permissions it has within each of those groups.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        groups: Option<HashMap<Uuid,GcpAppPermissions>>
    },
    /// Authentication using a signed JWT directly as a bearer token.
    SignedJwt {
        valid_issuers: HashSet<String>,
        signing_keys: SigningKeys
    },
    /// LDAP credentials of an App used for authentication.
    Ldap (
        Uuid
    ),
    /// Sign-in credentials to authenticate with AWS for it's services and resources.
    AwsIam {

    },
    /// SigV4 credentials used for AWS XKS APIs
    AwsXks {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        access_key_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        secret_key: Option<ZeroizedString>
    },
    GoogleWorkspaceCse {

    }
}

/// App credential response.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppCredentialResponse {
    /// Unique identifier of the App.
    pub app_id: Uuid,
    /// Credential of an App which determine the App authentication mechanisms.
    pub credential: AppCredential,
    /// Expired app-credentials that may be valid during transitional period.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_credential: Option<PreviousCredential>
}

#[derive(Default, Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct AppGroupMembership {
    pub group_id: Uuid,
    pub roles: HashSet<AppGroupRoleDescriptor>
}

#[derive(Debug, Eq, PartialEq, Copy, Hash, PartialOrd, Ord, Serialize, Deserialize, Clone)]
pub enum AppGroupRole {
    GroupAuditor,
    GroupAdministrator
}

#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Ord, Serialize, Deserialize, Clone)]
#[serde(content = "value", tag = "$type")]
pub enum AppGroupRoleDescriptor {
    SystemDefined (
        AppGroupRole
    ),
    Custom (
        Uuid
    )
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
    /// The type used to create, modify, or view the assigned account roles.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_membership: Option<AppAccountMembership>,
    /// Groups an application wants to be part of. Should belong to atleast one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_groups: Option<AppGroups>,
    /// The user-defined type of this application.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_type: Option<String>,
    /// Client configurations that are set on the application level.
    /// App level client configs override those set at group or account level.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_configurations: Option<ClientConfigurationsRequest>,
    /// Credential for an application which determine the App authentication mechanisms.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<AppCredential>,
    /// Migration period for which credentials(and its sessions) remain valid during api key regeneration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential_migration_period: Option<u32>,
    /// The default group an application belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_group: Option<Uuid>,
    /// Groups an application no longer needs to be a part of. Array of UUID of groups.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub del_groups: Option<HashSet<Uuid>>,
    /// Description of this application.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether this application is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Interface used with this application (PKCS11, CNG, JCE, KMIP, etc).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interface: Option<String>,
    /// The IPs that are allowed for an application. ipv4 or ipv6 both are acceptable types.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_address_policy: Option<IpAddressPolicy>,
    /// Modify the permissions an application has in the groups it belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mod_groups: Option<AppGroups>,
    /// Name of this application, which must be unique within an account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// OAuth settings for an app. If enabled, an app can request to act on behalf of a user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth_config: Option<AppOauthConfig>,
    /// Application's role.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<AppRole>,
    /// Size in bytes of app's secret.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_size: Option<u32>
}

/// Request for resetting the app secret.
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct AppResetSecretRequest {
    /// Size of app's secret in bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_size: Option<u32>,
    /// Time until which previous credentials(or its sessions)
    /// will not be invalidated as the API key gets regenerated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential_migration_period: Option<u32>
}

/// App's role.
#[derive(Debug, Eq, PartialEq, Copy, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AppRole {
    /// Can perform similar actions to an account admin user, but not crypto ops.
    Admin,
    /// Can perform crypto ops
    Crypto
}

/// Sort apps as per given ordering.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum AppSort {
    /// Sort apps on the basis of their app_id.
    ByAppId {
        /// Ascending or Descending order.
        order: Order,
        /// Starting from a particular app_id.
        #[serde(skip_serializing_if = "Option::is_none")]
        start: Option<Uuid>
    },
    /// Sort apps on the basis of their app_name.
    ByAppName {
        /// Ascending or Descending order.
        order: Order,
        /// Starting from a particular app_name.
        #[serde(skip_serializing_if = "Option::is_none")]
        start: Option<String>
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
            AppSort::ByAppName{ ref order, ref start } => {
                m.insert("sort".to_string(), format!("app_name:{}", order));
                if let Some(v) = start {
                    m.insert("start".to_string(), v.to_string());
                }
            }
        }
    }
}

/// Request for assigning a group membership to an (AppRole::Admin) app.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateGroupMembership {
    /// The id of the target group
    pub group_id: Uuid,
    /// The roles being assigned for the group.
    pub membership: AppGroupMembership
}

pub use self::gcp_app_permissions::GcpAppPermissions;
pub mod gcp_app_permissions {
    bitflags_set!{
        pub struct GcpAppPermissions: u64 {
            const CRYPTO_SPACE_GET_INFO = 0x0000000000000001;
            const CRYPTO_SPACE_GET_PUBLIC_KEY = 0x0000000000000002;
        }
    }
}

/// The response for the GetAllGroupMembership endpoint
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetAppGroupMemberships {
    /// Additional information about the group(s)
    pub metadata: GroupMetaData,
    /// The collection of group memberships the entity is a member in
    pub items: Vec<AppGroupMembership>
}

/// Query params for individual App APIs
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GetAppParams {
    /// Flag specifying if group permissions should be returned with the app group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_permissions: Option<bool>,
    /// The App's role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>
}

impl UrlEncode for GetAppParams {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        if let Some(ref v) = self.group_permissions {
            m.insert("group_permissions".to_string(), v.to_string());
        }
        if let Some(ref v) = self.role {
            m.insert("role".to_string(), v.to_string());
        }
    }
}

/// Type for updating a group membership
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupMembershipRequest {
    /// The set of roles to add
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_group_roles: Option<BTreeSet<AppGroupRoleDescriptor>>,
    /// The set of roles to remove
    #[serde(skip_serializing_if = "Option::is_none")]
    pub del_group_roles: Option<BTreeSet<AppGroupRoleDescriptor>>
}

/// Additional information or context regarding the groups the entity
/// holds membership in
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupMetaData {
    /// Whether the entity has been assigned an exclusive "all groups role"
    pub all_groups: bool
}

/// The IPs that are allowed for an application. ipv4 or ipv6 both are acceptable types.
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generic: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tokenization: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tep: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accelerator: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets_management: Option<u64>
}

/// Query params for Get all apps API
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListAppsParams {
    /// Group for which the associated apps should be retrived.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<Uuid>,
    /// Maximum number of apps to return. Default limit is 1001.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    /// Number of apps to skip from the beginning/start.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    /// Sort apps by app_id in ascending or descending order.
    #[serde(flatten)]
    pub sort: AppSort,
    /// Flag specifying if group permissions should be returned with the apps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_permissions: Option<bool>,
    /// Specify role of the apps. If `role=admin` is specified, only admin apps are returned,
    /// otherwise, only crypto apps are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<AppRole>,
    /// User specified filter.
    ///
    /// The following fields can be referenced in the filter:
    /// - `name`
    /// - `app_type`
    /// - `created_at`
    /// - `auth_type`
    /// - `description`
    /// - `enabled`
    /// - `interface`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// Continuation token to continue getting results. It must be the same
    /// token returned from the backend from a previous call, or empty.
    ///
    /// Existence of this query parameter controls the response
    /// (and the backend behavior):
    /// - If specified (including an empty value), the backend returns metadata alongside
    ///   the collection of apps. The metadata will potentially contain a fresh `continuation_token`.
    ///
    ///   Note: If there is a `limit` specified in the request and DSM returns `limit`-many items in the
    ///   response, it will still include a fresh continuation token if there are more items in the collection.
    ///   Additionally, unlike other query parameters, `limit` is not required to remain unchanged in a chain of
    ///   requests with `coninutation_token`s.
    /// - If omitted, the backend returns just a collection of apps with no metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>
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
        if let Some(ref v) = self.group_permissions {
            m.insert("group_permissions".to_string(), v.to_string());
        }
        if let Some(ref v) = self.role {
            m.insert("role".to_string(), v.to_string());
        }
        if let Some(ref v) = self.filter {
            m.insert("filter".to_string(), v.to_string());
        }
        if let Some(ref v) = self.continuation_token {
            m.insert("continuation_token".to_string(), v.to_string());
        }
    }
}

/// The response of the get all apps API
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ListAppsResponse {
    /// A response that includes metadata
    WithMetadata {
        /// The list of apps satisfying the request
        items: Vec<App>,
        /// The metadata associated with the response
        metadata: CollectionMetadata
    },
    /// A response that omits metadata
    WithoutMetadata (
        Vec<App>
    )
}

/// Expired app-credentials that are still valid for a transitional period.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct PreviousCredential {
    /// App authentication mechanisms.
    pub credential: AppCredential,
    /// Validity period of the App credentials.
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

/// `TrustedCa` app auth
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct TrustedCaCredential {
    #[serde(flatten)]
    pub subject: TrustAnchorSubject,
    pub ca_certificate: ZeroizedBlob,
    /// When `true`, revocation status of certificates is checked, and revoked
    /// certificates are rejected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_revocation: Option<bool>
}

pub struct OperationAddGroupMembership;
#[allow(unused)]
impl Operation for OperationAddGroupMembership {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = CreateGroupMembership;
    type Output = AppGroupMembership;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/apps/{app_id}/groups", app_id = p.0)
    }
}

impl SdkmsClient {
    pub fn add_group_membership(&self, app_id: &Uuid, req: &CreateGroupMembership) -> Result<AppGroupMembership> {
        self.execute::<OperationAddGroupMembership>(req, (app_id,), None)
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
        Method::Delete
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/apps/{app_id}", app_id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn delete_app(&self, app_id: &Uuid) -> Result<()> {
        self.execute::<OperationDeleteApp>(&(), (app_id,), None)
    }
}

pub struct OperationDeleteGroupMembership;
#[allow(unused)]
impl Operation for OperationDeleteGroupMembership {
    type PathParams = (Uuid, Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = ();

    fn method() -> Method {
        Method::Delete
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/apps/{app_id}/groups/{group_id}", app_id = p.0, group_id = p.1)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn delete_group_membership(&self, app_id: &Uuid, group_id: &Uuid) -> Result<()> {
        self.execute::<OperationDeleteGroupMembership>(&(), (app_id, group_id,), None)
    }
}

pub struct OperationGetAllGroupMemberships;
#[allow(unused)]
impl Operation for OperationGetAllGroupMemberships {
    type PathParams = (Uuid,);
    type QueryParams = GetGroupsParams;
    type Body = ();
    type Output = GetAppGroupMemberships;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/apps/{app_id}/groups?{q}", app_id = p.0, q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_all_group_memberships(&self, app_id: &Uuid, query_params: Option<&GetGroupsParams>) -> Result<GetAppGroupMemberships> {
        self.execute::<OperationGetAllGroupMemberships>(&(), (app_id,), query_params)
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
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/apps/{app_id}?{q}", app_id = p.0, q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_app(&self, app_id: &Uuid, query_params: Option<&GetAppParams>) -> Result<App> {
        self.execute::<OperationGetApp>(&(), (app_id,), query_params)
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
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/apps/{app_id}/credential", app_id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_app_credential(&self, app_id: &Uuid) -> Result<AppCredentialResponse> {
        self.execute::<OperationGetAppCredential>(&(), (app_id,), None)
    }
    pub fn request_approval_to_get_app_credential(
        &self, app_id: &Uuid,
        description: Option<String>) -> Result<PendingApproval<OperationGetAppCredential>> {
        self.request_approval::<OperationGetAppCredential>(&(), (app_id,), None, description)
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
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/sys/v1/apps/client_configs".to_string()
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_client_configs(&self) -> Result<ClientConfigurations> {
        self.execute::<OperationGetClientConfigs>(&(), (), None)
    }
}

pub struct OperationGetGroupMembership;
#[allow(unused)]
impl Operation for OperationGetGroupMembership {
    type PathParams = (Uuid, Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = AppGroupMembership;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/apps/{app_id}/groups/{group_id}", app_id = p.0, group_id = p.1)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_group_membership(&self, app_id: &Uuid, group_id: &Uuid) -> Result<AppGroupMembership> {
        self.execute::<OperationGetGroupMembership>(&(), (app_id, group_id,), None)
    }
}

pub struct OperationListApps;
#[allow(unused)]
impl Operation for OperationListApps {
    type PathParams = ();
    type QueryParams = ListAppsParams;
    type Body = ();
    type Output = ListAppsResponse;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/apps?{q}", q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn list_apps(&self, query_params: Option<&ListAppsParams>) -> Result<ListAppsResponse> {
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
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/apps/{app_id}/reset_secret?{q}", app_id = p.0, q = q.encode())
    }
}

impl SdkmsClient {
    pub fn reset_app_secret(&self, app_id: &Uuid, query_params: Option<&GetAppParams>, req: &AppResetSecretRequest) -> Result<App> {
        self.execute::<OperationResetAppSecret>(req, (app_id,), query_params)
    }
    pub fn request_approval_to_reset_app_secret(
        &self, app_id: &Uuid, query_params: Option<&GetAppParams>, req: &AppResetSecretRequest,
        description: Option<String>) -> Result<PendingApproval<OperationResetAppSecret>> {
        self.request_approval::<OperationResetAppSecret>(req, (app_id,), query_params, description)
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
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/apps/{app_id}?{q}", app_id = p.0, q = q.encode())
    }
}

impl SdkmsClient {
    pub fn update_app(&self, app_id: &Uuid, query_params: Option<&GetAppParams>, req: &AppRequest) -> Result<App> {
        self.execute::<OperationUpdateApp>(req, (app_id,), query_params)
    }
    pub fn request_approval_to_update_app(
        &self, app_id: &Uuid, query_params: Option<&GetAppParams>, req: &AppRequest,
        description: Option<String>) -> Result<PendingApproval<OperationUpdateApp>> {
        self.request_approval::<OperationUpdateApp>(req, (app_id,), query_params, description)
    }
}

pub struct OperationUpdateGroupMembership;
#[allow(unused)]
impl Operation for OperationUpdateGroupMembership {
    type PathParams = (Uuid, Uuid,);
    type QueryParams = ();
    type Body = GroupMembershipRequest;
    type Output = AppGroupMembership;

    fn method() -> Method {
        Method::Patch
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/apps/{app_id}/groups/{group_id}", app_id = p.0, group_id = p.1)
    }
}

impl SdkmsClient {
    pub fn update_group_membership(&self, app_id: &Uuid, group_id: &Uuid, req: &GroupMembershipRequest) -> Result<AppGroupMembership> {
        self.execute::<OperationUpdateGroupMembership>(req, (app_id, group_id,), None)
    }
}

