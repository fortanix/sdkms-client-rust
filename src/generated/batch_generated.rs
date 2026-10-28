/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

#[derive(Debug, PartialEq, Copy, Serialize, Deserialize, Clone)]
pub enum BatchExecutionType {
    Serial,
    Unordered
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BatchRequest {
    Batch (
        BatchRequestList
    ),
    SingleItem (
        BatchRequestItem
    )
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatchRequestItem {
    pub method: String,
    pub operation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatchRequestList {
    pub batch_execution_type: BatchExecutionType,
    pub items: Vec<BatchRequest>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BatchResponse {
    Batch (
        BatchResponseList
    ),
    SingleItem (
        BatchResponseObject
    )
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatchResponseList {
    #[serde(default)]
    pub items: Vec<BatchResponse>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BatchResponseObject {
    Result {
        status: u16,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        body: Option<serde_json::Value>
    },
    Skipped {
        reason: String
    }
}

pub struct OperationBatch;
#[allow(unused)]
impl Operation for OperationBatch {
    type PathParams = ();
    type QueryParams = ();
    type Body = BatchRequest;
    type Output = BatchResponse;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/batch/v1".to_string()
    }
}

impl SdkmsClient {
    pub fn batch(&self, req: &BatchRequest) -> Result<BatchResponse> {
        self.execute::<OperationBatch>(req, (), None)
    }
    pub fn request_approval_to_batch(
        &self, req: &BatchRequest,
        description: Option<String>) -> Result<PendingApproval<OperationBatch>> {
        self.request_approval::<OperationBatch>(req, (), None, description)
    }
}

