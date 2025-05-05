/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

#[derive(Ord, PartialOrd, Debug, Eq, PartialEq, Hash, Copy, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum ActionType {
    Administrative,
    Auth,
    CryptoOperation,
    RunPlugin,
    Custom,
    Other
}

/// Response parameters to show Audit log details.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EsAuditLog {
    /// Action Type
    pub action_type: ActionType,
    /// Actor Type
    /// Available values are: User, App & Plugin.
    pub actor_type: String,
    /// Audit log message
    pub message: String,
    /// Severity of event
    pub severity: SeverityLevel,
    /// Time of Event
    pub time: AuditLogTime,
    /// UUID of account
    pub acct_id: Uuid,
    /// UUID of Actor (User, App or Plugin)
    pub actor_id: Uuid,
    /// UUIDs of groups involved/used in event
    pub group_ids: Vec<Uuid>,
    /// UUID of entity affected by event. For instance, if a group is created object_id will be UUID of group.
    pub object_id: Uuid,
    /// IP Address of client
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<IpAddr>,
    /// Time taken for event/operation completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_time: Option<Duration>
}

/// Response structure of a single log.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EsAuditLogOuter {
    /// UUID of audit log
    pub _id: String,
    /// Source of audit log
    pub _source: EsAuditLog
}

/// Response for Audit log Query.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EsAuditQueryResponse {
    /// List of audit logs.
    pub hits: Vec<EsAuditLogOuter>
}

/// Query parameters to get audit logs.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LogsParams {
    /// Maximum number of entries to return. Upper limit for max entries is 1000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,
    /// Starting offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<u32>,
    /// Starting time for search. This is EPOCH time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_from: Option<u64>,
    /// Ending time for search. This is EPOCH time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_to: Option<u64>,
    /// Action Type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<Vec<ActionType>>,
    /// Actor Type
    /// Available values are: User, App & Plugin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_type: Option<Vec<String>>,
    /// UUID of Actor (User, App or Plugin)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<Uuid>,
    /// UUID of entity affected by event. For instance, if a group is created object_id will be UUID of group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<Uuid>,
    /// UUID of log after which further logs are required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_id: Option<Uuid>,
    /// Severity of event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<Vec<SeverityLevel>>
}

impl UrlEncode for LogsParams {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        if let Some(ref v) = self.size {
            m.insert("size".to_string(), v.to_string());
        }
        if let Some(ref v) = self.from {
            m.insert("from".to_string(), v.to_string());
        }
        if let Some(ref v) = self.range_from {
            m.insert("range_from".to_string(), v.to_string());
        }
        if let Some(ref v) = self.range_to {
            m.insert("range_to".to_string(), v.to_string());
        }
        if let Some(ref comma_separated_type) = self.action_type {
            comma_separated_type.url_encode(m);
        }
        if let Some(ref comma_separated_type) = self.actor_type {
            comma_separated_type.url_encode(m);
        }
        if let Some(ref v) = self.actor_id {
            m.insert("actor_id".to_string(), v.to_string());
        }
        if let Some(ref v) = self.object_id {
            m.insert("object_id".to_string(), v.to_string());
        }
        if let Some(ref v) = self.previous_id {
            m.insert("previous_id".to_string(), v.to_string());
        }
        if let Some(ref comma_separated_type) = self.severity {
            comma_separated_type.url_encode(m);
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, PartialOrd, Ord, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum SeverityLevel {
    Info,
    Warning,
    Error,
    Critical
}

pub struct OperationGetAllLogs;
#[allow(unused)]
impl Operation for OperationGetAllLogs {
    type PathParams = ();
    type QueryParams = LogsParams;
    type Body = ();
    type Output = EsAuditQueryResponse;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/logs?{q}", q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_all_logs(&self, query_params: Option<&LogsParams>) -> Result<EsAuditQueryResponse> {
        self.execute::<OperationGetAllLogs>(&(), (), query_params)
    }
}

