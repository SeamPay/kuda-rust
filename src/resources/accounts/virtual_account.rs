use serde_derive::{Deserialize, Serialize};
use std::str;

use crate::config::{Client, Response};
use crate::params::Response as R;

pub struct VirtualAccount {}

/// The parameters for `VirtualAccount::request::create`.
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualAccountCreateRequest {
    /// User's email address.
    pub email: String,
    /// User's phone number.
    pub phone_number: String,
    /// User's last name.
    pub last_name: String,
    /// User's first name.
    pub first_name: String,
    /// Unique identifier for the account.
    pub tracking_reference: String,
}

impl VirtualAccountCreateRequest {
    pub fn new() -> Self {
        VirtualAccountCreateRequest {
            email: Default::default(),
            phone_number: Default::default(),
            last_name: Default::default(),
            first_name: Default::default(),
            tracking_reference: Default::default(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VirtualAccountCreateResponseData {
    #[serde(rename = "AccountNumber")]
    pub account_number: String,
}

/// The parameters for `VirtualAccount::request::get`.
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualAccountGetRequest {
    /// Unique identifier for the account.
    pub tracking_reference: String,
}

impl VirtualAccountGetRequest {
    pub fn new() -> Self {
        VirtualAccountGetRequest {
            tracking_reference: Default::default(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualAccountGetResponseData {
    #[serde(rename = "Account")]
    pub account: VirtualAccountGetResponseAccount,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualAccountGetResponseAccount {
    #[serde(rename = "AccountNumber")]
    pub account_number: String,
    #[serde(rename = "Email")]
    pub email: String,
    #[serde(rename = "PhoneNumber")]
    pub phone_number: String,
    #[serde(rename = "LastName")]
    pub last_name: String,
    #[serde(rename = "FirstName")]
    pub first_name: String,
    #[serde(rename = "AccountName")]
    pub account_name: String,
    #[serde(rename = "TrackingReference")]
    pub tracking_reference: String,
    #[serde(rename = "CreationDate")]
    pub creation_date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualAccountGetBalanceResponseData {
    #[serde(rename = "LedgerBalance")]
    pub ledger_balance: f64,
    #[serde(rename = "AvailableBalance")]
    pub available_balance: f64,
    #[serde(rename = "WithdrawableBalance")]
    pub withdrawable_balance: f64,
}

/// The parameters for `VirtualAccount::request::fund/withdraw`.
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualAccountFundWithdrawRequest {
    /// Unique identifier for the account.
    #[serde(rename = "TrackingReference")]
    pub tracking_reference: String,
    /// Amount to be funded/withdrawn. All amounts in kobo.
    pub amount: String,
    /// Transaction narration.
    pub narration: String,
}

impl VirtualAccountFundWithdrawRequest {
    pub fn new() -> Self {
        VirtualAccountFundWithdrawRequest {
            tracking_reference: Default::default(),
            amount: Default::default(),
            narration: Default::default(),
        }
    }
}

/// The parameters for `VirtualAccount::request::fund_transfer`.
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualAccountFundTransferRequest {
    /// Unique identifier of the sender.
    pub tracking_reference: String,
    /// Account number of the recipient.
    #[serde(rename = "beneficiaryAccount")]
    pub beneficiary_account: String,
    /// Amount to be transferred. All amounts in kobo.
    pub amount: String,
    /// Description of the transaction.
    pub narration: String,
    /// Bank code for the destination bank.
    #[serde(rename = "beneficiarybankCode")]
    pub beneficiary_bank_code: String,
    /// Name of the person receiving money.
    #[serde(rename = "beneficiaryName")]
    pub beneficiary_name: String,
    /// Name of the person sending money.
    #[serde(rename = "senderName")]
    pub sender_name: String,
    /// Session ID generated from the nameEnquiry request.
    #[serde(rename = "nameEnquiryId")]
    pub name_enquiry_id: String,
}

impl VirtualAccountFundTransferRequest {
    pub fn new() -> Self {
        VirtualAccountFundTransferRequest {
            tracking_reference: Default::default(),
            beneficiary_account: Default::default(),
            amount: Default::default(),
            narration: Default::default(),
            beneficiary_bank_code: Default::default(),
            beneficiary_name: Default::default(),
            sender_name: Default::default(),
            name_enquiry_id: Default::default(),
        }
    }
}

/// The parameters for `VirtualAccount::request::transaction_history`.
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualAccountTransactionHistoryRequest {
    /// Unique identifier for the account.
    pub tracking_reference: String,
    pub page_size: i64,
    pub page_number: i64,
}

impl VirtualAccountTransactionHistoryRequest {
    pub fn new() -> Self {
        VirtualAccountTransactionHistoryRequest {
            tracking_reference: Default::default(),
            page_size: Default::default(),
            page_number: Default::default(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualAccountTransactionHistoryResponseData {
    #[serde(rename = "PostingsHistory")]
    pub postings_history: Vec<VirtualAccountTransactionHistoryPostingsHistory>,
    #[serde(rename = "Message")]
    pub message: ::serde_json::Value,
    #[serde(rename = "StatusCode")]
    pub status_code: String,
    #[serde(rename = "TotalRecordInStore")]
    pub total_record_in_store: i64,
    #[serde(rename = "TotalDebit")]
    pub total_debit: f64,
    #[serde(rename = "TotalCredit")]
    pub total_credit: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualAccountTransactionHistoryPostingsHistory {
    #[serde(rename = "ReferenceNumber")]
    pub reference_number: String,
    #[serde(rename = "ReversalReferenceNumber")]
    pub reversal_reference_number: ::serde_json::Value,
    #[serde(rename = "AccountNumber")]
    pub account_number: String,
    #[serde(rename = "LinkedAccountNumber")]
    pub linked_account_number: ::serde_json::Value,
    #[serde(rename = "RealDate")]
    pub real_date: String,
    #[serde(rename = "Amount")]
    pub amount: f64,
    #[serde(rename = "OpeningBalance")]
    pub opening_balance: f64,
    #[serde(rename = "BalanceAfter")]
    pub balance_after: f64,
    #[serde(rename = "Narration")]
    pub narration: String,
    #[serde(rename = "InstrumentNumber")]
    pub instrument_number: String,
    #[serde(rename = "PostingRecordType")]
    pub posting_record_type: i64,
    #[serde(rename = "PostedBy")]
    pub posted_by: String,
}

/// The parameters for `VirtualAccount::request::filtered_transaction_history`.
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualAccountFilteredTransactionHistoryRequest {
    /// Unique identifier for the account.
    pub tracking_reference: String,
    pub start_date: String,
    pub end_date: String,
    pub page_size: i64,
    pub page_number: i64,
}

impl VirtualAccountFilteredTransactionHistoryRequest {
    pub fn new() -> Self {
        VirtualAccountFilteredTransactionHistoryRequest {
            tracking_reference: Default::default(),
            start_date: Default::default(),
            end_date: Default::default(),
            page_size: Default::default(),
            page_number: Default::default(),
        }
    }
}

impl VirtualAccount {
    /// Create Virtual Account.
    pub fn create_virtual_account(
        client: &Client,
        params: VirtualAccountCreateRequest,
    ) -> Response<R<VirtualAccountCreateResponseData>> {
        client.post_body("/v1", "ADMIN_CREATE_VIRTUAL_ACCOUNT", Some(params))
    }

    /// Get Virtual Account.
    pub fn get_virtual_account(
        client: &Client,
        params: VirtualAccountGetRequest,
    ) -> Response<R<VirtualAccountGetResponseData>> {
        client.post_body("/v1", "ADMIN_RETRIEVE_SINGLE_VIRTUAL_ACCOUNT", Some(params))
    }

    /// Get Virtual Account Balance.
    pub fn get_virtual_account_balance(
        client: &Client,
        params: VirtualAccountGetRequest,
    ) -> Response<R<VirtualAccountGetBalanceResponseData>> {
        client.post_body("/v1", "RETRIEVE_VIRTUAL_ACCOUNT_BALANCE", Some(params))
    }

    /// Retrieve a list of all transactions for a specified virtual account.
    pub fn get_virtual_account_transaction_history(
        client: &Client,
        params: VirtualAccountTransactionHistoryRequest,
    ) -> Response<R<VirtualAccountTransactionHistoryResponseData>> {
        client.post_body("/v1", "ADMIN_VIRTUAL_ACCOUNT_TRANSACTIONS", params)
    }

    /// Retrieve a list of filtered transactions for a specified virtual account.
    pub fn get_virtual_account_filtered_transaction_history(
        client: &Client,
        params: VirtualAccountFilteredTransactionHistoryRequest,
    ) -> Response<R<VirtualAccountTransactionHistoryResponseData>> {
        client.post_body("/v1", "ADMIN_VIRTUAL_ACCOUNT_FILTERED_TRANSACTIONS", params)
    }

    /// Fund Virtual Account.
    pub fn fund_virtual_account(
        client: &Client,
        params: VirtualAccountFundWithdrawRequest,
    ) -> Response<R<serde_json::Value>> {
        client.post_body("/v1", "FUND_VIRTUAL_ACCOUNT", Some(params))
    }

    /// Withdraw Virtual Account.
    pub fn withdraw_virtual_account(
        client: &Client,
        params: VirtualAccountFundWithdrawRequest,
    ) -> Response<R<serde_json::Value>> {
        client.post_body("/v1", "WITHDRAW_VIRTUAL_ACCOUNT", Some(params))
    }

    /// Fund Transfer Virtual Account.
    pub fn fund_transfer_virtual_account(
        client: &Client,
        params: VirtualAccountFundTransferRequest,
    ) -> Response<R<serde_json::Value>> {
        client.post_body("/v1", "VIRTUAL_ACCOUNT_FUND_TRANSFER", Some(params))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;
    use rand::Rng;

    const PUBLIC_KEY: &str = "CHANGE_ME";

    const PRIVATE_KEY: &str = "CHANGE_ME";

    #[tokio::test]
    async fn test_create_virtual_account() {
        let client = Client::new(
            "https://kuda-openapi-uat.kudabank.com".to_string(),
            PRIVATE_KEY.to_string(),
            PUBLIC_KEY.to_string(),
            "CHANGE_ME".to_string(),
        );

        let mut rng = rand::thread_rng();

        let mut params = VirtualAccountCreateRequest::new();
        params.first_name = "Mark".to_string();
        params.last_name = "Smith".to_string();
        params.phone_number = "09039658058".to_string();
        params.tracking_reference = generat;
        params.email = "example@email.com".to_string();

        let response: Result<R<VirtualAccountCreateResponseData>, Error> =
            VirtualAccount::create_virtual_account(&client, params).await;
        let account = match response {
            Err(err) => panic!("{:?}", err),
            Ok(ok) => {
                println!("{:?}", ok);
                ok
            }
        };

        assert_eq!(account.status, true);
    }

    #[tokio::test]
    async fn test_get_virtual_account() {
        let client = Client::new(
            "https://kuda-openapi-uat.kudabank.com".to_string(),
            PRIVATE_KEY.to_string(),
            PUBLIC_KEY.to_string(),
            "CHANGE_ME".to_string(),
        );

        let mut params = VirtualAccountGetRequest::new();
        params.tracking_reference = "0012".to_string();

        let response: Result<R<VirtualAccountGetResponseData>, Error> =
            VirtualAccount::get_virtual_account(&client, params).await;
        let account = match response {
            Err(err) => panic!("{:?}", err),
            Ok(ok) => {
                println!("{:?}", ok);
                ok
            }
        };

        assert_eq!(account.status, true);
        assert_eq!(account.message, "Request successful.");
    }

    #[tokio::test]
    async fn test_get_virtual_account_balance() {
        let client = Client::new(
            "https://kuda-openapi-uat.kudabank.com".to_string(),
            PRIVATE_KEY.to_string(),
            PUBLIC_KEY.to_string(),
            "CHANGE_ME".to_string(),
        );

        let mut params = VirtualAccountGetRequest::new();
        params.tracking_reference = "0012".to_string();

        let response: Result<R<VirtualAccountGetBalanceResponseData>, Error> =
            VirtualAccount::get_virtual_account_balance(&client, params).await;
        let account = match response {
            Err(err) => panic!("{:?}", err),
            Ok(ok) => {
                println!("{:?}", ok);
                ok
            }
        };

        assert_eq!(account.status, true);
        assert_eq!(account.message, "Operation successful");
    }

    #[tokio::test]
    async fn test_get_virtual_account_transaction_history() {
        let client = Client::new(
            "https://kuda-openapi-uat.kudabank.com".to_string(),
            PRIVATE_KEY.to_string(),
            PUBLIC_KEY.to_string(),
            "CHANGE_ME".to_string(),
        );

        let mut params = VirtualAccountTransactionHistoryRequest::new();
        params.tracking_reference = "0012".to_string();
        params.page_number = 1;
        params.page_size = 100;

        let response: Result<R<VirtualAccountTransactionHistoryResponseData>, Error> =
            VirtualAccount::get_virtual_account_transaction_history(&client, params).await;
        let account = match response {
            Err(err) => panic!("{:?}", err),
            Ok(ok) => {
                println!("{:?}", ok);
                ok
            }
        };

        assert_eq!(account.status, true);
        assert_eq!(account.message, "Request successful.");
    }

    #[tokio::test]
    async fn test_get_virtual_account_filtered_transaction_history() {
        let client = Client::new(
            "https://kuda-openapi-uat.kudabank.com".to_string(),
            PRIVATE_KEY.to_string(),
            PUBLIC_KEY.to_string(),
            "CHANGE_ME".to_string(),
        );

        let mut params = VirtualAccountFilteredTransactionHistoryRequest::new();
        params.tracking_reference = "0012".to_string();
        params.start_date = "2021-05-24T00:00:00".to_string();
        params.end_date = "2021-05-24T23:59:59".to_string();
        params.page_number = 1;
        params.page_size = 100;

        let response: Result<R<VirtualAccountTransactionHistoryResponseData>, Error> =
            VirtualAccount::get_virtual_account_filtered_transaction_history(&client, params).await;
        let account = match response {
            Err(err) => panic!("{:?}", err),
            Ok(ok) => {
                println!("{:?}", ok);
                ok
            }
        };

        assert_eq!(account.status, true);
        assert_eq!(account.message, "Request successful.");
    }

    #[tokio::test]
    async fn test_fund_virtual_account() {
        let client = Client::new(
            "https://kuda-openapi-uat.kudabank.com".to_string(),
            PRIVATE_KEY.to_string(),
            PUBLIC_KEY.to_string(),
            "CHANGE_ME".to_string(),
        );

        let mut params = VirtualAccountFundWithdrawRequest::new();
        params.tracking_reference = "6b381fdf-933c-4c33-8fcf-6a6c3372fbc4".to_string();
        params.amount = "1777860".to_string();
        params.narration = "REVERT".to_string();

        let response: Result<R<serde_json::Value>, Error> =
            VirtualAccount::fund_virtual_account(&client, params).await;
        let account = match response {
            Err(err) => panic!("{:?}", err),
            Ok(ok) => {
                println!("{:?}", ok);
                ok
            }
        };

        assert_eq!(account.status, true);
        assert_eq!(account.message, "Transaction successful");
    }

    #[tokio::test]
    async fn test_withdraw_virtual_account() {
        let client = Client::new(
            "https://kuda-openapi-uat.kudabank.com".to_string(),
            PRIVATE_KEY.to_string(),
            PUBLIC_KEY.to_string(),
            "CHANGE_ME".to_string(),
        );

        let mut params = VirtualAccountFundWithdrawRequest::new();
        params.tracking_reference = "0012".to_string();
        params.amount = "1000".to_string();
        params.narration = "test withdraw".to_string();

        let response: Result<R<serde_json::Value>, Error> =
            VirtualAccount::withdraw_virtual_account(&client, params).await;
        let account = match response {
            Err(err) => panic!("{:?}", err),
            Ok(ok) => {
                println!("{:?}", ok);
                ok
            }
        };

        assert_eq!(account.status, true);
        assert_eq!(account.message, "Transaction successful");
    }

    #[tokio::test]
    async fn test_fund_transfer_virtual_account() {
        let client = Client::new(
            "https://kuda-openapi-uat.kudabank.com".to_string(),
            PRIVATE_KEY.to_string(),
            PUBLIC_KEY.to_string(),
            "CHANGE_ME".to_string(),
        );

        let mut params = VirtualAccountFundTransferRequest::new();
        params.tracking_reference = "0012".to_string();
        params.beneficiary_account = "0012".to_string();
        params.amount = "1000".to_string();
        params.narration = "test fund transfer".to_string();
        params.beneficiary_bank_code = "0012".to_string();
        params.beneficiary_name = "test beneficiary".to_string();
        params.sender_name = "test sender".to_string();
        params.name_enquiry_id = "0".to_string();

        let response: Result<R<serde_json::Value>, Error> =
            VirtualAccount::fund_transfer_virtual_account(&client, params).await;
        let account = match response {
            Err(err) => panic!("{:?}", err),
            Ok(ok) => {
                println!("{:?}", ok);
                ok
            }
        };

        assert_eq!(account.status, true);
        assert_eq!(account.message, "Transaction successful.");
    }
}
