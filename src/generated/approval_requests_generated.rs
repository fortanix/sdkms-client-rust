/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;
use serde::{Deserialize, Serialize};

/// A Principal who can approve or deny an approval request.
#[derive(Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ReviewerPrincipal {
    App(Uuid),
    User(Uuid),
}

/// Approval request status.
#[derive(Debug, Eq, PartialEq, Copy, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Denied,
    Failed,
}

/// Identifies an object acted upon by an approval request.
#[derive(Copy, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ApprovalSubject {
    Group(Uuid),
    Sobject(Uuid),
    App(Uuid),
    Plugin(Uuid),
    Account(Uuid),
    NewAccount,
}

/// Reviewer of an approval request.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct Reviewer {
    #[serde(flatten)]
    pub entity: ReviewerPrincipal,
    #[serde(default)]
    pub requires_password: bool,
    #[serde(default)]
    pub requires_2fa: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApprovalRequest {
    pub acct_id: Uuid,
    pub approvers: Vec<ReviewerPrincipal>,
    #[serde(default)]
    pub body: Option<serde_json::Value>,
    pub created_at: Time,
    #[serde(default)]
    pub denier: Option<ReviewerPrincipal>,
    #[serde(default)]
    pub description: Option<String>,
    pub expiry: Time,
    pub method: String,
    pub operation: String,
    pub request_id: Uuid,
    pub requester: Principal,
    #[serde(default)]
    pub reviewers: Option<Vec<Reviewer>>,
    pub status: ApprovalStatus,
    #[serde(default)]
    pub subjects: Option<HashSet<ApprovalSubject>>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct ApprovalRequestRequest {
    #[serde(default)]
    pub body: Option<serde_json::Value>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub method: Option<String>,
    #[serde(default)]
    pub operation: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ListApprovalRequestsParams {
    pub requester: Option<Uuid>,
    pub reviewer: Option<Uuid>,
    pub subject: Option<Uuid>,
    pub status: Option<ApprovalStatus>,
}

impl UrlEncode for ListApprovalRequestsParams {
    fn url_encode(&self, m: &mut HashMap<&'static str, String>) {
        if let Some(ref v) = self.requester {
            m.insert("requester", v.to_string());
        }
        if let Some(ref v) = self.reviewer {
            m.insert("reviewer", v.to_string());
        }
        if let Some(ref v) = self.subject {
            m.insert("subject", v.to_string());
        }
        if let Some(ref v) = self.status {
            m.insert("status", v.to_string());
        }
    }
}

#[derive(Debug, Eq, PartialEq, Default, Serialize, Deserialize, Clone)]
pub struct ApproveRequest {
    /// Password is required if the approval policy requires password authentication.
    pub password: Option<String>,
    /// U2F is required if the approval policy requires two factor authentication.
    pub u2f: Option<U2fAuthRequest>,
}

pub struct OperationListApprovalRequests;
#[allow(unused)]
impl Operation for OperationListApprovalRequests {
    type PathParams = ();
    type QueryParams = ListApprovalRequestsParams;
    type Body = ();
    type Output = Vec<ApprovalRequest>;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/approval_requests?{q}", q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

impl SdkmsClient {
    pub fn list_approval_requests(
        &self,
        query_params: Option<&ListApprovalRequestsParams>,
    ) -> Result<Vec<ApprovalRequest>> {
        self.execute::<OperationListApprovalRequests>(&(), (), query_params)
    }
}

pub struct OperationGetApprovalRequest;
#[allow(unused)]
impl Operation for OperationGetApprovalRequest {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = ApprovalRequest;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/approval_requests/{id}", id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

impl SdkmsClient {
    pub fn get_approval_request(&self, id: &Uuid) -> Result<ApprovalRequest> {
        self.execute::<OperationGetApprovalRequest>(&(), (id,), None)
    }
}

pub struct OperationCreateApprovalRequest;
#[allow(unused)]
impl Operation for OperationCreateApprovalRequest {
    type PathParams = ();
    type QueryParams = ();
    type Body = ApprovalRequestRequest;
    type Output = ApprovalRequest;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/approval_requests")
    }
}

impl SdkmsClient {
    pub fn create_approval_request(&self, req: &ApprovalRequestRequest) -> Result<ApprovalRequest> {
        self.execute::<OperationCreateApprovalRequest>(req, (), None)
    }
}

pub struct OperationApproveRequest;
#[allow(unused)]
impl Operation for OperationApproveRequest {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ApproveRequest;
    type Output = ApprovalRequest;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/approval_requests/{id}/approve", id = p.0)
    }
}

impl SdkmsClient {
    pub fn approve_request(&self, id: &Uuid, req: &ApproveRequest) -> Result<ApprovalRequest> {
        self.execute::<OperationApproveRequest>(req, (id,), None)
    }
}

pub struct OperationDenyRequest;
#[allow(unused)]
impl Operation for OperationDenyRequest {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = ApprovalRequest;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/approval_requests/{id}/deny", id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

impl SdkmsClient {
    pub fn deny_request(&self, id: &Uuid) -> Result<ApprovalRequest> {
        self.execute::<OperationDenyRequest>(&(), (id,), None)
    }
}

pub struct OperationGetApprovalRequestResult;
#[allow(unused)]
impl Operation for OperationGetApprovalRequestResult {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = ApprovableResult;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/approval_requests/{id}/result", id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

impl SdkmsClient {
    pub fn get_approval_request_result(&self, id: &Uuid) -> Result<ApprovableResult> {
        self.execute::<OperationGetApprovalRequestResult>(&(), (id,), None)
    }
}

pub struct OperationDeleteApprovalRequest;
#[allow(unused)]
impl Operation for OperationDeleteApprovalRequest {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = ();

    fn method() -> Method {
        Method::Delete
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/approval_requests/{id}", id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

impl SdkmsClient {
    pub fn delete_approval_request(&self, id: &Uuid) -> Result<()> {
        self.execute::<OperationDeleteApprovalRequest>(&(), (id,), None)
    }
}
