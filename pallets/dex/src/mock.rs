use crate as pallet_dex;
use frame_support::traits::{AsEnsureOriginWithArg, ConstU128, ConstU16, ConstU32, ConstU64};
use frame_system::{EnsureRoot, EnsureSigned};
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
	FixedU128
};
use sp_runtime::traits::Convert;
use pallet_dex::AssetBalanceOf;

type Block = frame_system::mocking::MockBlock<Test>;
type Balance = u128;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Balances: pallet_balances,
		Assets: pallet_assets,
		Dex: pallet_dex,
	}
);

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_balances::Config for Test {
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ConstU128<1>;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ConstU32<10>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type RuntimeHoldReason = ();
	type FreezeIdentifier = ();
	type MaxHolds = ConstU32<10>;
	type MaxFreezes = ConstU32<10>;
}

impl pallet_assets::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type AssetId = u32;
	type AssetIdParameter = codec::Compact<u32>;
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<Self::AccountId>>;
	type ForceOrigin = EnsureRoot<Self::AccountId>;
	type AssetDeposit = ConstU128<100>;
	type AssetAccountDeposit = ConstU128<1>;
	type MetadataDepositBase = ConstU128<10>;
	type MetadataDepositPerByte = ConstU128<1>;
	type ApprovalDeposit = ConstU128<1>;
	type StringLimit = ConstU32<50>;
	type Freezer = ();
	type Extra = ();
	type CallbackHandle = ();
	type WeightInfo = ();
	type RemoveItemsLimit = ConstU32<1000>;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

impl pallet_dex::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type NativeBalance = Balances;
	type Fungibles = Assets;
}

pub struct AssetBalanceConverter;
impl Convert<AssetBalanceOf<Test>, FixedU128> for AssetBalanceConverter {
	fn convert(a: AssetBalanceOf<Test>) -> FixedU128 {
		FixedU128::from_inner(a)
	}
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	// frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
	let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into();

	RuntimeGenesisConfig {
		assets: AssetsConfig {
			assets: vec![
				(1, 1, true, 1_000_000_000_000_000_000_000),
				(2, 1, true, 1_000_000_000_000_000_000_000),
				(3, 1, true, 1_000_000_000_000_000_000_000),
			],
			metadata: vec![
				(1, b"Bitcoin".to_vec(), b"BTC".to_vec(), 12),
				(2, b"Ethereum".to_vec(), b"ETH".to_vec(), 12),
				(2, b"Polkadot".to_vec(), b"DOT".to_vec(), 12),
			],
			accounts: vec![
				(1, 1, 1_000_000_000_000_000_000_000_000_000),
				(2, 1, 1_000_000_000_000_000_000_000_000_000),
				(3, 1, 1_000_000_000_000_000_000_000_000_000),
				(1, 2, 50_000_000_000_000_000_000_000_000),
				(2, 2, 50_000_000_000_000_000_000_000_000),
				(3, 2, 50_000_000_000_000_000_000_000_000),
			],
		},
		..Default::default()
	}.assimilate_storage(&mut t).unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
