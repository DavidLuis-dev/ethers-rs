//! Provides a unified interface for locally signing transactions and interacting
//! with the Ethereum JSON-RPC. You can implement the `Signer` trait to extend
//! functionality to other signers such as Hardware Security Modules, KMS etc.
//!
//! ```no_run
//! # use ethers::{
//!     providers::{Http, Provider},
//!     signers::Wallet,
//!     core::types::TransactionRequest
//! };
//! # use std::convert::TryFrom;
//! # async fn foo() -> Result<(), Box<dyn std::error::Error>> {
//! // connect to the network
//! let provider = Provider::<Http>::try_from("http://localhost:8545")?;
//!
//! // instantiate the wallet and connect it to the provider to get a client
//! let client = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7"
//!     .parse::<Wallet>()?
//!     .connect(provider);
//!
//! // create a transaction
//! let tx = TransactionRequest::new()
//!     .to("vitalik.eth") // this will use ENS
//!     .value(10000);
//!
//! // send it! (this will resolve the ENS name to an address under the hood)
//! let tx_hash = client.send_transaction(tx, None).await?;
//!
//! // get the receipt
//! let receipt = client.pending_transaction(tx_hash).await?;
//!
//! // get the mined tx
//! let tx = client.get_transaction(receipt.transaction_hash).await?;
//!
//! println!("{}", serde_json::to_string(&tx)?);
//! println!("{}", serde_json::to_string(&receipt)?);
//!
//! # Ok(())
//! # }
mod wallet;
pub use wallet::Wallet;

#[cfg(feature = "ledger")]
mod ledger;
#[cfg(feature = "ledger")]
pub use ledger::{app::LedgerEthereum as Ledger, types::{LedgerError, DerivationType as HDPath}};

mod nonce_manager;
pub(crate) use nonce_manager::NonceManager;

mod client;
pub use client::{Client, ClientError};

use async_trait::async_trait;
use ethers_core::types::{Address, Signature, Transaction, TransactionRequest};
use ethers_providers::Http;
use std::error::Error;

/// Trait for signing transactions and messages
///
/// Implement this trait to support different signing modes, e.g. Ledger, hosted etc.
#[async_trait(?Send)]
pub trait Signer {
    type Error: Error + Into<ClientError>;
    /// Signs the hash of the provided message after prefixing it
    async fn sign_message<S: Send + Sync + AsRef<[u8]>>(
        &self,
        message: S,
    ) -> Result<Signature, Self::Error>;

    /// Signs the transaction
    async fn sign_transaction(
        &self,
        message: TransactionRequest,
    ) -> Result<Transaction, Self::Error>;

    /// Returns the signer's Ethereum Address
    async fn address(&self) -> Result<Address, Self::Error>;
}

/// An HTTP client configured to work with ANY blockchain without replay protection
pub type HttpClient = Client<Http, Wallet>;
