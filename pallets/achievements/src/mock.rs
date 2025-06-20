use crate as pallet_achievements;
use frame_support::{
    derive_impl, parameter_types,
    traits::{ConstU32, Randomness},
};
use sp_runtime::{
    testing::H256,
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;

// Mock randomness implementation
pub struct MockRandomness;
impl Randomness<H256, u64> for MockRandomness {
    fn random(_subject: &[u8]) -> (H256, u64) {
        (H256::default(), 0)
    }
}

#[frame_support::runtime]
mod runtime {
    #[runtime::runtime]
    #[runtime::derive(
        RuntimeCall,
        RuntimeEvent,
        RuntimeError,
        RuntimeOrigin,
        RuntimeFreezeReason,
        RuntimeHoldReason,
        RuntimeSlashReason,
        RuntimeLockId,
        RuntimeTask
    )]
    pub struct Test;

    #[runtime::pallet_index(0)]
    pub type System = frame_system::Pallet<Test>;

    #[runtime::pallet_index(1)]
    pub type Achievements = pallet_achievements::Pallet<Test>;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type Block = Block;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Hash = H256;
    type Hashing = BlakeTwo256;
}

parameter_types! {
    pub const MaxAchievements: u32 = 1000;
}

impl pallet_achievements::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Balance = u64;
    type Moment = u64;
    type CollectionId = u32;
    type ItemId = u32;
    type MaxNameLength = ConstU32<64>;
    type MaxDescriptionLength = ConstU32<512>;
    type MaxMetadataLength = ConstU32<256>;
    type MaxUserAchievements = ConstU32<100>;
    type MaxAchievements = MaxAchievements;
    type Randomness = MockRandomness;
    type Nfts = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap()
        .into()
}
