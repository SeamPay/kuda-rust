use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Default)]
pub struct Headers {}

/// An request wrapper.
#[derive(Default, Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Request<T> {
    /// This is the name of the operation.
    pub service_type: String,
    /// This is a unique reference per request.
    pub request_ref: String,
    /// Data container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

/// An response wrapper.
#[derive(Default, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Response<T> {
    #[serde(rename = "RequestReference")]
    pub request_reference: Option<String>,
    #[serde(rename = "TransactionReference")]
    pub transaction_reference: Option<String>,
    #[serde(rename = "ResponseCode")]
    pub response_code: Option<String>,
    /// Response status.
    #[serde(rename = "Status")]
    pub status: bool,
    /// Response message.
    #[serde(rename = "Message")]
    pub message: String,
    /// Data container.
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}
