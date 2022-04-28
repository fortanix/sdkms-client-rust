/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct HealthParams {
    pub consistency: Option<String>,
    pub check_queues: bool
}

impl UrlEncode for HealthParams {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        if let Some(ref v) = self.consistency {
            m.insert("consistency".to_string(), v.to_string());
        }
        m.insert("check_queues".to_string(), self.check_queues.to_string());
    }
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
    pub base_dn: Option<String>,
    pub filters: Vec<LdapSearchFilter>,
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
    pub mail: Option<String>,
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
    pub password: String,
    pub account_role: Option<AccountRole>
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct LdapTestRequest {
    pub ldap: AuthConfigLdap,
    pub test_credentials: Option<LdapTestCredentials>
}

pub struct OperationGetHealth;
#[allow(unused)]
impl Operation for OperationGetHealth {
    type PathParams = ();
    type QueryParams = HealthParams;
    type Body = ();
    type Output = ();

    fn method() -> Method {
        Method::GET
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

pub struct OperationLdapSearch;
#[allow(unused)]
impl Operation for OperationLdapSearch {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = LdapSearchRequest;
    type Output = Vec<LdapSearchResultEntry>;

    fn method() -> Method {
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/ldap/search/{id}", id = p.0)
    }
}

impl SdkmsClient {
    pub fn ldap_search(&self, id: &Uuid, req: &LdapSearchRequest) -> Result<Vec<LdapSearchResultEntry>> {
        self.execute::<OperationLdapSearch>(req, (id,), None)
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
        Method::GET
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/saml/metadata.xml")
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
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/ldap/test")
    }
}

impl SdkmsClient {
    pub fn test_ldap_config(&self, req: &LdapTestRequest) -> Result<()> {
        self.execute::<OperationTestLdapConfig>(req, (), None)
    }
}

