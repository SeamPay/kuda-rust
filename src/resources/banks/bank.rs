use serde_derive::{Deserialize, Serialize};

use crate::config::{Client, Response};
use crate::params::Response as R;

pub struct Bank {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBankListResponseData {
    pub banks: Vec<GetBankListBankResponseBank>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBankListBankResponseBank {
    pub bank_code: String,
    pub bank_name: String,
}

impl Bank {
    // Get Bank List.
    pub fn get_bank_list(client: &Client) -> Response<R<GetBankListResponseData>> {
        client.post_body("/v1", "BANK_LIST", None::<Box<String>>)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    const PUBLIC_KEY: &str = "CHANGE_ME";

    const PRIVATE_KEY: &str = "CHANGE_ME";

    #[tokio::test]
    async fn test_get_bank_list() {
        let client = Client::new(
            "https://kuda-openapi-uat.kudabank.com".to_string(),
            PRIVATE_KEY.to_string(),
            PUBLIC_KEY.to_string(),
            "CHANGE_ME".to_string(),
        );

        let response: Result<R<GetBankListResponseData>, Error> =
            Bank::get_bank_list(&client).await;
        let bank = match response {
            Err(err) => panic!("{:?}", err),
            Ok(ok) => {
                println!("{:?}", ok);
                ok
            }
        };

        assert_eq!(bank.status, false);
        assert_eq!(bank.message, "Completed Successfully");
    }
}
