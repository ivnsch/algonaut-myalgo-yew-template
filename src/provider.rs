use algonaut::{
    algod::v2::Algod,
    core::{Address, MicroAlgos},
    transaction::{Pay, Transaction, TxnBuilder},
};
use anyhow::{anyhow, Result};
use log::debug;
use my_algo::MyAlgo;

pub struct Provider {
    algod: Algod,
    my_algo: MyAlgo,
}

impl Provider {
    pub fn new(algod: Algod, my_algo: MyAlgo) -> Provider {
        Provider { algod, my_algo }
    }

    /// Runs My Algo connection flow and selects the first chosen address (demo purpose, you may want to prompt the user again).
    pub async fn connect_wallet(&self) -> Result<Address> {
        self.my_algo
            .connect_wallet()
            .await
            .and_then(|addresses| match addresses.get(0) {
                Some(address) => Ok(address.to_owned()),
                None => Err(anyhow!(
                    "Unexpected: My Algo connect success but no addresses"
                )),
            })
    }

    /// Creates payment transaction, asks user to sign via My Algo wallet and submits the signed transaction.
    /// Returns transaction id if successful, error otherwise.
    pub async fn send_payment(&self, data: &SendPaymentData) -> Result<String> {
        let transaction = self.create_payment_txn(data).await?;
        let signed_bytes = self.my_algo.sign(&transaction).await?;
        let res = self
            .algod
            .broadcast_raw_transaction(&signed_bytes.blob)
            .await?;
        debug!("Broadcast txn res: {:?}", res);
        Ok(res.tx_id)
    }

    async fn create_payment_txn(&self, data: &SendPaymentData) -> Result<Transaction> {
        let params = self.algod.transaction_params().await?;
        Ok(TxnBuilder::new()
            .sender(data.sender)
            .first_valid(params.last_round)
            .last_valid(params.last_round + 10)
            .genesis_id(params.genesis_id)
            .genesis_hash(params.genesis_hash)
            .fee(data.fee)
            .note("Hello Rust! 🦀".as_bytes().to_vec())
            .payment(Pay::new().amount(data.amount).to(data.receiver).build())
            .build())
    }
}

pub struct SendPaymentData {
    pub sender: Address,
    pub receiver: Address,
    pub amount: MicroAlgos,
    pub fee: MicroAlgos,
}
