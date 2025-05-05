/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct GetAllServicesResponse {
    pub items: Vec<Service>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HealthCheckInfo {
    /// The time the health check was initiated.
    pub initiated_at: Time,
    /// The time the health check finished (regardless of its outcome).
    pub finished_at: Time,
    /// The result of the health check.
    pub result: String
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct HealthParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_queues: Option<bool>
}

impl UrlEncode for HealthParams {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        if let Some(ref v) = self.consistency {
            m.insert("consistency".to_string(), v.to_string());
        }
        if let Some(ref v) = self.check_queues {
            m.insert("check_queues".to_string(), v.to_string());
        }
    }
}

#[derive(Copy, Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub enum HealthStatus {
    Healthy,
    Unhealthy
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HostnameInfo {
    /// The health status of the hostname.
    pub health_status: HealthStatus,
    /// Information about the last completed active health check on the hostname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_health_check: Option<HealthCheckInfo>
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum LdapPrincipal {
    Unresolved {
        email: String
    },
    Resolved {
        dn: String
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct LdapSearchFilter {
    pub name: String,
    pub value: String
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct LdapSearchRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_dn: Option<String>,
    pub filters: Vec<LdapSearchFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_class: Option<String>,
    pub scope: LdapSearchScope
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct LdapSearchResultEntry {
    pub distinguished_name: String,
    pub ldap_object_id: Uuid,
    pub common_name: Vec<String>,
    pub description: Vec<String>,
    pub object_class: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum LdapSearchScope {
    SingleLevel,
    WholeSubtree
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct LdapTestCredentials {
    #[serde(flatten)]
    pub id: LdapPrincipal,
    pub password: ZeroizedString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_role: Option<LdapAccountRole>
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct LdapTestRequest {
    pub ldap: AuthConfigLdap,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_credentials: Option<LdapTestCredentials>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Service {
    pub name: String,
    pub hostnames: HashMap<String,HostnameInfo>
}

pub struct OperationGetAllServices;
#[allow(unused)]
impl Operation for OperationGetAllServices {
    type PathParams = ();
    type QueryParams = ();
    type Body = ();
    type Output = GetAllServicesResponse;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/sys/v1/services".to_string()
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_all_services(&self) -> Result<GetAllServicesResponse> {
        self.execute::<OperationGetAllServices>(&(), (), None)
    }
}

pub struct OperationGetHealth;
#[allow(unused)]
impl Operation for OperationGetHealth {
    type PathParams = ();
    type QueryParams = HealthParams;
    type Body = ();
    type Output = ();

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/health?{q}", q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_health(&self, query_params: Option<&HealthParams>) -> Result<()> {
        self.execute::<OperationGetHealth>(&(), (), query_params)
    }
}

pub struct OperationGetService;
#[allow(unused)]
impl Operation for OperationGetService {
    type PathParams = (String,);
    type QueryParams = ();
    type Body = ();
    type Output = Service;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/services/{name}", name = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_service(&self, name: &String) -> Result<Service> {
        self.execute::<OperationGetService>(&(), (name,), None)
    }
}

pub struct OperationLdapSearch;
#[allow(unused)]
impl Operation for OperationLdapSearch {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = LdapSearchRequest;
    type Output = Vec<LdapSearchResultEntry>;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/ldap/search/{ldap_id}", ldap_id = p.0)
    }
}

impl SdkmsClient {
    pub fn ldap_search(&self, ldap_id: &Uuid, req: &LdapSearchRequest) -> Result<Vec<LdapSearchResultEntry>> {
        self.execute::<OperationLdapSearch>(req, (ldap_id,), None)
    }
}

pub struct OperationSamlSpMetadata;
#[allow(unused)]
impl Operation for OperationSamlSpMetadata {
    type PathParams = ();
    type QueryParams = ();
    type Body = ();
    type Output = Vec<u8>;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/saml/metadata.xml".to_string()
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn saml_sp_metadata(&self) -> Result<Vec<u8>> {
        self.execute::<OperationSamlSpMetadata>(&(), (), None)
    }
}

pub struct OperationTestLdapConfig;
#[allow(unused)]
impl Operation for OperationTestLdapConfig {
    type PathParams = ();
    type QueryParams = ();
    type Body = LdapTestRequest;
    type Output = ();

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/sys/v1/ldap/test".to_string()
    }
}

impl SdkmsClient {
    pub fn test_ldap_config(&self, req: &LdapTestRequest) -> Result<()> {
        self.execute::<OperationTestLdapConfig>(req, (), None)
    }
}

