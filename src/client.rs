/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::api_model::*;
use crate::operations::*;

use headers::{ContentType, HeaderMap, HeaderMapExt, HeaderValue};
use simple_hyper_client::{Bytes, Method, StatusCode};
use simple_hyper_client::blocking::Client as HttpClient;
use simple_hyper_client::hyper::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use std::fmt;
use std::io::Read;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

pub const DEFAULT_API_ENDPOINT: &'static str = "https://sdkms.fortanix.com";

pub type Result<T> = ::std::result::Result<T, Error>;

enum Auth {
    Basic(String),
    Bearer(String),
}

impl Auth {
    fn from_api_key(api_key: &str) -> Self {
        Auth::Basic(api_key.to_owned())
    }

    fn from_user_pass<T: fmt::Display>(username: T, password: &str) -> Self {
        Auth::Basic(base64::encode(format!("{}:{}", username, password)))
    }

    fn format_header(&self) -> HeaderValue {
        let value = match *self {
            Auth::Basic(ref basic) => format!("Basic {}", basic),
            Auth::Bearer(ref bearer) => format!("Bearer {}", bearer),
        };
        let bytes = Bytes::from(value);
        // TODO: return error instead of expect
        HeaderValue::from_maybe_shared(bytes).expect("invalid characters in auth header")
    }
}

/// A builder for [`SdkmsClient`](./struct.SdkmsClient.html)
pub struct SdkmsClientBuilder {
    client: Option<HttpClient>,
    api_endpoint: Option<String>,
    auth: Option<Auth>,
}

impl SdkmsClientBuilder {
    /// This can be used to customize the underlying HTTP client if desired.
    pub fn with_http_client(mut self, client: HttpClient) -> Self {
        self.client = Some(client);
        self
    }
    /// This can be used to set the API endpoint. Otherwise the [default endpoint](./constant.DEFAULT_API_ENDPOINT.html) is used.
    pub fn with_api_endpoint(mut self, api_endpoint: &str) -> Self {
        self.api_endpoint = Some(api_endpoint.to_owned());
        self
    }
    /// This can be used to make API calls without establishing a session.
    /// The API key will be passed along as HTTP Basic auth header on all API calls.
    pub fn with_api_key(mut self, api_key: &str) -> Self {
        self.auth = Some(Auth::from_api_key(api_key));
        self
    }
    /// This can be used to restore an established session.
    pub fn with_access_token(mut self, access_token: &str) -> Self {
        self.auth = Some(Auth::Bearer(access_token.to_owned()));
        self
    }
    /// Build [`SdkmsClient`](./struct.SdkmsClient.html)
    pub fn build(self) -> Result<SdkmsClient> {
        let client = match self.client {
            Some(client) => client,
            None => {
                #[cfg(feature = "native-tls")]
                {
                    use simple_hyper_client::HttpsConnector;
                    use tokio_native_tls::native_tls::TlsConnector;

                    let ssl = TlsConnector::new()?;
                    let connector = HttpsConnector::new(ssl.into());
                    HttpClient::with_connector(connector)
                }
                #[cfg(not(feature = "native-tls"))]
                panic!("You should either provide an HTTP Client or compile this crate with native-tls feature");
            }
        };

        Ok(SdkmsClient {
            client,
            api_endpoint: self
                .api_endpoint
                .unwrap_or_else(|| DEFAULT_API_ENDPOINT.to_owned()),
            auth: self.auth,
            last_used: AtomicU64::new(0),
            auth_response: None,
        })
    }
}

