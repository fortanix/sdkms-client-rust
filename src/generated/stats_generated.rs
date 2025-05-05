/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EsCountStatsLog {
    pub buckets: Vec<OuterEsBucket>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EsStatsCountQueryResponse {
    pub time: EsCountStatsLog
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EsTotalTxn {
    pub buckets: Vec<InnerEsBucket>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InnerEsBucket {
    pub doc_count: u64,
    pub key: Uuid,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique_operations_count: Option<UniqueOperationsCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique_active_sobj_count: Option<UniqueOperationsCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique_active_app_count: Option<UniqueOperationsCount>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OuterEsBucket {
    pub doc_count: u64,
    pub key: u64,
    pub key_as_string: String,
    pub total_txn: EsTotalTxn
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct StatsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_points: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_from: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_to: Option<u64>
}

impl UrlEncode for StatsParams {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        if let Some(ref v) = self.num_points {
            m.insert("num_points".to_string(), v.to_string());
        }
        if let Some(ref v) = self.top_count {
            m.insert("top_count".to_string(), v.to_string());
        }
        if let Some(ref v) = self.range_from {
            m.insert("range_from".to_string(), v.to_string());
        }
        if let Some(ref v) = self.range_to {
            m.insert("range_to".to_string(), v.to_string());
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UniqueOperationsCount {
    pub value: u64
}

pub struct OperationGetAppAggregate;
#[allow(unused)]
impl Operation for OperationGetAppAggregate {
    type PathParams = ();
    type QueryParams = StatsParams;
    type Body = ();
    type Output = EsStatsCountQueryResponse;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/stats/apps?{q}", q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_app_aggregate(&self, query_params: Option<&StatsParams>) -> Result<EsStatsCountQueryResponse> {
        self.execute::<OperationGetAppAggregate>(&(), (), query_params)
    }
}

pub struct OperationGetAppStats;
#[allow(unused)]
impl Operation for OperationGetAppStats {
    type PathParams = (Uuid,);
    type QueryParams = StatsParams;
    type Body = ();
    type Output = EsStatsCountQueryResponse;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/stats/{app_id}/app?{q}", app_id = p.0, q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_app_stats(&self, app_id: &Uuid, query_params: Option<&StatsParams>) -> Result<EsStatsCountQueryResponse> {
        self.execute::<OperationGetAppStats>(&(), (app_id,), query_params)
    }
}

pub struct OperationGetGroupAggregate;
#[allow(unused)]
impl Operation for OperationGetGroupAggregate {
    type PathParams = ();
    type QueryParams = StatsParams;
    type Body = ();
    type Output = EsStatsCountQueryResponse;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/stats/groups?{q}", q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_group_aggregate(&self, query_params: Option<&StatsParams>) -> Result<EsStatsCountQueryResponse> {
        self.execute::<OperationGetGroupAggregate>(&(), (), query_params)
    }
}

pub struct OperationGetGroupStats;
#[allow(unused)]
impl Operation for OperationGetGroupStats {
    type PathParams = (Uuid,);
    type QueryParams = StatsParams;
    type Body = ();
    type Output = EsStatsCountQueryResponse;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/stats/{group_id}/group?{q}", group_id = p.0, q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_group_stats(&self, group_id: &Uuid, query_params: Option<&StatsParams>) -> Result<EsStatsCountQueryResponse> {
        self.execute::<OperationGetGroupStats>(&(), (group_id,), query_params)
    }
}

pub struct OperationGetSobjectStats;
#[allow(unused)]
impl Operation for OperationGetSobjectStats {
    type PathParams = (Uuid,);
    type QueryParams = StatsParams;
    type Body = ();
    type Output = EsStatsCountQueryResponse;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/stats/{key_id}/key?{q}", key_id = p.0, q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_sobject_stats(&self, key_id: &Uuid, query_params: Option<&StatsParams>) -> Result<EsStatsCountQueryResponse> {
        self.execute::<OperationGetSobjectStats>(&(), (key_id,), query_params)
    }
}

