use crate as pallet_reputation;
use frame_support::{
    derive_impl, parameter_types,
    traits::{ConstU128, ConstU32, ConstU64},
};
use sp_core::H256;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;

// Mock randomness implementation
pub struct MockRandomness;
impl frame_support::traits::Randomness<H256, u64> for MockRandomness {
    fn random(_subject: &[u8]) -> (H256, u64) {
        (H256::default(), 0)
    }
}

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        Balances: pallet_balances,
        Timestamp: pallet_timestamp,
        Tasks: pallet_tasks,
        Reputation: pallet_reputation,
    }
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type Block = Block;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountData = pallet_balances::AccountData<u128>;
}

impl pallet_balances::Config for Test {
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type Balance = u128;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU128<500>;
    type AccountStore = System;
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Test>;
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type RuntimeHoldReason = ();
    type RuntimeFreezeReason = ();
    type DoneSlashHandler = ();
}

impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = ConstU64<5>;
    type WeightInfo = ();
}

// 任务pallet配置的常量
parameter_types! {
    pub const MaxTitleLength: u32 = 100;
    pub const MaxDescriptionLength: u32 = 1000;
    pub const MaxTasksPerUser: u32 = 100;
    pub const MaxTasksPerStatus: u32 = 1000;
    pub const MaxTasksPerPriority: u32 = 1000;
    pub const MaxTasksPerDeadline: u32 = 1000;
    pub const MinVerificationVotes: u32 = 3;
    pub const MinApprovalPercentage: u32 = 60;
    pub const VerificationPeriod: u64 = 100;
}

impl pallet_tasks::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Balance = u128;
    type Moment = u64;
    type Currency = Balances;
    type MaxTitleLength = MaxTitleLength;
    type MaxDescriptionLength = MaxDescriptionLength;
    type MaxTasksPerUser = MaxTasksPerUser;
    type MaxTasksPerStatus = MaxTasksPerStatus;
    type MaxTasksPerPriority = MaxTasksPerPriority;
    type MaxTasksPerDeadline = MaxTasksPerDeadline;
    type Randomness = MockRandomness;
    type MinVerificationVotes = MinVerificationVotes;
    type MinApprovalPercentage = MinApprovalPercentage;
    type VerificationPeriod = VerificationPeriod;
}

// 声誉pallet配置的常量
parameter_types! {
    pub const MaxCommentLength: u32 = 500;
    pub const MaxEvaluationsPerTask: u32 = 50;
    pub const MaxReputationHistory: u32 = 100;
    pub const BaseReputationScore: u32 = 10;
    pub const DifficultyBonus: u32 = 2;
    pub const CancellationPenalty: u32 = 5;
}

impl pallet_reputation::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Balance = u128;
    type Moment = u64;
    type MaxCommentLength = MaxCommentLength;
    type MaxEvaluationsPerTask = MaxEvaluationsPerTask;
    type MaxReputationHistory = MaxReputationHistory;
    type BaseReputationScore = BaseReputationScore;
    type DifficultyBonus = DifficultyBonus;
    type CancellationPenalty = CancellationPenalty;
    type Randomness = MockRandomness;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();
    pallet_balances::GenesisConfig::<Test> {
        balances: vec![(1, 1000), (2, 1000), (3, 1000)],
        dev_accounts: Default::default(),
    }
    .assimilate_storage(&mut t)
    .unwrap();

    t.into()
}
