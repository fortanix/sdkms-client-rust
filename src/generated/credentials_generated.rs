/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::*;

/// Authentication configuration for Azure integrations that use Credential Objects.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "$type")]
pub enum AzureCredentialAuth {
    /// OAuth2 client credentials with a client secret value.
    /// The `tenant_id` and `client_id` is here as well as we
    /// need these to get a token from the client credentials flow.
    ClientSecret {
        /// Directory (tenant) ID of the Azure application
        tenant_id: String,
        /// Application (client) ID of the Azure application
        client_id: String,
        /// Reference to a `Secret` sobject containing the Azure app's client secret value
        client_secret: SobjectId
    },
    /// OAuth2 client credentials with a certificate.
    /// Azure AD verifies the JWT using the uploaded certificate's public key
    Certificate {
        /// Directory (tenant) ID of the Azure application
        tenant_id: String,
        /// Application (client) ID of the Azure application
        client_id: String,
        /// Reference to a `Certificate` sobject used to compute
        /// the SHA-1 thumbprint (`x5t` header) in the JWT assertion.
        client_cert: SobjectId,
        /// Reference to an `RSA` sobject where the private key is used to sign the JWT assertion
        client_key: SobjectId
    }
}

/// Describes the authentication type for a given integration.
/// 
/// This type is the "backbone" to the credential objects in DSM as it's what
/// contains the relevant information and references to the sobject(s) needed to authenticate
/// to external systems/services.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(content = "body", tag = "$type")]
pub enum CredentialDetails {
    /// Credential details for an OCI Key Vault Integration.
    Oci (
        OciCredentialAuth
    ),
    /// Credential details for Azure Monitor Logs integrations.
    Azure (
        AzureCredentialAuth
    ),
    /// Credential details for LDAP integrations.
    Ldap (
        LdapCredentialAuth
    )
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub enum CredentialSort {
    /// Sort the credentials on the basis of the credential_id
    ByCredentialId {
        order: Order
    },
    /// Sort credentials on the basis of the credential name
    ByCredentialName {
        order: Order
    }
}

impl UrlEncode for CredentialSort {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        match *self {
            CredentialSort::ByCredentialId{ ref order } => {
                m.insert("sort_by".to_string(), format!("credential_id:{}", order));
            }
            CredentialSort::ByCredentialName{ ref order } => {
                m.insert("sort_by".to_string(), format!("credential_name:{}", order));
            }
        }
    }
}

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct IntegrationCredential {
    /// The time the credential was created
    pub created_at: Time,
    /// The principal that created the credential
    pub creator: Principal,
    /// The information for a credential to a specific integration for DSM
    pub credential_details: CredentialDetails,
    pub credential_id: Uuid,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the group that this credential is associated with
    pub group_id: Uuid,
    /// The human readable name of the credential
    pub name: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IntegrationCredentialCreateRequest {
    /// The information for a credential to a specific integration for DSM
    pub credential_details: CredentialDetails,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the group that this credential is associated with
    pub group_id: Uuid,
    /// The human readable name of the credential
    pub name: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IntegrationCredentialRotationRequest {
    /// The kind of rotation that the credential will undergo. 
    pub method: RotationMethod
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct IntegrationCredentialUpdateRequest {
    /// The information for a credential to a specific integration for DSM
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential_details: Option<CredentialDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The human readable name of the credential
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>
}

/// Credential details for an LDAP auth configuration.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", tag = "$type")]
pub enum LdapCredentialAuth {
    /// Details for an LDAP service account and its password.
    ServiceAccountWithPassword {
        /// Distinguished name of the service account.
        dn: String,
        /// Information about the service account password.
        password: LdapCredentialPassword
    }
}

/// Information about a password used in an [`LdapCredentialAuth`] object.
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", tag = "source")]
pub enum LdapCredentialPassword {
    Sobject {
        /// The ID of the security object containing the password. This must point
        /// to a security object of type Secret, consisting of valid UTF-8 bytes.
        sobject_id: SobjectId
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ListCredentialsMetadata {
    pub total_count: usize,
    pub filtered_count: usize
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListCredentialsParams {
    /// Specify how to sort the credentials returned in the response
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub sort: Option<CredentialSort>,
    /// Set the max number of credentials to be returned in the response (default: 1000).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    /// Skip first n (offset) matches
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    /// Allows for more specific filters on the credential.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>
}

impl UrlEncode for ListCredentialsParams {
    fn url_encode(&self, m: &mut HashMap<String, String>) {
        self.sort.url_encode(m);
        if let Some(ref v) = self.limit {
            m.insert("limit".to_string(), v.to_string());
        }
        if let Some(ref v) = self.offset {
            m.insert("offset".to_string(), v.to_string());
        }
        if let Some(ref v) = self.filter {
            m.insert("filter".to_string(), v.to_string());
        }
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ListCredentialsResponse {
    pub items: Vec<IntegrationCredential>,
    pub metadata: ListCredentialsMetadata
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "$type")]
pub enum OciCredentialAuth {
    /// **NOTE**: This is not to be confused with a static long-lived token.
    ApiKey {
        /// The user's OCID from Oracle who will have the public key
        /// configured as an "API Key" on Oracle servers. That public
        /// key is part of the key-pair represented by `signing_key`.
        user_ocid: String,
        /// The tenant OCID from Oracle
        tenant_ocid: String,
        /// A reference to the security object which will be used
        /// to sign requests to OCI.
        signing_key: SobjectId
    }
}

/// Describes the how to rotate the credential and any additional information
/// that is required for that to happen successfully.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "$type")]
pub enum RotationMethod {
    /// The [`IntegrationCredential`] being rotated will have the
    /// underlying security objects generated by DSM based on the
    /// current parameters.
    Generate {

    }
}

/// A wrapper type to provide better clarity that the id referenced
/// is a security object.
pub type SobjectId = Uuid;

pub struct OperationCreateCredential;
#[allow(unused)]
impl Operation for OperationCreateCredential {
    type PathParams = ();
    type QueryParams = ();
    type Body = IntegrationCredentialCreateRequest;
    type Output = IntegrationCredential;

    fn method() -> Method {
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        "/sys/v1/credentials".to_string()
    }
}

impl SdkmsClient {
    pub fn create_credential(&self, req: &IntegrationCredentialCreateRequest) -> Result<IntegrationCredential> {
        self.execute::<OperationCreateCredential>(req, (), None)
    }
}

pub struct OperationDeleteCredential;
#[allow(unused)]
impl Operation for OperationDeleteCredential {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = ();

    fn method() -> Method {
        Method::DELETE
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/credentials/{cred_id}", cred_id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn delete_credential(&self, cred_id: &Uuid) -> Result<()> {
        self.execute::<OperationDeleteCredential>(&(), (cred_id,), None)
    }
}

pub struct OperationGetAllCredentials;
#[allow(unused)]
impl Operation for OperationGetAllCredentials {
    type PathParams = ();
    type QueryParams = ListCredentialsParams;
    type Body = ();
    type Output = ListCredentialsResponse;

    fn method() -> Method {
        Method::GET
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/credentials?{q}", q = q.encode())
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_all_credentials(&self, query_params: Option<&ListCredentialsParams>) -> Result<ListCredentialsResponse> {
        self.execute::<OperationGetAllCredentials>(&(), (), query_params)
    }
}

pub struct OperationGetCredential;
#[allow(unused)]
impl Operation for OperationGetCredential {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = ();
    type Output = IntegrationCredential;

    fn method() -> Method {
        Method::GET
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/credentials/{cred_id}", cred_id = p.0)
    }
    fn to_body(body: &Self::Body) -> Option<serde_json::Value> { None }}

impl SdkmsClient {
    pub fn get_credential(&self, cred_id: &Uuid) -> Result<IntegrationCredential> {
        self.execute::<OperationGetCredential>(&(), (cred_id,), None)
    }
}

pub struct OperationRotateCredential;
#[allow(unused)]
impl Operation for OperationRotateCredential {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = IntegrationCredentialRotationRequest;
    type Output = IntegrationCredential;

    fn method() -> Method {
        Method::POST
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/credentials/{cred_id}/rotate", cred_id = p.0)
    }
}

impl SdkmsClient {
    pub fn rotate_credential(&self, cred_id: &Uuid, req: &IntegrationCredentialRotationRequest) -> Result<IntegrationCredential> {
        self.execute::<OperationRotateCredential>(req, (cred_id,), None)
    }
}

pub struct OperationUpdateCredential;
#[allow(unused)]
impl Operation for OperationUpdateCredential {
    type PathParams = (Uuid,);
    type QueryParams = ();
    type Body = IntegrationCredentialUpdateRequest;
    type Output = IntegrationCredential;

    fn method() -> Method {
        Method::PATCH
    }
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: Option<&Self::QueryParams>) -> String {
        format!("/sys/v1/credentials/{cred_id}", cred_id = p.0)
    }
}

impl SdkmsClient {
    pub fn update_credential(&self, cred_id: &Uuid, req: &IntegrationCredentialUpdateRequest) -> Result<IntegrationCredential> {
        self.execute::<OperationUpdateCredential>(req, (cred_id,), None)
    }
}

