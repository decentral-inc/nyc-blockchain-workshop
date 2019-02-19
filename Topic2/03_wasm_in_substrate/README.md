# Topic 2-3. wasm in substrate

On topic 2-3, we will build a simple game with blockchain in 30 mins using substrate.

# Substrate

Substrate is a blockchain platform for innovators who wants to innovate social infrastructure of his or her community. It is also a building block of Polkadot network, the internet of blockchains.

### Prerequisite

To install substrate, you should have:

- Node + NPM
- VSCode
- Chrome or Chromium based browser(e.g. microsoft edge)

### Setup

to install substrate, 

```bash
curl https://getsubstrate.io -sSf | bash
```

bear with me, it is going to take long.

### To start developing a blockchain protocol, run

```bash
substrate-node-new <BLOCKCHAIN> <AUTHOR>
```

this will make boilerplate for setting your own blockchain. Default supporting features are:

- Consensus(PoA, PBFT, etc)
- State Machine(Ethereum)
- ALICE(Genesis Account) to test transfer

to run the node, run 
```bash
cd <BLOCKCHAIN>
./target/release/substratekitties --dev
```

If you are successful following the instruction, you should see this on terminal.

![the right way](https://shawntabrizi.github.io/substrate-collectables-workshop/0/assets/building-blocks.png)


### Interacting with substrate with polkadot-JS

So polkadot-js is used to interact with nodes which consists of substrate nodes, which is why it is used for interacting with substrate. On initial state of Substrate, Alice has all Dot tokens. the other accounts are used for testing.

![sending transaction](https://shawntabrizi.github.io/substrate-collectables-workshop/0/assets/first-transfer.png)


### Resetting the chain state/data

To reset the blockchain protocol on development process(only) for significant update, run command
```bash
./target/release/<BLOCKCHAIN> purge-chain --dev
```

### Building your runtime module

On Substrate, wasm is used for building logics in the blockchain, which means pure native code is developed with rust instead of applying EVM and its domain specific language(e.g. solidity, vyper). This also means it also supports other programming languages as long as it can be compiled to wasm. Go is preparing its implementation from Chainsafe🚀🚀🚀. Let us learn wasm for now with rust. What's more, Substrate introduces runtime processed in faster rate than ethereum smart contract.

### Runtime & Module vs. Smart contract

Smart contract is a ethereum byte opcodes stored in key-value database(e.g. BoltsDB for Eth2.0 by Prysmatic lab). Here is the execution process for smart contract.

1. User sends encrypted transaction with ethereum byte code data to smart contract account
2. Ethereum blockchain verifies tx and locates the contract address as key and look for "code" property where the ethereum opcode is stored in its embedded key-value storage. 
3. VM in ethereum node processes execution from the input from transaction to the stored ethereum byte code
4. result is applied to state and state root is added to the block.

On the other hand, Runtime is a built-in "state transition function"(STF), which directly changes the state of the blockchain. On ethereum there was one STF, `apply_transaction`. Without going through EVM, transactions can be processed with much less computational overhead. They do not go through metering for its opcodes, so only arguments for the runtime endpoints are required on JSON-RPC call, downsizing the request data significantly. Here is the execution process for runtime module.

1. User sends encrypted transaction to a Substrate node to its runtime endpoint
2. Substrate node verifies tx and processes built-in function and change state if the tx requests for change
3. State root is added to the block to record the change

Modules are compared to built-in functions in Ethereum smart contract(e.g. keccak256, ecrecover). They provide features and STFs in modular way to operate Substrate blockchain such as:

- Account Management
- Token Balances
- Governances
- Runtime Upgrades
- and more to be added with open source contribution

### Creating a new module

We will build our own blockchain function 
to start with, let us locate to `runtime/` in substrate folder.
We are building a simple coin-flip game with the special event where reward is given to every 42th player of the game.

make file `tutorial.rs` and code as below.
```rust
use support::{decl_storage, decl_module, StorageValue, dispatch::Result, decl_event};
use {balances, system::{self, ensure_signed}};
use runtime_primitives::traits::Hash;
use parity_codec::Encode;


pub trait Trait: balances::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}


decl_storage! {
    trait Store for Module<T: Trait> as Storage {
        // Declare storage and getter function here
        Trial get(get_trial): u64;
        Payment get(payment): Option<T::Balance>;
        Pot get(pot): T::Balance;
    }
}



decl_event!(
    pub enum Event<T>
    where
        <T as balances::Trait>::Balance
    {
        Win(Balance),
        Deposit(Balance),
        Trial(u64),
        Payment(Balance),
    }
);

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn deposit_event<T>() = default;


        fn play(origin) -> Result {
            let sender = ensure_signed(origin)?;

            let payment = Self::payment().ok_or("Must have payment amount set")?;

            <balances::Module<T>>::decrease_free_balance(&sender, payment)?;


            if <Trial<T>>::get() == 42  {
                <Trial<T>>::mutate(|trial| *trial = 0);
                Self::deposit_event(RawEvent::Win(<Pot<T>>::get()));
                <balances::Module<T>>::increase_free_balance_creating(&sender, <Pot<T>>::take());
            }

            if(<system::Module<T>>::random_seed(), &sender).using_encoded(<T as system::Trait>::Hashing::hash).using_encoded(|e| e[0] < 128)
            {
               Selfknpm ::deposit_event(RawEvent::Win(<Pot<T>>::get()));

               <balances::Module<T>>::increase_free_balance_creating(&sender, <Pot<T>>::take()); 
            }

            <Pot<T>>::mutate(|pot| *pot += payment);    
            <Trial<T>>::mutate(|trial| *trial += 1);
            Self::deposit_event(RawEvent::Deposit(payment));
            
            Ok(())        
        } 

        fn set_payment(_origin, value: T::Balance) -> Result {
            <Trial<T>>::put(0);
            let trial = 0;
            Self::deposit_event(RawEvent::Trial(trial));
  
            //If the payment has not been set...
            if Self::payment().is_none() {
            // ... we will set it to the value we passed in.
            <Payment<T>>::put(value);
    
            // We will also put that initial value into the pot for someone to win
            <Pot<T>>::put(value);
            }  

            Self::deposit_event(RawEvent::Payment(value));
  
            Ok(())
        }       
    }
}
```

`decl_storage!` is where you define the data to store. In this tutorial we store a number to count number of trials.

`decl_module` is where you declare functions for operating substrate.

`decl_event` is where you declare events to subsribe changes in blockchain state.

in `lib.rs`
```rust
//! The Substrate Node Template runtime. This can be compiled with `#[no_std]`, ready for Wasm.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit="256"]

#[cfg(feature = "std")]
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate parity_codec_derive;

use rstd::prelude::*;
#[cfg(feature = "std")]
use primitives::bytes;
use primitives::{Ed25519AuthorityId, OpaqueMetadata};
use runtime_primitives::{
	ApplyResult, transaction_validity::TransactionValidity, Ed25519Signature, generic,
	traits::{self, BlakeTwo256, Block as BlockT, StaticLookup}, create_runtime_str
};
use client::{
	block_builder::api::{CheckInherentsResult, InherentData, self as block_builder_api},
	runtime_api, impl_runtime_apis
};
use version::RuntimeVersion;
#[cfg(feature = "std")]
use version::NativeVersion;

// A few exports that help ease life for downstream crates.
#[cfg(any(feature = "std", test))]
pub use runtime_primitives::BuildStorage;
pub use consensus::Call as ConsensusCall;
pub use timestamp::Call as TimestampCall;
pub use balances::Call as BalancesCall;
pub use runtime_primitives::{Permill, Perbill};
pub use timestamp::BlockPeriod;
pub use support::{StorageValue, construct_runtime};

/// Alias to Ed25519 pubkey that identifies an account on the chain.
pub type AccountId = primitives::H256;

/// A hash of some data used by the chain.
pub type Hash = primitives::H256;

/// Index of a block number in the chain.
pub type BlockNumber = u64;

/// Index of an account's extrinsic in the chain.
pub type Nonce = u64;


mod tutorial;
/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core datastructures.
pub mod opaque {
	use super::*;

	/// Opaque, encoded, unchecked extrinsic.
	#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
	pub struct UncheckedExtrinsic(#[cfg_attr(feature = "std", serde(with="bytes"))] pub Vec<u8>);
	impl traits::Extrinsic for UncheckedExtrinsic {
		fn is_signed(&self) -> Option<bool> {
			None
		}
	}
	/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256, generic::DigestItem<Hash, Ed25519AuthorityId>>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;
	/// Opaque session key type.
	pub type SessionKey = Ed25519AuthorityId;
}

/// This runtime version.
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("substrate-tutorial"),
	impl_name: create_runtime_str!("substrate-tutorial"),
	authoring_version: 3,
	spec_version: 3,
	impl_version: 0,
	apis: RUNTIME_API_VERSIONS,
};

/// The version infromation used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion {
		runtime_version: VERSION,
		can_author_with: Default::default(),
	}
}

