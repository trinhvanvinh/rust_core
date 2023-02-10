use pallet_grandpa::{
    fg_primitives, AuthorityId as GrandpaId, AuthorityList as GrandpaAuthorityList,
};
use sp_api::impl_runtime_apis;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata, H256};
use sp_runtime::{
    create_runtime_str, generic, impl_opaque_keys,
    traits::{
        AccountIdLookup, BlakeTwo256, Block as BlockT, IdentifyAccount, NumberFor, One, Verify,
    },
    transaction_validity::{TransactionSource, TransactionValidity},
    ApplyExtrinsicResult, MultiSignature,
};

use sp_std::prelude::*;

use sp_version::{NativeVersion, RuntimeVersion};

pub use frame_support::{
    construct_runtime, parameter_types,
    traits::{
        ConstU128, ConstU32, ConstU64, ConstU8, KeyOwnerProofSystem, Randomness, StorageInfo,
    },
    weights::{
        constants::{
            BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_REF_TIME_PER_SECOND,
        },
        IdentityFee, Weight,
    },
    StorageValue,
};

pub use frame_system::Call as SystemCall;
pub use pallet_balances::Call as BalancesCall;
pub use pallet_timestamp::Call as TimestampCall;
use pallet_transaction_payment::{ConstFeeMultiplier, CurrencyAdapter, Multiplier};

pub use sp_runtime::{BuildStorage, Perbill, Permill};

pub type BlockNumber = u32;

pub type Signature = MultiSignature;

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

pub type Balance = u128;

pub type Index = u32;

pub type Hash = H256;

pub const MILLISECS_PER_BLOCK: u64 = 6000;

pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;
