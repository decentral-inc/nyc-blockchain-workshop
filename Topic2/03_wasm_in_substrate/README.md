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

goto file `tutorial.rs` and `lib.rs` and code as written there.


## Interacting with Substrate 

### Polkadot explorer

Polkadot network has prepared a tool to test your runtime functions on web browser.

Go to settings on [polkadot explorer](https://polkadot.js.org/apps/#/settings) and configure it to face local node

Then go to Extrinsic menu and interact with it.
![menu for extrinsic](https://i.imgur.com/pdPSIvd.png)

### Substrate UI

Substrate UI is where you build Custom UI to interact with Substate. It is built with React with Semactic UI css framework.

This can be used to make dapp on browser.

to make a new fronend app, simply run command

```bash
substrate-ui-new <substrate project>
cd <substrateproject>-ui
yarn install
```
To run the web app,
```bash
yarn run dev
```

then you will be able to see UI like this.
![substrate-ui](https://i.imgur.com/CZYdtzq.png)




### Reference

For other tutorials with ERC721 on substrate in detail, check out the tutorial from Parity team.
[Shawn Tabrizi's substratekitties workshop](https://shawntabrizi.github.io/substrate-collectables-workshop/#/1/creating-a-module)
