use crate as pallet_tasks;
use frame_support::{
    derive_impl, parameter_types,
    traits::{ConstU128, ConstU32, Randomness},
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
    pub type Balances = pallet_balances::Pallet<Test>;

    #[runtime::pallet_index(2)]
    pub type Tasks = pallet_tasks::Pallet<Test>;
}

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
    type RuntimeHoldReason = RuntimeHoldReason;
    type RuntimeFreezeReason = RuntimeFreezeReason;
    type DoneSlashHandler = ();
}

parameter_types! {
    #[derive(Clone, PartialEq)]
    pub const MaxTitleLength: u32 = 100;
    #[derive(Clone, PartialEq)]
    pub const MaxDescriptionLength: u32 = 1000;
    #[derive(Clone, PartialEq)]
    pub const MaxTasksPerUser: u32 = 100;
    #[derive(Clone, PartialEq)]
    pub const MaxTasksPerStatus: u32 = 1000;
    #[derive(Clone, PartialEq)]
    pub const MaxTasksPerPriority: u32 = 1000;
    #[derive(Clone, PartialEq)]
    pub const MaxTasksPerDeadline: u32 = 1000;
    #[derive(Clone, PartialEq)]
    pub const MinVerificationVotes: u32 = 3;
    #[derive(Clone, PartialEq)]
    pub const MinApprovalPercentage: u32 = 60;
    #[derive(Clone, PartialEq)]
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

    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1));
    ext
}
