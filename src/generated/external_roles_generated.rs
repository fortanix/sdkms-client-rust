/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExternalRole {
    pub external_role_id: Uuid,
    pub groups: HashMap<Uuid,ExternalRoleMapping>,
    pub kind: ExternalRoleKind,
    pub last_synced: Time,
    pub name: String,
    pub source_id: Uuid,
    pub acct_id: Uuid
}

/// Type of an external role.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum ExternalRoleKind {
    LdapGroup
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct ExternalRoleMapping {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<UserGroupRole>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apps: Option<AppPermissions>
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct ExternalRoleRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_groups: Option<HashMap<Uuid,ExternalRoleMapping>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub del_groups: Option<HashSet<Uuid>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<ExternalRoleKind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mod_groups: Option<HashMap<Uuid,ExternalRoleMapping>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<Uuid>
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ListExternalRolesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<Uuid>
}

impl UrlEncode for ListExternalRolesParams {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        if let Some(ref v) = self.group_id {
            m.insert("group_id".to_string(), v.to_string());
        }
    }
}

pub struct OperationCreateExternalRole;
#[allow(unused)]
impl Operation for OperationCreateExternalRole {
    type PathParams = ();
    type QueryParams = ();
    type Body = ExternalRoleRequest;
    type Output = ExternalRole;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/sys/v1/external_roles".to_string()
    }
}

impl SdkmsClient {
    pub fn create_external_role(&self, req: &ExternalRoleRequest) -> Result<ExternalRole> {
        self.execute::<OperationCreateExternalRole>(req, (), None)
    }
}

pub struct OperationDeleteExternalRole;
#[allow(unused)]
impl Operation for OperationDeleteExternalRole {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = ();

    fn method() -> Method {
        Method::Delete
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/external_roles/{external_role_id}", external_role_id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn delete_external_role(&self, external_role_id: &Uuid) -> Result<()> {
        self.execute::<OperationDeleteExternalRole>(&(), (external_role_id,), None)
    }
}

pub struct OperationGetExternalRole;
#[allow(unused)]
impl Operation for OperationGetExternalRole {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = ExternalRole;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/external_roles/{external_role_id}", external_role_id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_external_role(&self, external_role_id: &Uuid) -> Result<ExternalRole> {
        self.execute::<OperationGetExternalRole>(&(), (external_role_id,), None)
    }
}

pub struct OperationListExternalRoles;
#[allow(unused)]
impl Operation for OperationListExternalRoles {
    type PathParams = ();
    type QueryParams = ListExternalRolesParams;
    type Body = ();
    type Output = Vec<ExternalRole>;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/external_roles?{q}", q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn list_external_roles(&self, query_params: Option<&ListExternalRolesParams>) -> Result<Vec<ExternalRole>> {
        self.execute::<OperationListExternalRoles>(&(), (), query_params)
    }
}

pub struct OperationSyncExternalRole;
#[allow(unused)]
impl Operation for OperationSyncExternalRole {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = ExternalRole;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/external_roles/{external_role_id}/sync", external_role_id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn sync_external_role(&self, external_role_id: &Uuid) -> Result<ExternalRole> {
        self.execute::<OperationSyncExternalRole>(&(), (external_role_id,), None)
    }
}

pub struct OperationUpdateExternalRole;
#[allow(unused)]
impl Operation for OperationUpdateExternalRole {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ExternalRoleRequest;
    type Output = ExternalRole;

    fn method() -> Method {
        Method::Patch
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/external_roles/{external_role_id}", external_role_id = p.0)
    }
}

impl SdkmsClient {
    pub fn update_external_role(&self, external_role_id: &Uuid, req: &ExternalRoleRequest) -> Result<ExternalRole> {
        self.execute::<OperationUpdateExternalRole>(req, (external_role_id,), None)
    }
}

