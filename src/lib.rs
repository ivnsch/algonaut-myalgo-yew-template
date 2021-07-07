#![recursion_limit = "1024"]

mod bindings;
mod dependencies;
mod my_algo;
mod provider;
mod to_my_algo_transaction;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::provider::SendPaymentData;

use algonaut::core::{Address, MicroAlgos};
use anyhow::Result;
use wasm_bindgen::JsValue;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yewtil::future::LinkFuture;

pub struct Model {
    link: ComponentLink<Self>,
    address: Option<Address>,
    payment_inputs: PaymentInputs,
    payment_result_msg: Option<String>,
}

#[derive(Clone, Debug)]
pub enum Msg {
    Connect,
    Send,
    SetAddress(Address),
    UpdateReceiverInput(String),
    UpdateAmountInput(String),
    UpdateFeeInput(String),
    ShowPaymentResultMsg(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            address: None,
            payment_inputs: PaymentInputs::default(),
            payment_result_msg: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Connect => self.link.send_future(async move {
                let msg = Self::connect_wallet().await;
                msg
            }),
            Msg::Send => {
                let address = self.address.clone();
                let payment_inputs = self.payment_inputs.clone();
                self.link.send_future(async move {
                    Msg::ShowPaymentResultMsg(match address {
                        Some(address) => match Self::send_payment(address, payment_inputs).await {
                            Ok(tx_id) => format!("Success! Tx id: {}", tx_id),
                            Err(e) => format!("Error: {}", e),
                        },
                        None => "Not connected or no addresses".to_owned(),
                    })
                });
            }
            Msg::UpdateReceiverInput(input) => self.payment_inputs.receiver = input,
            Msg::UpdateAmountInput(input) => self.payment_inputs.amount = input,
            Msg::UpdateFeeInput(input) => self.payment_inputs.fee = input,
            Msg::SetAddress(address) => self.address = Some(address),
            Msg::ShowPaymentResultMsg(msg) => self.payment_result_msg = Some(msg),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::Connect)>{ "Connect" }</button>
                <div class="your-address">{ "Your address: " }{self.address.clone().map(|a| a.to_string()).unwrap_or_else(|| "".to_owned())}</div>
                <div> { "Send a payment"} </div>
                <div class="form">
                    <input
                        placeholder="Receiver"
                        size=64
                        value=self.payment_inputs.receiver.clone()
                        oninput=self.link.callback(|e: InputData| Msg::UpdateReceiverInput(e.value))
                    />
                    <input
                        placeholder="Amount (microAlgos)"
                        size=20
                        value=self.payment_inputs.amount.clone()
                        oninput=self.link.callback(|e: InputData| Msg::UpdateAmountInput(e.value))
                    />
                    <input
                        placeholder="Fee (microAlgos)"
                        size=20
                        value=self.payment_inputs.fee.clone()
                        oninput=self.link.callback(|e: InputData| Msg::UpdateFeeInput(e.value))
                    />
                    <button onclick=self.link.callback(|_| Msg::Send)>{ "Send" }</button>
                </div>
                <div>{ "Transaction result: "} {self.payment_result_msg.clone().unwrap_or_else(|| "".to_owned()) }</div>
            </div>
        }
    }
}

impl Model {
    async fn connect_wallet() -> Msg {
        let provider = dependencies::provider(dependencies::algod(), dependencies::my_algo());

        match provider.connect_wallet().await {
            Ok(address) => Msg::SetAddress(address),
            Err(e) => Msg::ShowPaymentResultMsg(format!("Error connecting wallet: {}", e)),
        }
    }

    async fn send_payment(sender: Address, inputs: PaymentInputs) -> Result<String> {
        let validated_inputs = Self::validate_payment_inputs(inputs)?;
        let provider = dependencies::provider(dependencies::algod(), dependencies::my_algo());

        provider
            .send_payment(&SendPaymentData {
                sender,
                receiver: validated_inputs.receiver,
                amount: validated_inputs.amount,
                fee: validated_inputs.fee,
            })
            .await
    }

    fn validate_payment_inputs(inputs: PaymentInputs) -> Result<ValidatedPaymentInputs> {
        let receiver = inputs.receiver.parse().map_err(anyhow::Error::msg)?;
        let amount = MicroAlgos(inputs.amount.parse()?);
        let fee = MicroAlgos(inputs.fee.parse()?);
        Ok(ValidatedPaymentInputs {
            receiver,
            amount,
            fee,
        })
    }
}

#[derive(Debug, Clone, Default)]
struct PaymentInputs {
    receiver: String,
    amount: String,
    fee: String,
}

pub struct ValidatedPaymentInputs {
    pub receiver: Address,
    pub amount: MicroAlgos,
    pub fee: MicroAlgos,
}

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
    Ok(())
}
