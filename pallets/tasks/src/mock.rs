use crate as pallet_tasks;
use frame_support::{
    parameter_types,
    traits::{ConstU16, ConstU32, ConstU64, Randomness},
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

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub struct Test {
        System: frame_system,
        Tasks: pallet_tasks,
    }
);

impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type Block = Block;
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Nonce = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU64<250>;
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
    type RuntimeTask = ();
    type ExtensionsWeightInfo = ();
    type SingleBlockMigrations = ();
    type MultiBlockMigrator = ();
    type PreInherents = ();
    type PostInherents = ();
    type PostTransactions = ();
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
}

impl pallet_tasks::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Balance = u64;
    type Moment = u64;
    type MaxTitleLength = MaxTitleLength;
    type MaxDescriptionLength = MaxDescriptionLength;
    type MaxTasksPerUser = MaxTasksPerUser;
    type MaxTasksPerStatus = MaxTasksPerStatus;
    type MaxTasksPerPriority = MaxTasksPerPriority;
    type MaxTasksPerDeadline = MaxTasksPerDeadline;
    type Randomness = MockRandomness;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();
    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1));
    ext
}
