import MyAlgo from "@randlabs/myalgo-connect";

const myAlgoWallet = new MyAlgo();

/*Warning: Browser will block pop-up if user doesn't trigger myAlgoWallet.connect() with a button interation */
// Note that we don't do try catch: This returns Result<JsValue, JsValue>, which is handled in Rust
export const connectWallet = async () => {
  const accounts = await myAlgoWallet.connect();
  const addresses = accounts.map((account) => account.address);
  console.log("Wallet connected! addresses: " + addresses);
  return addresses;
};

// Note that we don't do try catch: This returns Result<JsValue, JsValue>, which is handled in Rust
export const signTransaction = async (transaction) => {
  console.log("Transaction in js: " + JSON.stringify(transaction));
  let signedTxn = await myAlgoWallet.signTransaction(transaction);
  signedTxn.blob = Array.from(signedTxn.blob); // Uint8Array -> array (otherwise parsing to Vec<u8> in Rust doesn't work)
  return signedTxn;
};
