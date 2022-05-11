/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

/// Language of plugin code.
#[derive(Debug, Eq, PartialEq, Copy, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum Language {
    Lua,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ListPluginsParams {
    pub group_id: Option<Uuid>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    #[serde(flatten)]
    pub sort: PluginSort,
}

impl UrlEncode for ListPluginsParams {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        if let Some(ref v) = self.group_id {
            m.insert("group_id".to_string(), v.to_string());
        }
        if let Some(ref v) = self.limit {
            m.insert("limit".to_string(), v.to_string());
        }
        if let Some(ref v) = self.offset {
            m.insert("offset".to_string(), v.to_string());
        }
        self.sort.url_encode(m);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Plugin {
    pub acct_id: Uuid,
    pub created_at: Time,
    pub creator: Principal,
    pub default_group: Uuid,
    #[serde(default)]
    pub description: Option<String>,
    pub enabled: bool,
    #[serde(default)]
    pub lastrun_at: Option<Time>,
    pub lastupdated_at: Time,
    pub legacy_access: bool,
    pub name: String,
    pub plugin_id: Uuid,
    pub plugin_type: PluginType,
    pub source: PluginSource,
    pub groups: HashSet<Uuid>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct PluginRequest {
    #[serde(default)]
    pub default_group: Option<Uuid>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub plugin_type: Option<PluginType>,
    #[serde(rename = "source", default)]
    pub source_req: Option<PluginSourceRequest>,
    pub add_groups: Option<HashSet<Uuid>>,
    pub del_groups: Option<HashSet<Uuid>>,
    pub mod_groups: Option<HashSet<Uuid>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PluginSort {
    ByPluginId { order: Order, start: Option<Uuid> },
}

impl UrlEncode for PluginSort {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        match *self {
            PluginSort::ByPluginId {
                ref order,
                ref start,
            } => {
                m.insert("sort".to_string(), format!("plugin_id:{}", order));
                if let Some(v) = start {
                    m.insert("start".to_string(), v.to_string());
                }
            }
        }
    }
}

/// Plugin code that will be executed inside SGX enclave.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum PluginSource {
    FromRepo {
        repo_url: String,
        name: String,
        version: PluginVersion,
        language: Language,
        code: String,
    },
    Inline {
        language: Language,
        code: String,
    },
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum PluginSourceRequest {
    FromRepo {
        repo_url: String,
        plugin_name: String,
        version: PluginVersion,
    },
    Inline {
        language: Language,
        code: String,
    },
}

/// Type of a plugin.
#[derive(Debug, Eq, PartialEq, Copy, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum PluginType {
    Standard,
    Impersonating,
    CustomAlgorithm,
}

pub struct OperationCreatePlugin;
#[allow(unused)]
impl Operation for OperationCreatePlugin {
    type PathParams = ();
    type QueryParams = ();
    type Body = PluginRequest;
    type Output = Plugin;

    fn method() -> Method {
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/plugins")
    }
}

impl SdkmsClient {
    pub fn create_plugin(&self, req: &PluginRequest) -> Result<Plugin> {
        self.execute::<OperationCreatePlugin>(req, (), None)
    }
    pub fn request_approval_to_create_plugin(
        &self,
        req: &PluginRequest,
        description: Option<String>,
    ) -> Result<PendingApproval<OperationCreatePlugin>> {
        self.request_approval::<OperationCreatePlugin>(req, (), None, description)
    }
}

pub struct OperationDeletePlugin;
#[allow(unused)]
impl Operation for OperationDeletePlugin {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = ();

    fn method() -> Method {
        Method::DELETE
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/plugins/{id}", id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

impl SdkmsClient {
    pub fn delete_plugin(&self, id: &Uuid) -> Result<()> {
        self.execute::<OperationDeletePlugin>(&(), (id,), None)
    }
}

pub struct OperationGetPlugin;
#[allow(unused)]
impl Operation for OperationGetPlugin {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = Plugin;

    fn method() -> Method {
        Method::GET
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/plugins/{id}", id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

impl SdkmsClient {
    pub fn get_plugin(&self, id: &Uuid) -> Result<Plugin> {
        self.execute::<OperationGetPlugin>(&(), (id,), None)
    }
}

pub struct OperationInvokePlugin;
#[allow(unused)]
impl Operation for OperationInvokePlugin {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = serde_json::Value;
    type Output = PluginOutput;

    fn method() -> Method {
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/plugins/{id}", id = p.0)
    }
}

impl SdkmsClient {
    pub fn invoke_plugin(&self, id: &Uuid, req: &serde_json::Value) -> Result<PluginOutput> {
        self.execute::<OperationInvokePlugin>(req, (id,), None)
    }
    pub fn request_approval_to_invoke_plugin(
        &self,
        id: &Uuid,
        req: &serde_json::Value,
        description: Option<String>,
    ) -> Result<PendingApproval<OperationInvokePlugin>> {
        self.request_approval::<OperationInvokePlugin>(req, (id,), None, description)
    }
}

pub struct OperationListPlugins;
#[allow(unused)]
impl Operation for OperationListPlugins {
    type PathParams = ();
    type QueryParams = ListPluginsParams;
    type Body = ();
    type Output = Vec<Plugin>;

    fn method() -> Method {
        Method::GET
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/plugins?{q}", q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        None
    }
}

impl SdkmsClient {
    pub fn list_plugins(&self, query_params: Option<&ListPluginsParams>) -> Result<Vec<Plugin>> {
        self.execute::<OperationListPlugins>(&(), (), query_params)
    }
}

pub struct OperationUpdatePlugin;
#[allow(unused)]
impl Operation for OperationUpdatePlugin {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = PluginRequest;
    type Output = Plugin;

    fn method() -> Method {
        Method::PATCH
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/plugins/{id}", id = p.0)
    }
}

impl SdkmsClient {
    pub fn update_plugin(&self, id: &Uuid, req: &PluginRequest) -> Result<Plugin> {
        self.execute::<OperationUpdatePlugin>(req, (id,), None)
    }
    pub fn request_approval_to_update_plugin(
        &self,
        id: &Uuid,
        req: &PluginRequest,
        description: Option<String>,
    ) -> Result<PendingApproval<OperationUpdatePlugin>> {
        self.request_approval::<OperationUpdatePlugin>(req, (id,), None, description)
    }
}