/// A client session with SDKMS.
///
/// REST APIs are exposed as methods on this type. Communication with SDKMS API endpoint is protected with TLS and this
/// type uses [`simple_hyper_client::blocking::Client`] along with [`tokio_native_tls::TlsConnector`] for HTTP/TLS.
///
/// When making crypto API calls using an API key, it is possible to pass the API key as an HTTP Basic Authorization
/// header along with each request. This can be achieved by setting the API key using
/// [`SdkmsClientBuilder::with_api_key()`]. Note that some features, e.g. transient keys, may not be available when
/// using this authentication method. To be able to use such features, you can establish a session using any of the
/// following methods:
/// - [`authenticate_with_api_key()`](#method.authenticate_with_api_key)
/// - [`authenticate_with_cert()`](#method.authenticate_with_cert)
/// - [`authenticate_app()`](#method.authenticate_app)
///
/// Note that certain non-cryptographic APIs require a user session, which can be established using
/// [`authenticate_user()`](#method.authenticate_user). This includes many APIs such as:
/// - [`create_group()`](#method.create_group)
/// - [`create_app()`](#method.create_app)
/// - ...
///
/// Also note that a user session is generally not permitted to call crypto APIs. In case your current authorization
/// is not appropriate for a particular API call, you'll get an error to that effect from SDKMS.
///
/// Certain APIs are "approvable", i.e. they can be subject to an approval policy. In such cases there are two methods
/// on [`SdkmsClient`], e.g. [`encrypt()`] / [`request_approval_to_encrypt()`]. Whether or not you need to call
/// [`request_approval_to_encrypt()`] depends on the approval policy that is applicable to the security object being
/// used in your request. You can find out if a particular request is subject to an approval policy by first calling
/// the regular API, e.g. [`encrypt()`] and checking if the response indicates that an approval request is needed at
/// which point you can call [`request_approval_to_encrypt()`]. There is an example of how to do this in
/// [the repository](https://github.com/fortanix/sdkms-client-rust/blob/master/examples/approval_request.rs).
///
/// [`simple_hyper_client::blocking::Client`]: TODO
/// [`tokio_native_tls::TlsConnector`]: https://docs.rs/tokio-native-tls/0.3.0/tokio_native_tls/struct.TlsConnector.html
/// [`SdkmsClientBuilder::with_api_key()`]: ./struct.SdkmsClientBuilder.html#method.with_api_key
/// [`SdkmsClient`]: ./struct.SdkmsClient.html
/// [`encrypt()`]: #method.encrypt
/// [`request_approval_to_encrypt()`]: #method.request_approval_to_encrypt
pub struct SdkmsClient {
    auth: Option<Auth>,
    api_endpoint: String,
    client: HttpClient,
    last_used: AtomicU64, // Time.0
    auth_response: Option<AuthResponse>,
}

impl SdkmsClient {
    pub fn builder() -> SdkmsClientBuilder {
        SdkmsClientBuilder {
            client: None,
            api_endpoint: None,
            auth: None,
        }
    }

    fn authenticate(&self, auth: Option<&Auth>) -> Result<Self> {
        let auth_response: AuthResponse = json_request_with_auth(
            &self.client,
            &self.api_endpoint,
            Method::POST,
            "/sys/v1/session/auth",
            auth,
            None::<&()>,
        )?;
        Ok(SdkmsClient {
            client: self.client.clone(),
            api_endpoint: self.api_endpoint.clone(),
            auth: Some(Auth::Bearer(auth_response.access_token.clone())),
            last_used: AtomicU64::new(now().0),
            auth_response: Some(auth_response),
        })
    }

    pub fn authenticate_with_api_key(&self, api_key: &str) -> Result<Self> {
        self.authenticate(Some(Auth::from_api_key(api_key)).as_ref())
    }

    pub fn authenticate_with_cert(&self, app_id: Option<&Uuid>) -> Result<Self> {
        self.authenticate(app_id.map(|id| Auth::from_user_pass(id, "")).as_ref())
    }

    pub fn authenticate_app(&self, app_id: &Uuid, app_secret: &str) -> Result<Self> {
        self.authenticate(Some(Auth::from_user_pass(app_id, app_secret)).as_ref())
    }

    pub fn authenticate_user(&self, email: &str, password: &str) -> Result<Self> {
        self.authenticate(Some(Auth::from_user_pass(email, password)).as_ref())
    }

    pub fn api_endpoint(&self) -> &str {
        &self.api_endpoint
    }

    pub fn auth_response(&self) -> Option<&AuthResponse> {
        self.auth_response.as_ref()
    }

    pub fn entity_id(&self) -> Option<Uuid> {
        self.auth_response().map(|ar| ar.entity_id)
    }

    pub fn has_session(&self) -> bool {
        match self.auth {
            Some(Auth::Bearer(_)) => true,
            _ => false,
        }
    }

    fn json_request<E, D>(&self, method: Method, uri: &str, req: Option<&E>) -> Result<D>
    where
        E: Serialize,
        D: for<'de> Deserialize<'de>,
    {
        let Self {
            ref client,
            ref api_endpoint,
            ref auth,
            ..
        } = *self;
        let result = json_request_with_auth(client, api_endpoint, method, uri, auth.as_ref(), req)?;
        self.last_used.store(now().0, Ordering::Relaxed);
        Ok(result)
    }
}

impl Drop for SdkmsClient {
    fn drop(&mut self) {
        let _ = self.terminate();
    }
}

impl SdkmsClient {
    pub fn terminate(&mut self) -> Result<()> {
        if let Some(Auth::Bearer(_)) = self.auth {
            self.json_request(Method::POST, "/sys/v1/session/terminate", None::<&()>)?;
            self.auth = None;
        }
        Ok(())
    }

    pub fn invoke_plugin_nice<I, O>(&self, id: &Uuid, req: &I) -> Result<O>
    where
        I: Serialize,
        O: for<'de> Deserialize<'de>,
    {
        let req = serde_json::to_value(req)?;
        let output = self.execute::<OperationInvokePlugin>(&req, (id,), None)?;
        Ok(serde_json::from_value(output)?)
    }

