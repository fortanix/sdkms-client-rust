/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Group {
    pub acct_id: Uuid,
    #[serde(default)]
    pub approval_policy: Option<ApprovalPolicy>,
    pub created_at: Time,
    pub creator: Principal,
    #[serde(default)]
    pub description: Option<String>,
    pub group_id: Uuid,
    pub name: String,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct GroupRequest {
    #[serde(default)]
    pub approval_policy: Option<ApprovalPolicy>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
}

pub struct OperationListGroups;
#[allow(unused)]
impl Operation for OperationListGroups {
    type PathParams = ();
    type QueryParams = ();
    type Body = ();
    type Output = Vec<Group>;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/sys/v1/groups")
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

impl SdkmsClient {
    pub fn list_groups(&self) -> Result<Vec<Group>> {
        self.execute::<OperationListGroups>(&(), (), &())
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
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/sys/v1/groups/{id}", id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

impl SdkmsClient {
    pub fn get_group(&self, id: &Uuid) -> Result<Group> {
        self.execute::<OperationGetGroup>(&(), (id,), &())
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
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/sys/v1/groups")
    }
}

impl SdkmsClient {
    pub fn create_group(&self, req: &GroupRequest) -> Result<Group> {
        self.execute::<OperationCreateGroup>(req, (), &())
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
        Method::Patch
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/sys/v1/groups/{id}", id = p.0)
    }
}

impl SdkmsClient {
    pub fn update_group(&self, id: &Uuid, req: &GroupRequest) -> Result<Group> {
        self.execute::<OperationUpdateGroup>(req, (id,), &())
    }
    pub fn request_approval_to_update_group(
        &self,
        id: &Uuid,
        req: &GroupRequest,
        description: Option<String>,
    ) -> Result<PendingApproval<OperationUpdateGroup>> {
        self.request_approval::<OperationUpdateGroup>(req, (id,), &(), description)
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
        Method::Delete
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String {
        format!("/sys/v1/groups/{id}", id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

impl SdkmsClient {
    pub fn delete_group(&self, id: &Uuid) -> Result<()> {
        self.execute::<OperationDeleteGroup>(&(), (id,), &())
    }
}
