use serde_derive::{Deserialize, Serialize};
use std::str;

use crate::config::{Client, Response};
use crate::params::Response as R;

pub struct AdminAccount {}

/// The parameters for `AdminAccount::request::balance`.
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminAccountBalanceRequest {
    /// Unique identifier for the account.
    pub tracking_reference: String,
}

impl AdminAccountBalanceRequest {
    pub fn new() -> Self {
        AdminAccountBalanceRequest {
            tracking_reference: Default::default(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminAccountGetBalanceResponseData {
    #[serde(rename = "LedgerBalance")]
    pub ledger_balance: f64,
    #[serde(rename = "AvailableBalance")]
    pub available_balance: f64,
    #[serde(rename = "WithdrawableBalance")]
    pub withdrawable_balance: f64,
}

/// The parameters for `Account::request::enquiry`.
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountEnquiryRequest {
    /// Destination bank account number.
    #[serde(rename = "beneficiaryAccountNumber")]
    pub beneficiary_account_number: String,
    /// Destination bank code.
    #[serde(rename = "beneficiaryBankCode")]
    pub beneficiary_bank_code: String,
}

impl AccountEnquiryRequest {
    pub fn new() -> Self {
        AccountEnquiryRequest {
            beneficiary_account_number: Default::default(),
            beneficiary_bank_code: Default::default(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountEnquiryResponseData {
    #[serde(rename = "BeneficiaryAccountNumber")]
    pub beneficiary_account_number: String,
    #[serde(rename = "BeneficiaryName")]
    pub beneficiary_name: String,
    #[serde(rename = "SenderAccountNumber")]
    pub sender_account_number: String,
    #[serde(rename = "SenderName")]
    pub sender_name: Option<String>,
    #[serde(rename = "BeneficiaryCustomerID")]
    pub beneficiary_customer_id: i64,
    #[serde(rename = "BeneficiaryBankCode")]
    pub beneficiary_bank_code: String,
    #[serde(rename = "NameEnquiryID")]
    pub name_enquiry_id: i64,
    #[serde(rename = "ResponseCode")]
    pub response_code: String,
    #[serde(rename = "TransferCharge")]
    pub transfer_charge: f64,
    #[serde(rename = "SessionID")]
    pub session_id: String,
}

/// The parameters for `AdminAccount::request::fund_transfer`.
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminAccountFundTransferRequest {
    /// Destination bank account number.
    #[serde(rename = "beneficiaryAccount")]
    pub beneficiary_account: String,
    /// Destination bank code.
    #[serde(rename = "beneficiarybankCode")]
    pub beneficiary_bank_code: String,
    /// Name of the recipient.
    #[serde(rename = "beneficiaryName")]
    pub beneficiary_name: String,
    /// Amount to be transferred. All amounts in kobo.
    pub amount: String,
    /// Transaction narration.
    pub narration: String,
    /// Session ID generated from the nameEnquiry request.
    #[serde(rename = "nameEnquirySessionID")]
    pub name_enquiry_session_id: String,
    pub tracking_reference: String,
    /// Name of the person sending money.
    #[serde(rename = "senderName")]
    pub sender_name: String,
}

impl AdminAccountFundTransferRequest {
    pub fn new() -> Self {
        AdminAccountFundTransferRequest {
            beneficiary_account: Default::default(),
            beneficiary_bank_code: Default::default(),
            beneficiary_name: Default::default(),
            amount: Default::default(),
            narration: Default::default(),
            name_enquiry_session_id: Default::default(),
            tracking_reference: Default::default(),
            sender_name: Default::default(),
        }
    }
}

/// The parameters for `AdminAccount::request::transaction_history`.
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminAccountTransactionHistoryRequest {
    /// Unique identifier for the account.
    pub tracking_reference: String,
    pub page_size: i64,
    pub page_number: i64,
}

impl AdminAccountTransactionHistoryRequest {
    pub fn new() -> Self {
        AdminAccountTransactionHistoryRequest {
            tracking_reference: Default::default(),
            page_size: Default::default(),
            page_number: Default::default(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminAccountTransactionHistoryResponseData {
    #[serde(rename = "PostingsHistory")]
    pub postings_history: Vec<AdminAccountTransactionHistoryPostingsHistory>,
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
pub struct AdminAccountTransactionHistoryPostingsHistory {
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

/// The parameters for `AdminAccount::request::filtered_transaction_history`.
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminAccountFilteredTransactionHistoryRequest {
    /// Unique identifier for the account.
    pub tracking_reference: String,
    pub start_date: String,
    pub end_date: String,
    pub page_size: i64,
    pub page_number: i64,
}

impl AdminAccountFilteredTransactionHistoryRequest {
    pub fn new() -> Self {
        AdminAccountFilteredTransactionHistoryRequest {
            tracking_reference: Default::default(),
            start_date: Default::default(),
            end_date: Default::default(),
            page_size: Default::default(),
            page_number: Default::default(),
        }
    }
}

/// The parameters for `AdminAccount::request::check_transfer_status`.
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminAccountCheckTransferRequest {
    /// Unique identifier for the account.
    pub is_third_party_bank_transfer: bool,
    pub transaction_request_reference: String,
}

impl AdminAccountCheckTransferRequest {
    pub fn new() -> Self {
        AdminAccountCheckTransferRequest {
            is_third_party_bank_transfer: Default::default(),
            transaction_request_reference: Default::default(),
        }
    }
}

impl AdminAccount {
    /// Get Admin Account Balance.
    pub fn get_admin_account_balance(
        client: &Client,
        params: AdminAccountBalanceRequest,
    ) -> Response<R<AdminAccountGetBalanceResponseData>> {
        client.post_body("/v1", "ADMIN_RETRIEVE_MAIN_ACCOUNT_BALANCE", params)
    }

    /// Retrieve a list of all transactions for a main account.
    pub fn get_admin_account_transaction_history(
        client: &Client,
        params: AdminAccountTransactionHistoryRequest,
    ) -> Response<R<AdminAccountTransactionHistoryResponseData>> {
        client.post_body("/v1", "ADMIN_MAIN_ACCOUNT_TRANSACTIONS", params)
    }

    /// Retrieve a list of filtered transactions for a main account.
    pub fn get_admin_account_filtered_transaction_history(
        client: &Client,
        params: AdminAccountFilteredTransactionHistoryRequest,
    ) -> Response<R<AdminAccountTransactionHistoryResponseData>> {
        client.post_body("/v1", "ADMIN_MAIN_ACCOUNT_FILTERED_TRANSACTIONS", params)
    }

    /// Confirm Transfer Recipient.
    pub fn enquire_bank_account(
        client: &Client,
        params: AccountEnquiryRequest,
    ) -> Response<R<AccountEnquiryResponseData>> {
        client.post_body("/v1", "NAME_ENQUIRY", params)
    }

    /// Send Money from a Kuda Account.
    pub fn fund_transfer(
        client: &Client,
        params: AdminAccountFundTransferRequest,
    ) -> Response<R<serde_json::Value>> {
        client.post_body("/v1", "SINGLE_FUND_TRANSFER", params)
    }

    /// Check the status of a bank transfer.
    pub fn check_transfer_status(
        client: &Client,
        params: AdminAccountCheckTransferRequest,
    ) -> Response<R<serde_json::Value>> {
        client.post_body("/v1", "TRANSACTION_STATUS_QUERY", params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    const PUBLIC_KEY: &str = "CHANGE_ME";

    const PRIVATE_KEY: &str = "CHANGE_ME";

    #[tokio::test]
    async fn test_get_admin_account_balance() {
        let client = Client::new(
            "https://kuda-openapi-uat.kudabank.com".to_string(),
            PRIVATE_KEY.to_string(),
            PUBLIC_KEY.to_string(),
            "CHANGE_ME".to_string(),
        );

        let mut params = AdminAccountBalanceRequest::new();
        params.tracking_reference = "0012".to_string();

        let response: Result<R<AdminAccountGetBalanceResponseData>, Error> =
            AdminAccount::get_admin_account_balance(&client, params).await;
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
    async fn test_get_admin_account_transaction_history() {
        let client = Client::new(
            "https://kuda-openapi-uat.kudabank.com".to_string(),
            PRIVATE_KEY.to_string(),
            PUBLIC_KEY.to_string(),
            "CHANGE_ME".to_string(),
        );

        let mut params = AdminAccountTransactionHistoryRequest::new();
        params.tracking_reference = "0012".to_string();
        params.page_number = 1;
        params.page_size = 100;

        let response: Result<R<AdminAccountTransactionHistoryResponseData>, Error> =
            AdminAccount::get_admin_account_transaction_history(&client, params).await;
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
    async fn test_get_admin_account_filtered_transaction_history() {
        let client = Client::new(
            "https://kuda-openapi-uat.kudabank.com".to_string(),
            PRIVATE_KEY.to_string(),
            PUBLIC_KEY.to_string(),
            "CHANGE_ME".to_string(),
        );

        let mut params = AdminAccountFilteredTransactionHistoryRequest::new();
        params.tracking_reference = "0012".to_string();
        params.start_date = "2021-05-24T00:00:00".to_string();
        params.end_date = "2021-05-24T23:59:59".to_string();
        params.page_number = 1;
        params.page_size = 100;

        let response: Result<R<AdminAccountTransactionHistoryResponseData>, Error> =
            AdminAccount::get_admin_account_filtered_transaction_history(&client, params).await;
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
    async fn test_enquire_bank_account() {
        let client = Client::new(
            "https://kuda-openapi-uat.kudabank.com".to_string(),
            PRIVATE_KEY.to_string(),
            PUBLIC_KEY.to_string(),
            "CHANGE_ME".to_string(),
        );

        let mut params = AccountEnquiryRequest::new();
        params.beneficiary_account_number = "0012".to_string();
        params.beneficiary_bank_code = "0012".to_string();

        let response: Result<R<AccountEnquiryResponseData>, Error> =
            AdminAccount::enquire_bank_account(&client, params).await;
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
    async fn test_admin_account_fund_transfer() {
        let client = Client::new(
            "https://kuda-openapi-uat.kudabank.com".to_string(),
            PRIVATE_KEY.to_string(),
            PUBLIC_KEY.to_string(),
            "CHANGE_ME".to_string(),
        );

        let mut params = AdminAccountFundTransferRequest::new();
        params.beneficiary_account = "0012".to_string();
        params.beneficiary_bank_code = "0012".to_string();
        params.beneficiary_name = "0012".to_string();
        params.amount = "10000".to_string();
        params.narration = "test fund transfer".to_string();
        params.name_enquiry_session_id = "0".to_string();
        params.tracking_reference = "0012".to_string();
        params.sender_name = "test sender".to_string();

        let response: Result<R<serde_json::Value>, Error> =
            AdminAccount::fund_transfer(&client, params).await;
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

    #[tokio::test]
    async fn test_check_transfer_status() {
        let client = Client::new(
            "https://kuda-openapi-uat.kudabank.com".to_string(),
            PRIVATE_KEY.to_string(),
            PUBLIC_KEY.to_string(),
            "CHANGE_ME".to_string(),
        );

        let mut params = AdminAccountCheckTransferRequest::new();
        params.is_third_party_bank_transfer = true;
        params.transaction_request_reference = "SP-SBI8IIU".to_string();

        let response: Result<R<serde_json::Value>, Error> =
            AdminAccount::check_transfer_status(&client, params).await;
        let account = match response {
            Err(err) => panic!("{:?}", err),
            Ok(ok) => {
                println!("{:?}", ok);
                ok
            }
        };

        assert_eq!(account.status, true);
        assert_eq!(account.message, "RecordNotFound");
    }
}
