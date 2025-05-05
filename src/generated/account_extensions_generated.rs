/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct AccountExtension {
    pub acct_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cryptographic_policy: Option<CryptographicPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_history_policy: Option<KeyHistoryPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_metadata_policy: Option<KeyMetadataPolicy>,
    pub custom_metadata: HashMap<String,String>,
    pub custom_metadata_attributes: HashMap<String,CustomAttributeSearchMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_code_signing_policy: Option<PluginCodeSigningPolicy>,
    pub mark_key_disabled_when_deactivated: bool
}

/// The model used to create a new account extension.
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct AccountExtensionCreateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cryptographic_policy: Option<CryptographicPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_history_policy: Option<KeyHistoryPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_metadata_policy: Option<KeyMetadataPolicy>,
    pub custom_metadata: HashMap<String,String>,
    pub custom_metadata_attributes: HashMap<String,CustomAttributeSearchMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_code_signing_policy: Option<PluginCodeSigningPolicy>,
    pub mark_key_disabled_when_deactivated: bool
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct AccountExtensionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cryptographic_policy: Option<Removable<CryptographicPolicy>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_history_policy: Option<Removable<KeyHistoryPolicy>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_metadata_policy: Option<Removable<KeyMetadataPolicy>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_metadata: Option<HashMap<String,String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_metadata_attributes: Option<HashMap<String,CustomAttributeSearchMetadata>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_code_signing_policy: Option<Removable<PluginCodeSigningPolicy>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_key_disabled_when_deactivated: Option<bool>
}

pub struct OperationCreateAccountExtension;
#[allow(unused)]
impl Operation for OperationCreateAccountExtension {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = AccountExtensionCreateRequest;
    type Output = AccountExtension;

    fn method() -> Method {
        Method::Put
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/account_extensions/{acct_id}", acct_id = p.0)
    }
}

impl SdkmsClient {
    pub fn create_account_extension(&self, acct_id: &Uuid, req: &AccountExtensionCreateRequest) -> Result<AccountExtension> {
        self.execute::<OperationCreateAccountExtension>(req, (acct_id,), None)
    }
}

pub struct OperationGetAccountExtension;
#[allow(unused)]
impl Operation for OperationGetAccountExtension {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = AccountExtension;

    fn method() -> Method {
        Method::Get
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/account_extensions/{acct_id}", acct_id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_account_extension(&self, acct_id: &Uuid) -> Result<AccountExtension> {
        self.execute::<OperationGetAccountExtension>(&(), (acct_id,), None)
    }
}

pub struct OperationUpdateAccountExtension;
#[allow(unused)]
impl Operation for OperationUpdateAccountExtension {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = AccountExtensionRequest;
    type Output = AccountExtension;

    fn method() -> Method {
        Method::Patch
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/account_extensions/{acct_id}", acct_id = p.0)
    }
}

impl SdkmsClient {
    pub fn update_account_extension(&self, acct_id: &Uuid, req: &AccountExtensionRequest) -> Result<AccountExtension> {
        self.execute::<OperationUpdateAccountExtension>(req, (acct_id,), None)
    }
}

