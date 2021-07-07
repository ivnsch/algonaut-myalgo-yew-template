use crate::{bindings, to_my_algo_transaction};
use algonaut::{core::Address, transaction::Transaction};
use anyhow::Result;
use log::debug;
use wasm_bindgen::JsValue;

/// Wrapper to talk with My Algo
pub struct MyAlgo {}

impl MyAlgo {
    /// Runs My Algo flow to connect and select accounts.
    pub async fn connect_wallet(&self) -> Result<Vec<Address>> {
        debug!("Will connect wallet");
        let res = bindings::connect_wallet().await.map_err(|js_value| {
            anyhow::Error::msg(format!("Error connecting wallet: {:?}", js_value))
        })?;
        let addresses: Vec<Address> = JsValue::into_serde::<Vec<String>>(&res)
            .unwrap()
            .into_iter()
            .map(|s| s.parse().unwrap())
            .collect();

        debug!("Finished connecting wallet, addresses: {:?}", addresses);
        Ok(addresses)
    }

    /// Runs My Algo signing flow.
    pub async fn sign(&self, transaction: &Transaction) -> Result<MyAlgoSignedTransaction> {
        let transaction_js = to_my_algo_transaction::to_my_algo_transaction(&transaction.clone())?;
        debug!("Transaction JsValue: {:?}", transaction_js);
        let signed_transaction_js =
            bindings::sign_transaction(transaction_js)
                .await
                .map_err(|js_value| {
                    anyhow::Error::msg(format!("Error signing transaction: {:?}", js_value))
                })?;
        let signed_transaction =
            JsValue::into_serde::<MyAlgoSignedTransaction>(&signed_transaction_js)?;
        debug!("Signed transaction: {:?}", signed_transaction);
        Ok(signed_transaction)
    }
}

/// Rust mapping of signed transaction JSON returned by My Algo
#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize)]
pub struct MyAlgoSignedTransaction {
    #[serde(rename = "txID")]
    pub tx_id: String,
    pub blob: Vec<u8>,
}