impl system::Trait for Runtime {
	/// The identifier used to distinguish between accounts.
	type AccountId = AccountId;
	/// The lookup mechanism to get account ID from whatever is passed in dispatchers.
	type Lookup = Indices;
	/// The index type for storing how many extrinsics an account has signed.
	type Index = Nonce;
	/// The index type for blocks.
	type BlockNumber = BlockNumber;
	/// The type for hashing blocks and tries.
	type Hash = Hash;
	/// The hashing algorithm used.
	type Hashing = BlakeTwo256;
	/// The header digest type.
	type Digest = generic::Digest<Log>;
	/// The header type.
	type Header = generic::Header<BlockNumber, BlakeTwo256, Log>;
	/// The ubiquitous event type.
	type Event = Event;
	/// The ubiquitous log type.
	type Log = Log;
	/// The ubiquitous origin type.
	type Origin = Origin;
}

impl aura::Trait for Runtime {
	type HandleReport = ();
}

impl consensus::Trait for Runtime {
	/// The identifier we use to refer to authorities.
	type SessionKey = Ed25519AuthorityId;
	// The aura module handles offline-reports internally
	// rather than using an explicit report system.
	type InherentOfflineReport = ();
	/// The ubiquitous log type.
	type Log = Log;
}

impl indices::Trait for Runtime {
	/// The type for recording indexing into the account enumeration. If this ever overflows, there
	/// will be problems!
	type AccountIndex = u32;
	/// Use the standard means of resolving an index hint from an id.
	type ResolveHint = indices::SimpleResolveHint<Self::AccountId, Self::AccountIndex>;
	/// Determine whether an account is dead.
	type IsDeadAccount = Balances;
	/// The uniquitous event type.
	type Event = Event;
}

