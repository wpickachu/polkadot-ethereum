// Mock runtime
use frame_support::{dispatch::DispatchResult, parameter_types};
use frame_system as system;
use sp_core::{H160, H256};
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentifyAccount, IdentityLookup, Verify},
	MultiSignature,
};

use artemis_core::{AssetId, ChannelId, SubmitOutboundChannel};

use crate as erc20_app;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Module, Call, Storage, Event<T>},
		Assets: artemis_assets::{Module, Call, Storage, Event<T>},
		Dispatch: artemis_dispatch::{Module, Call, Storage, Origin, Event<T>},
		ERC20App: erc20_app::{Module, Call, Config, Storage, Event<T>},
	}
);

pub type Signature = MultiSignature;

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

impl system::Config for Test {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
}

impl artemis_assets::Config for Test {
	type Event = Event;
}

impl artemis_dispatch::Config for Test {
	type Origin = Origin;
	type Event = Event;
	type MessageId = u64;
	type Call = Call;
	type CallFilter = ();
}

pub struct MockSubmitOutbound;

impl SubmitOutboundChannel for MockSubmitOutbound {
	fn submit(_: ChannelId, _: H160, _: &[u8]) -> DispatchResult {
		Ok(())
	}
}

parameter_types! {
	pub const EthAssetId: AssetId = AssetId::ETH;
}

impl erc20_app::Config for Test {
	type Event = Event;
	type Assets = Assets;
	type SubmitOutbound = MockSubmitOutbound;
	type CallOrigin = artemis_dispatch::EnsureEthereumAccount;
}

pub fn new_tester() -> sp_io::TestExternalities {
	let mut storage = system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap();

	let config = erc20_app::GenesisConfig {
		address: H160::repeat_byte(1),
	};
	config.assimilate_storage(&mut storage).unwrap();

	let mut ext: sp_io::TestExternalities = storage.into();
	ext.execute_with(|| System::set_block_number(1));
	ext
}
