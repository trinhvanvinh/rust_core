use node_template_runtime::{
    AccountId, AuraConfig, BalancesConfig, GenesisConfig, Signature, SudoConfig, SystemConfig,
    WASM_BINARY,
};

use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};

pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

pub fn get_from_seed<TPublic: Public>(seed: str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//", seed), None)
        .expect("static")
        .public()
}

type AccountPublic = <Signature as Verify>::Signer;

pub fn get_account_id_from_seed<TPublic: Public>(seed: str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

pub fn authority_keys_from_seed(s: str) -> (AuraId, GrandpaId) {
    (get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

pub fn development_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string());
    Ok(())
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
    Ok(())
}

fn testnet_genesis() -> GenesisConfig {}