impl timestamp::Trait for Runtime {
	/// A timestamp: seconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = Aura;
}

impl balances::Trait for Runtime {
	/// The type for recording an account's balance.
	type Balance = u128;
	/// What to do if an account's free balance gets zeroed.
	type OnFreeBalanceZero = ();
	/// What to do if a new account is created.
	type OnNewAccount = Indices;
	/// Restrict whether an account can transfer funds. We don't place any further restrictions.
	type EnsureAccountLiquid = ();
	/// The uniquitous event type.
	type Event = Event;
}

impl sudo::Trait for Runtime {
	/// The uniquitous event type.
	type Event = Event;
	type Proposal = Call;
}

impl tutorial::Trait for Runtime {
	type Event = Event;
}

construct_runtime!(
	pub enum Runtime with Log(InternalLog: DigestItem<Hash, Ed25519AuthorityId>) where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: system::{default, Log(ChangesTrieRoot)},
		Timestamp: timestamp::{Module, Call, Storage, Config<T>, Inherent},
		Consensus: consensus::{Module, Call, Storage, Config<T>, Log(AuthoritiesChange), Inherent},
		Aura: aura::{Module},
		Indices: indices,
		Balances: balances,
		Sudo: sudo,
        Tutorial: tutorial::{Module, Call, Storage, Event<T>},
	}
);

/// The type used as a helper for interpreting the sender of transactions.
type Context = system::ChainContext<Runtime>;
/// The address format for describing accounts.
type Address = <Indices as StaticLookup>::Source;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256, Log>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedMortalCompactExtrinsic<Address, Nonce, Call, Ed25519Signature>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Nonce, Call>;
/// Executive: handles dispatch to the various modules.
pub type Executive = executive::Executive<Runtime, Block, Context, Balances, AllModules>;

// Implement our runtime API endpoints. This is just a bunch of proxying.
impl_runtime_apis! {
	impl runtime_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn authorities() -> Vec<Ed25519AuthorityId> {
			Consensus::authorities()
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block)
		}

		fn initialise_block(header: &<Block as BlockT>::Header) {
			Executive::initialise_block(header)
		}
	}

	impl runtime_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			Runtime::metadata().into()
		}
	}

	impl block_builder_api::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalise_block() -> <Block as BlockT>::Header {
			Executive::finalise_block()
		}

		fn inherent_extrinsics(data: InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(block: Block, data: InherentData) -> CheckInherentsResult {
			data.check_extrinsics(&block)
		}

		fn random_seed() -> <Block as BlockT>::Hash {
			System::random_seed()
		}
	}

	impl runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(tx: <Block as BlockT>::Extrinsic) -> TransactionValidity {
			Executive::validate_transaction(tx)
		}
	}

	impl consensus_aura::AuraApi<Block> for Runtime {
		fn slot_duration() -> u64 {
			Aura::slot_duration()
		}
	}
}
```



## Interacting with Substrate 

Go to settings on [polkadot explorer](https://polkadot.js.org/apps/#/settings) and configure it to face local node

Then go to Extrinsic menu and interact with it.
![menu for extrinsic](https://i.imgur.com/pdPSIvd.png)



### Reference

For other tutorials with ERC721 on substrate, check out the tutorial from Parity team.
[Shawn Tabrizi's substratekitties workshop](https://shawntabrizi.github.io/substrate-collectables-workshop/#/1/creating-a-module)
