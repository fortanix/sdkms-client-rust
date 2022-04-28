use crate::api_model::*;
use crate::operations::*;
use super::common::*;

use super::SdkmsClient;
use std::fmt;
use std::io::Read;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use super::common::*;
use headers::HeaderMapExt;
use headers::{ContentType, HeaderMap};
use simple_hyper_client::to_bytes;
use uuid::Uuid;
use simple_hyper_client::Client as HttpClient;
use simple_hyper_client::{Bytes, Method, StatusCode};
use simple_hyper_client::hyper::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};

impl SdkmsClient {    
    // pub async fn terminate(&mut self) -> Result<()> {
    //     if let Some(Auth::Bearer(_)) = self.auth {
    //         self.json_request(Method::POST, "/sys/v1/session/terminate", None::<&()>).await?;
    //         self.auth = None;
    //     }
    //     Ok(())
    // }

    pub async fn invoke_plugin_nice<I, O>(&self, id: &Uuid, req: &I) -> Result<O>
    where
        I: Serialize,
        O: for<'de> Deserialize<'de>,
    {
        let req = serde_json::to_value(req)?;
        let output = self.execute::<OperationInvokePlugin>(&req, (id,), None).await?;
        Ok(serde_json::from_value(output)?)
    }

    pub async fn execute<O: Operation>(
        &self,
        body: &O::Body,
        p: <O::PathParams as TupleRef<'_>>::Ref,
        q: Option<&O::QueryParams>,
    ) -> Result<O::Output> {
        self.json_request(O::method(), &O::path(p, q), O::to_body(body).as_ref()).await
    }

    pub async fn request_approval<O: Operation>(
        &self,
        body: &O::Body,
        p: <O::PathParams as TupleRef<'_>>::Ref,
        q: Option<&O::QueryParams>,
        description: Option<String>,
    ) -> Result<PendingApproval<O>> {
        let request = self.create_approval_request(&ApprovalRequestRequest {
            operation: Some(O::path(p, q)),
            method: Some(format!("{}", O::method())),
            body: O::to_body(body),
            description,
        }).await?;
        Ok(PendingApproval::from_request_id(request.request_id))
    }
    
    async fn old_auth(&self, auth: Option<&Auth>) -> Result<Self> {
        let auth_response: AuthResponse = json_request_with_auth(
            &self.client,
            &self.api_endpoint,
            Method::POST,
            "/sys/v1/session/auth",
            auth,
            None::<&()>,
        ).await?;
        Ok(SdkmsClient {
            client: self.client.clone(),
            api_endpoint: self.api_endpoint.clone(),
            auth: Some(Auth::Bearer(auth_response.access_token.clone())),
            last_used: AtomicU64::new(now().0),
            auth_response: Some(auth_response),
        })
    }

    pub async fn authenticate_with_api_key(&self, api_key: &str) -> Result<Self> {
        self.old_auth(Some(Auth::from_api_key(api_key)).as_ref()).await
    }

    pub async fn authenticate_with_cert(&self, app_id: Option<&Uuid>) -> Result<Self> {
        self.old_auth(app_id.map(|id| Auth::from_user_pass(id, "")).as_ref()).await
    }

    pub async fn authenticate_app(&self, app_id: &Uuid, app_secret: &str) -> Result<Self> {
        self.old_auth(Some(Auth::from_user_pass(app_id, app_secret)).as_ref()).await
    }

    pub async fn authenticate_user(&self, email: &str, password: &str) -> Result<Self> {
        self.old_auth(Some(Auth::from_user_pass(email, password)).as_ref()).await
    }

    async fn json_request<E, D>(&self, method: Method, uri: &str, req: Option<&E>) -> Result<D>
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
        let result = json_request_with_auth(client, api_endpoint, method, uri, auth.as_ref(), req).await?;
        self.last_used.store(now().0, Ordering::Relaxed);
        Ok(result)
    }
}

async fn json_request_with_auth<E, D>(
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
    match req.send().await {
        Err(e) => {
            info!("Error {} {}", method, url);
            Err(Error::NetworkError(e))
        }
        Ok(ref mut res) if res.status().is_success() => {
            info!("{} {} {}", res.status().as_u16(), method, url);
            //TODO Remove Unwrap
            json_decode_bytes(&mut to_bytes(res.body_mut()).await.expect("Could not convert body to bytes")).map_err(|err| Error::EncoderError(err))
        }
        Ok(ref mut res) => {
            info!("{} {} {}", res.status().as_u16(), method, url);
            let buffer = String::from_utf8(to_bytes(res.body_mut()).await.expect("Could not convert to Bytes").to_vec()).unwrap();
            Err(Error::from_status(res.status(), buffer))
        }
    }
}
