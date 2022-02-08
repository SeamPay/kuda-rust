use std::pin::Pin;

use futures::future;
use futures::future::Future;
use http::header::{HeaderMap, HeaderName, HeaderValue};
use http::request::Builder as RequestBuilder;
use hyper_tls::HttpsConnector;
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

use crate::encryption::*;
use crate::error::{Error, RequestError};
use crate::params::{Headers, Request};

type HttpClient = hyper::Client<HttpsConnector<hyper::client::HttpConnector>, hyper::Body>;
pub type Response<T> = Pin<Box<dyn Future<Output = Result<T, Error>> + Send>>;

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn ok<T: Send + 'static>(ok: T) -> Response<T> {
    Box::pin(future::ready(Ok(ok)))
}

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn err<T: Send + 'static>(err: Error) -> Response<T> {
    Box::pin(future::ready(Err(err)))
}

#[derive(Clone)]
pub struct Client {
    host: String,
    client: HttpClient,
    private_key: String,
    public_key: String,
    client_key: String,
    headers: Headers,
}

impl Client {
    /// Creates a new client
    pub fn new(
        scheme_host: String,
        private_key: String,
        public_key: String,
        client_key: String,
    ) -> Client {
        Client::from_url(scheme_host, private_key, public_key, client_key)
    }

    /// Creates a new client posted to a custom `scheme://host/`
    pub fn from_url(
        scheme_host: impl Into<String>,
        private_key: impl Into<String>,
        public_key: impl Into<String>,
        client_key: impl Into<String>,
    ) -> Client {
        let host = scheme_host.into();
        let private_key = private_key.into();
        let public_key = public_key.into();
        let client_key = client_key.into();
        let https = HttpsConnector::new();
        let client = hyper::Client::builder().build(https);
        let headers = Headers::default();

        Client {
            host,
            client,
            private_key,
            public_key,
            client_key,
            headers,
        }
    }

    /// Make a `POST` http request with body
    pub fn post_body<T, B>(&self, path: &str, service_type: &str, body: B) -> Response<T>
    where
        T: DeserializeOwned + Send + 'static,
        B: serde::Serialize,
    {
        let url = self.url(path);
        let req = Request {
            service_type: service_type.to_string(),
            request_ref: self.request_ref(),
            data: Some(body),
        };
        let password = self.password();

        let payload = serde_json::to_string(&req).unwrap();

        let dk = make_derived_key(&password);
        let iv = make_iv(&password);

        let encrypted_payload = encrypt_aes_256_cbc(payload.as_bytes(), &dk, &iv).unwrap();
        let encrypted_password = rsa_encrypt(&password, &self.public_key);

        let mut request_payload: HashMap<String, String> = HashMap::new();
        request_payload.insert("data".to_string(), base64::encode(encrypted_payload));

        let mut req = RequestBuilder::new()
            .method("POST")
            .uri(url)
            .body(match serde_json::to_string(&request_payload) {
                Err(err) => return Box::pin(future::ready(Err(Error::serialize(err)))),
                Ok(body) => hyper::Body::from(body),
            })
            .unwrap();
        *req.headers_mut() = self.headers();
        req.headers_mut().insert(
            HeaderName::from_static("content-type"),
            HeaderValue::from_str("application/json").unwrap(),
        );
        req.headers_mut().insert(
            HeaderName::from_static("password"),
            HeaderValue::from_str(&encrypted_password).unwrap(),
        );

        send(&self.client, self.private_key.clone(), req)
    }

    fn url(&self, path: &str) -> String {
        format!("{}/{}", self.host, path.trim_start_matches('/'))
    }

    fn headers(&self) -> HeaderMap {
        HeaderMap::new()
    }

    fn request_ref(&self) -> String {
        let g = libxid::new_generator();
        let id = g.new_id().unwrap();

        let string_list = vec!["SP".to_string(), id.to_string().to_uppercase()];
        string_list.join("-")
    }

    fn password(&self) -> String {
        let random: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        let string_list = vec![self.client_key.clone(), random.to_uppercase()];
        string_list.join("-")
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KudaResponse {
    pub data: String,
    pub password: String,
}

fn send<T>(
    client: &HttpClient,
    private_key: String,
    request: hyper::Request<hyper::Body>,
) -> Response<T>
where
    T: DeserializeOwned + Send + 'static,
{
    let client = client.clone(); // N.B. Client is send sync;  cloned clients share the same pool.
    Box::pin(async move {
        let response = client.request(request).await?;
        let status = response.status();
        let bytes = hyper::body::to_bytes(response.into_body()).await?;
        if !status.is_success() {
            let mut err = serde_json::from_slice(&bytes).unwrap_or_else(|err| RequestError {
                message: format!("failed to deserialize error: {}", err),
                ..Default::default()
            });
            err.http_status = status.as_u16();
            return Err(Error::from(err));
        }

        let encrypted_response: KudaResponse = serde_json::from_slice(&bytes)
            .map_err(Error::deserialize)
            .unwrap();
        let encrypted_password: String = encrypted_response.password;
        let encrypted_data: String = encrypted_response.data;

        let decrypted_password = rsa_decrypt(encrypted_password, &private_key.to_string());

        let dk = make_derived_key(decrypted_password.trim_matches(char::from(0)));
        let iv = make_iv(decrypted_password.trim_matches(char::from(0)));

        let decrypted_data =
            decrypt_aes_256_cbc(&base64::decode(&encrypted_data).unwrap(), &dk, &iv).unwrap();

        serde_json::from_str(
            String::from_utf8(decrypted_data)
                .unwrap()
                .trim_matches(char::from(0)),
        )
        .map_err(Error::deserialize)
    })
}