    pub fn execute<O: Operation>(
        &self,
        body: &O::Body,
        p: <O::PathParams as TupleRef>::Ref,
        q: Option<&O::QueryParams>,
    ) -> Result<O::Output> {
        self.json_request(O::method(), &O::path(p, q), O::to_body(body).as_ref())
    }

    pub fn request_approval<O: Operation>(
        &self,
        body: &O::Body,
        p: <O::PathParams as TupleRef>::Ref,
        q: Option<&O::QueryParams>,
        description: Option<String>,
    ) -> Result<PendingApproval<O>> {
        let request = self.create_approval_request(&ApprovalRequestRequest {
            operation: Some(O::path(p, q)),
            method: Some(format!("{}", O::method())),
            body: O::to_body(body),
            description,
        })?;
        Ok(PendingApproval::from_request_id(request.request_id))
    }

    pub fn expires_in(&self) -> Option<u64> {
        let expires_at = self.last_used.load(Ordering::Relaxed)
            + self.auth_response().map_or(0, |ar| ar.expires_in as u64);
        expires_at.checked_sub(now().0)
    }
}

pub struct PendingApproval<O: Operation>(Uuid, PhantomData<O>);

impl<O: Operation> fmt::Debug for PendingApproval<O> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, formatter)
    }
}

impl<O: Operation> PendingApproval<O> {
    pub fn from_request_id(request_id: Uuid) -> Self {
        PendingApproval(request_id, PhantomData)
    }

    pub fn request_id(&self) -> Uuid {
        self.0
    }

    pub fn get(&self, sdkms: &SdkmsClient) -> Result<ApprovalRequest> {
        sdkms.get_approval_request(&self.0)
    }

    pub fn status(&self, sdkms: &SdkmsClient) -> Result<ApprovalStatus> {
        Ok(self.get(sdkms)?.status)
    }

    pub fn result(&self, sdkms: &SdkmsClient) -> Result<Result<O::Output>> {
        let result = sdkms.get_approval_request_result(&self.0)?;
        Ok(if result.is_ok() {
            serde_json::from_value::<O::Output>(result.body).map_err(Error::EncoderError)
        } else {
            let msg: String = serde_json::from_value(result.body).map_err(Error::EncoderError)?;
            Err(Error::from_status(StatusCode::from_u16(result.status).unwrap(), msg))
        })
    }
}

impl<O: Operation> Clone for PendingApproval<O> {
    fn clone(&self) -> Self {
        PendingApproval(self.0, PhantomData)
    }
}

fn now() -> Time {
    Time(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Invalid system time")
            .as_secs(),
    )
}

fn json_decode_reader<R: Read, T: for<'de> Deserialize<'de>>(rdr: &mut R) -> serde_json::Result<T> {
    match serde_json::from_reader(rdr) {
        // When the body of the response is empty, attempt to deserialize null value instead
        Err(ref e) if e.is_eof() && e.line() == 1 && e.column() == 0 => {
            serde_json::from_value(serde_json::Value::Null)
        }
        v => v,
    }
}

fn json_request_with_auth<E, D>(
    client: &HttpClient,
    api_endpoint: &str,
    method: Method,
    path: &str,
    auth: Option<&Auth>,
    body: Option<&E>,
) -> Result<D>
where
    E: Serialize,
    D: for<'de> Deserialize<'de>,
{
    let url = format!("{}{}", api_endpoint, path);
    let mut req = client.request(method.clone(), &url)?;
    let mut headers = HeaderMap::new();
    if let Some(auth) = auth {
        headers.insert(AUTHORIZATION, auth.format_header());
    }
    if let Some(request_body) = body {
        headers.typed_insert(ContentType::json());
        let body = serde_json::to_string(request_body).map_err(Error::EncoderError)?;
        req = req.body(body);
    }
    req = req.headers(headers);
    match req.send() {
        Err(e) => {
            info!("Error {} {}", method, url);
            Err(Error::NetworkError(e))
        }
        Ok(ref mut res) if res.status().is_success() => {
            info!("{} {} {}", res.status().as_u16(), method, url);
            json_decode_reader(res.body_mut()).map_err(|err| Error::EncoderError(err))
        }
        Ok(ref mut res) => {
            info!("{} {} {}", res.status().as_u16(), method, url);
            let mut buffer = String::new();
            res.body_mut()
                .read_to_string(&mut buffer)
                .map_err(|err| Error::IoError(err))?;
            Err(Error::from_status(res.status(), buffer))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    #[test]
    fn client_is_send_and_sync() {
        assert_send::<SdkmsClient>();
        assert_sync::<SdkmsClient>();

        assert_send::<SdkmsClientBuilder>();
        assert_sync::<SdkmsClientBuilder>();
    }
}
