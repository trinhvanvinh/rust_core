use crate::service::FullClient;
use node_template_runtime as runtime;
use runtime::{AccountId, Balance, BalancesCall, SystemCall};
use sc_cli::Result;
use sc_client_api::BlockBackend;
use sp_core::{Encode, Pair};
use sp_inherents::{InherentData, InherentDataProvider};
use sp_keyring::Sr25519Keyring;
use sp_runtime::{OpaqueExtrinsic, SaturatedConversion};

use std::{ops::Rem, sync::Arc, time::Duration};

pub struct RemarkBuilder {
    client: Arc<FullClient>,
}

impl RemarkBuilder {
    pub fn new(client: Arc<FullClient>) -> Self {
        Self { client }
    }
}

impl frame_benchmarking_cli::ExtrinsicBuilder for RemarkBuilder {
    fn pallet(&self) -> &str {
        "System"
    }

    fn extrinsic(&self) -> &str {
        "remark"
    }

    fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
        let acc = Sr25519Keyring::Bob.pair();
        let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
            &self.client,
            acc,
            SystemCall::remark { remark: vec![] }.into(),
            nonce,
        );
        Ok(extrinsic)
    }
}

pub struct TransferKeepAliveBuilder {
    client: Arc<FullClient>,
    dest: AccountId,
    value: Balance,
}

impl TransferKeepAliveBuilder {
    pub fn new(client: Arc<FullClient>, dest: AccountId, value: Balance) -> Self {
        Self {
            client,
            dest,
            value,
        }
    }
}

impl frame_benchmarking_cli::ExtrinsicBuilder for TransferKeepAliveBuilder {
    fn pallet(&self) -> &str {
        "balances"
    }
    fn extrinsic(&self) -> &str {
        "transfer_keep_alive"
    }
    fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
        let acc = Sr25519Keyring::Bob.pair();
        let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
            &self.client,
            acc,
            BalancesCall::transfer_keep_alive {
                dest: self.dest,
                value: self.value,
            },
            nonce,
        );
        Ok(extrinsic)
    }
}

pub fn create_benchmark_extrinsic(
    client: &FullClient,
    sender: sp_core::sr25519::Pair,
    call: runtime::RuntimeCall,
    nonce: u32,
) -> runtime::UncheckedExtrinsic {
    let genesis_hash = client
        .block_hash(0)
        .ok()
        .flatten()
        .expect("genesis block exists");
    let best_hash = client.chain_info().best_hash;
    let best_block = client.chain_info().best_number;

    let period = runtime::BlockHashCount::get()
        .checked_next_power_of_two()
        .map(|c| c / 2)
        .unwrap_or(2) as u64;
    let extra: runtime::SignedExtra = ();
    let raw_payload = runtime::SignedPayload::from_raw(call, extra, ());

    let signature = raw_payload.using_encoded(|e| sender.sign(e));

    runtime::UncheckedExtrinsic::new_signed(
        call,
        sp_runtime::AccountId32::from(sender),
        runtime::Signature::Sr25519(signature),
        extra,
    )
}

pub fn inherent_benchmark_data() -> Result<InherentData> {
    let mut inherent_data = InherentData::new();
    let d = Duration::from_millis(0);
    let timestamp = sp_timestamp::InherentDataProvider::new(d.into());

    futures::executor::block_on(timestamp.provide_inherent_data(&mut inherent_data))
        .map_err(|e| format!("create inherent data {:?}", e));

    Ok(inherent_data)
}
