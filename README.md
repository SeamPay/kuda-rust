# kuda-rust
Rust bindings for Kuda HTTP API

This repo contains the Kuda Bank SDK for Rust.

## Getting Started with the SDK

The SDK provides bindings to Kuda Bank REST API. You must add [Tokio](https://crates.io/crates/tokio) as a dependency 
within your Rust project to execute asynchronous code.

1. Create a new Rust project: `cargo new sdk-example`
2. Add dependencies to kuda-rust and Tokio to your **Cargo.toml** file:

 ```toml
[dependencies]
kuda-rust = { git = "https://github.com/SeamPay/kuda-rust", tag = "v0.1.0" }
tokio = { version = "1", features = ["full"] }
 ```
3. Make a request

```rust
use std::env;
use kuda::params::Response;
use kuda::{Client, VirtualAccount, VirtualAccountCreateRequest, VirtualAccountCreateResponseData};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let endpoint = env::var("KUDA_ENDPOINT").is_err();
    let privatekey = env::var("KUDA_PRIVATEKEY").is_err();
    let publickey = env::var("KUDA_PUBLICKEY").is_err();
    let clientkey = env::var("KUDA_CLIENTKEY").is_err();
  
    // create the client
    let kuda_client = Client::new(
        endpoint,
        privatekey,
        publickey,
        clientkey,
    );
    
    // create the request
    let request = VirtualAccountCreateRequest::new();
    params.first_name = "Mark".to_string();
    params.last_name = "Smith".to_string();
    params.phone_number = "09039658058".to_string();
    params.tracking_reference = "0012".to_string();
    params.email = "example@email.com".to_string();
    
    let resp = VirtualAccount::create_virtual_account(&kuda_client, request)
        .await?;
    println!("Response for creating virtual account: {:?}", resp);
    Ok(())
}
```

### Prerequisites

In order to use the SDK, you must already have Rust and Cargo installed. If you don't, [these instructions](https://doc.rust-lang.org/book/ch01-01-installation.html) describe how to install Rust and Cargo.

## Kuda Services Covered
- [x] Admin Account
  - [x] Get Admin Account Balance
  - [x] Get Admin Account Transaction History
  - [x] Get Admin Account Filtered Transaction History
  - [x] Enquire Bank Account
  - [x] Fund Transfer
  - [x] Check Transfer Status


- [x] Virtual Account
  - [x] Create Virtual Account
  - [x] Get Virtual Account
  - [x] Get Virtual Account Balance
  - [x] Get Virtual Account Transaction History
  - [x] Get Virtual Account Filtered Transaction History
  - [x] Fund Virtual Account
  - [x] Withdraw Virtual Account
  - [x] Fund Transfer Virtual Account


- [x] Banks
    - [x] List Banks
