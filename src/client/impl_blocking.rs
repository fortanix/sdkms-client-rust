use crate::api_model::*;
use crate::operations::*;

use super::SdkmsClient;
use super::common::*;
use headers::{ContentType, HeaderMap, HeaderMapExt};
use simple_hyper_client::hyper::header::AUTHORIZATION;
use simple_hyper_client::blocking::Client as HttpClient;
use serde::{Deserialize, Serialize};
use simple_hyper_client::{Method};
use std::sync::atomic::{AtomicU64, Ordering};
use uuid::Uuid;
use std::io::Read;


impl SdkmsClient {
    fn authenticate_client(&self, auth: Option<&Auth>) -> Result<Self> {
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
        self.authenticate_client(Some(Auth::from_api_key(api_key)).as_ref())
    }
    
    pub fn authenticate_with_cert(&self, app_id: Option<&Uuid>) -> Result<Self> {
        self.authenticate_client(app_id.map(|id| Auth::from_user_pass(id, "")).as_ref())
    }
    
    pub fn authenticate_app(&self, app_id: &Uuid, app_secret: &str) -> Result<Self> {
        self.authenticate_client(Some(Auth::from_user_pass(app_id, app_secret)).as_ref())
    }
    
    pub fn authenticate_user(&self, email: &str, password: &str) -> Result<Self> {
        self.authenticate_client(Some(Auth::from_user_pass(email, password)).as_ref())
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
    
    pub fn terminate_client(&mut self) -> Result<()> {
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
    // dbg!(&headers);
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