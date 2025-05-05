/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, Clone)]
pub struct AccountRole {
    pub permissions: AccountPermissions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive: Option<bool>,
    /// If specified, users with this account role will have the specified role
    /// in all groups. The uuid should refer to an existing `Role` of kind
    /// `RoleKind::Group`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_groups_role: Option<Uuid>
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize, Clone)]
pub struct GroupRole {
    pub permissions: GroupPermissions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive: Option<bool>
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListRolesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    #[serde(flatten)]
    pub sort: RoleSort
}

impl UrlEncode for ListRolesParams {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        if let Some(ref v) = self.filter {
            m.insert("filter".to_string(), v.to_string());
        }
        if let Some(ref v) = self.limit {
            m.insert("limit".to_string(), v.to_string());
        }
        self.sort.url_encode(m);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListRolesResponse {
    pub metadata: Metadata,
    pub items: Vec<Role>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Role {
    pub created_at: Time,
    pub creator: Principal,
    pub description: String,
    pub details: RoleDetails,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<RoleKind>,
    pub last_updated_at: Time,
    pub name: String,
    pub role_id: Uuid,
    pub acct_id: Uuid
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum RoleDetails {
    Account (
        AccountRole
    ),
    Group (
        GroupRole
    )
}

#[derive(Debug, Copy, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RoleKind {
    Account,
    Group
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct RoleRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<RoleDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum RoleSort {
    ByRoleId {
        order: Order,
        #[serde(skip_serializing_if = "Option::is_none")]
        start: Option<Uuid>
    }
}

impl UrlEncode for RoleSort {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        match *self {
            RoleSort::ByRoleId{ ref order, ref start } => {
                m.insert("sort".to_string(), format!("role_id:{}", order));
                if let Some(v) = start {
                    m.insert("start".to_string(), v.to_string());
                }
            }
        }
    }
}

pub struct OperationCreateRole;
#[allow(unused)]
impl Operation for OperationCreateRole {
    type PathParams = ();
    type QueryParams = ();
    type Body = RoleRequest;
    type Output = Role;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/sys/v1/roles".to_string()
    }
}

impl SdkmsClient {
    pub fn create_role(&self, req: &RoleRequest) -> Result<Role> {
        self.execute::<OperationCreateRole>(req, (), None)
    }
}

pub struct OperationDeleteRole;
#[allow(unused)]
impl Operation for OperationDeleteRole {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = ();

    fn method() -> Method {
        Method::Delete
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/roles/{custom_role_id}", custom_role_id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn delete_role(&self, custom_role_id: &Uuid) -> Result<()> {
        self.execute::<OperationDeleteRole>(&(), (custom_role_id,), None)
    }
}

pub struct OperationGetRole;
#[allow(unused)]
impl Operation for OperationGetRole {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = Role;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/roles/{custom_role_id}", custom_role_id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_role(&self, custom_role_id: &Uuid) -> Result<Role> {
        self.execute::<OperationGetRole>(&(), (custom_role_id,), None)
    }
}

pub struct OperationListRoles;
#[allow(unused)]
impl Operation for OperationListRoles {
    type PathParams = ();
    type QueryParams = ListRolesParams;
    type Body = ();
    type Output = ListRolesResponse;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/roles?{q}", q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn list_roles(&self, query_params: Option<&ListRolesParams>) -> Result<ListRolesResponse> {
        self.execute::<OperationListRoles>(&(), (), query_params)
    }
}

pub struct OperationUpdateRole;
#[allow(unused)]
impl Operation for OperationUpdateRole {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = RoleRequest;
    type Output = Role;

    fn method() -> Method {
        Method::Patch
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/roles/{custom_role_id}", custom_role_id = p.0)
    }
}

impl SdkmsClient {
    pub fn update_role(&self, custom_role_id: &Uuid, req: &RoleRequest) -> Result<Role> {
        self.execute::<OperationUpdateRole>(req, (custom_role_id,), None)
    }
    pub fn request_approval_to_update_role(
        &self, custom_role_id: &Uuid, req: &RoleRequest,
        description: Option<String>) -> Result<PendingApproval<OperationUpdateRole>> {
        self.request_approval::<OperationUpdateRole>(req, (custom_role_id,), None, description)
    }
}

