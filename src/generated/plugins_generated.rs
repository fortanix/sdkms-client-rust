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
    Lua
}

/// Query parameters to get Plugins.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ListPluginsParams {
    /// Group for which the associated plugins should be retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<Uuid>,
    /// Maximum number of entries to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    /// Starting offset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    /// Sort plugins in ascending or descending order by Plugin Id.
    #[serde(flatten)]
    pub sort: PluginSort
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
    /// The id of the Account that the plugin belongs to.
    pub acct_id: Uuid,
    /// Timestamp when the plugin was created.
    pub created_at: Time,
    /// Creator of the plugin.
    pub creator: Principal,
    /// The default group a plugin belongs to.
    pub default_group: Uuid,
    /// Description of the plugin.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Is plugin enabled.
    pub enabled: bool,
    /// Timestamp when the plugin was most recently used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lastrun_at: Option<Time>,
    /// Timestamp when the plugin was most recently updated.
    pub lastupdated_at: Time,
    /// If a requester is updating/using a Plugin they must have the relevant
    /// permissions in all Groups that Plugin has access to. But for legacy Plugins,
    /// the requester is required to have relevant permissions in any one of the groups
    /// that Plugin has access to.
    pub legacy_access: bool,
    /// Name of the plugin, which must be unique within an account.
    pub name: String,
    /// Unique id to identify a plugin.
    pub plugin_id: Uuid,
    /// Type of plugin.
    pub plugin_type: PluginType,
    /// Source of plugin. It contains language & source code of plugin. In case of marketplace plugin repo_url & version as well
    pub source: PluginSource,
    /// Set of all the groups that plugin is part of.
    pub groups: HashSet<Uuid>
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct PluginRequest {
    /// The default group a plugin belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_group: Option<Uuid>,
    /// Description of the plugin.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Is plugin enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Name of the plugin, which must be unique within an account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Type of plugin.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugin_type: Option<PluginType>,
    /// A detached OpenPGP signature over the plugin source code.
    /// 
    /// The signature packet must be armored. If the account has a plugin code signing
    /// policy, the signature is required in the following cases:
    /// - when creating a new plugin
    /// - when updating the plugin code
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    /// Request to get source of plugin.
    #[serde(default, rename = "source", skip_serializing_if = "Option::is_none")]
    pub source_req: Option<PluginSourceRequest>,
    /// Set of all the groups that plugin is part of.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_groups: Option<HashSet<Uuid>>,
    /// Set of all the groups that plugin is part of.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub del_groups: Option<HashSet<Uuid>>,
    /// Set of all the groups that plugin is part of.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mod_groups: Option<HashSet<Uuid>>
}

/// Sorting order on listed Plugins.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PluginSort {
    /// Sort plugins by Plugin Id.
    ByPluginId {
        /// Order of sorting(Ascending/Descending).
        order: Order,
        /// Starting offset(UUID of plugin).
        #[serde(skip_serializing_if = "Option::is_none")]
        start: Option<Uuid>
    }
}

impl UrlEncode for PluginSort {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        match *self {
            PluginSort::ByPluginId{ ref order, ref start } => {
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
        code: String
    },
    Inline {
        language: Language,
        code: String
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum PluginSourceRequest {
    FromRepo {
        repo_url: String,
        plugin_name: String,
        version: PluginVersion
    },
    Inline {
        language: Language,
        code: String
    }
}

/// Type of a plugin.
#[derive(Debug, Eq, PartialEq, Copy, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum PluginType {
    Standard,
    Impersonating,
    CustomAlgorithm
}

pub struct OperationCreatePlugin;
#[allow(unused)]
impl Operation for OperationCreatePlugin {
    type PathParams = ();
    type QueryParams = ();
    type Body = PluginRequest;
    type Output = Plugin;

    fn method() -> Method {
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/sys/v1/plugins".to_string()
    }
}

impl SdkmsClient {
    pub fn create_plugin(&self, req: &PluginRequest) -> Result<Plugin> {
        self.execute::<OperationCreatePlugin>(req, (), None)
    }
    pub fn request_approval_to_create_plugin(
        &self, req: &PluginRequest,
        description: Option<String>) -> Result<PendingApproval<OperationCreatePlugin>> {
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
        Method::Delete
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/plugins/{plugin_id}", plugin_id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn delete_plugin(&self, plugin_id: &Uuid) -> Result<()> {
        self.execute::<OperationDeletePlugin>(&(), (plugin_id,), None)
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
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/plugins/{plugin_id}", plugin_id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_plugin(&self, plugin_id: &Uuid) -> Result<Plugin> {
        self.execute::<OperationGetPlugin>(&(), (plugin_id,), None)
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
        Method::Post
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/plugins/{plugin_id}", plugin_id = p.0)
    }
}

impl SdkmsClient {
    pub fn invoke_plugin(&self, plugin_id: &Uuid, req: &serde_json::Value) -> Result<PluginOutput> {
        self.execute::<OperationInvokePlugin>(req, (plugin_id,), None)
    }
    pub fn request_approval_to_invoke_plugin(
        &self, plugin_id: &Uuid, req: &serde_json::Value,
        description: Option<String>) -> Result<PendingApproval<OperationInvokePlugin>> {
        self.request_approval::<OperationInvokePlugin>(req, (plugin_id,), None, description)
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
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/plugins?{q}", q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

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
        Method::Patch
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/plugins/{plugin_id}", plugin_id = p.0)
    }
}

impl SdkmsClient {
    pub fn update_plugin(&self, plugin_id: &Uuid, req: &PluginRequest) -> Result<Plugin> {
        self.execute::<OperationUpdatePlugin>(req, (plugin_id,), None)
    }
    pub fn request_approval_to_update_plugin(
        &self, plugin_id: &Uuid, req: &PluginRequest,
        description: Option<String>) -> Result<PendingApproval<OperationUpdatePlugin>> {
        self.request_approval::<OperationUpdatePlugin>(req, (plugin_id,), None, description)
    }
}

