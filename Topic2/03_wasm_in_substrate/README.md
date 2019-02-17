# Topic 2-3. wasm in substrate

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

### Runtime vs. Smart contract

Smart contract is a ethereum byte opcodes stored in key-value database(e.g. BoltsDB for Eth2.0 by Prysmatic lab). Here is the execution process for smart contract.

1. User sends encrypted transaction with ethereum byte code data to smart contract account
2. Ethereum blockchain verifies tx and locates the contract address as key and look for "code" property where the ethereum opcode is stored in its embedded key-value storage. 
3. VM in ethereum node processes execution from the input from transaction to the stored ethereum byte code
4. result is applied to state and state root is added to the block.

On the other hand, Runtime in substrate is a built-in "state transition function"(STF), which directly changes the state of the blockchain. On ethereum there was one STF, `apply_transaction`. Polkadot network approach scalability problem with modular blockchain where each service providers separate their services with their own blockchain. As a result, it enables optimized state transition for a blockchain and connected same time, strengthening its integrity by joining mining power between blockchain protocols. Here is the execution process for runtime module.

1. User sends encrypted transaction to a Substrate node to its runtime endpoint
2. Substrate node verifies tx and processes built-in function and change state if the tx requests for change
3. State root is added to the block to record the change

### Creating a module

to start with, let us locate to `runtime/` in substrate folder.

```rust
use support::{decl_storage, decl_module};

pub trait Trait: system::Trait {}

decl_stroge! {
    trait Store for Module<T: Trait> as KittyStorage {
        // Declare storage and getter function here
    }
}

decl_module! {
    pub struc Module<T: Trait> for enum Call where origin: T::Origin {
        // Declare public functions here
    }
}
```


### Reference
[Shawn Tabrizi's substratekitties workshop](https://shawntabrizi.github.io/substrate-collectables-workshop/#/1/creating-a-module)