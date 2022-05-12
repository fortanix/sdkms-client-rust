/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExternalRole {
    pub external_role_id: Uuid,
    pub groups: HashMap<Uuid, ExternalRoleMapping>,
    pub kind: ExternalRoleKind,
    pub last_synced: Time,
    pub name: String,
    pub source_id: Uuid,
    pub acct_id: Uuid,
}

/// Type of an external role.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum ExternalRoleKind {
    LdapGroup,
}

#[derive(Copy, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct ExternalRoleMapping {
    #[serde(default)]
    pub users: Option<UserGroupRole>,
    #[serde(default)]
    pub apps: Option<AppPermissions>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct ExternalRoleRequest {
    #[serde(default)]
    pub add_groups: Option<HashMap<Uuid, ExternalRoleMapping>>,
    #[serde(default)]
    pub del_groups: Option<HashSet<Uuid>>,
    #[serde(default)]
    pub external_role_id: Option<Uuid>,
    #[serde(default)]
    pub kind: Option<ExternalRoleKind>,
    #[serde(default)]
    pub mod_groups: Option<HashMap<Uuid, ExternalRoleMapping>>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub source_id: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ListExternalRolesParams {
    pub group_id: Option<Uuid>,
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
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/external_roles")
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
        Method::DELETE
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/external_roles/{id}", id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

impl SdkmsClient {
    pub fn delete_external_role(&self, id: &Uuid) -> Result<()> {
        self.execute::<OperationDeleteExternalRole>(&(), (id,), None)
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
        Method::GET
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/external_roles/{id}", id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

impl SdkmsClient {
    pub fn get_external_role(&self, id: &Uuid) -> Result<ExternalRole> {
        self.execute::<OperationGetExternalRole>(&(), (id,), None)
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
        Method::GET
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/external_roles?{q}", q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

impl SdkmsClient {
    pub fn list_external_roles(
        &self,
        query_params: Option<&ListExternalRolesParams>,
    ) -> Result<Vec<ExternalRole>> {
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
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/external_roles/{id}/sync", id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

impl SdkmsClient {
    pub fn sync_external_role(&self, id: &Uuid) -> Result<ExternalRole> {
        self.execute::<OperationSyncExternalRole>(&(), (id,), None)
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
        Method::PATCH
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/external_roles/{id}", id = p.0)
    }
}

impl SdkmsClient {
    pub fn update_external_role(
        &self,
        id: &Uuid,
        req: &ExternalRoleRequest,
    ) -> Result<ExternalRole> {
        self.execute::<OperationUpdateExternalRole>(req, (id,), None)
    }
}
