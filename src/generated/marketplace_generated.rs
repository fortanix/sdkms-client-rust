/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GetMarketplaceParams {
    pub repo_url: String
}

impl UrlEncode for GetMarketplaceParams {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        m.insert("repo_url".to_string(), self.repo_url.to_string());
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MarketplacePlugin {
    pub name: String,
    pub versions: HashMap<PluginVersion,Option<String>>
}

pub struct OperationGetMarketplace;
#[allow(unused)]
impl Operation for OperationGetMarketplace {
    type PathParams = ();
    type QueryParams = GetMarketplaceParams;
    type Body = ();
    type Output = Vec<MarketplacePlugin>;

    fn method() -> Method {
        Method::GET
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/marketplace?{q}", q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_marketplace(&self, query_params: Option<&GetMarketplaceParams>) -> Result<Vec<MarketplacePlugin>> {
        self.execute::<OperationGetMarketplace>(&(), (), query_params)
    }
}

