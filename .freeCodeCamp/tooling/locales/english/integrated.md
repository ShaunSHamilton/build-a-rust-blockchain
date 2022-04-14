# Build a Rust Blockchain

## 1

### --description--

You will build a Proof of Stake blockchain using Rust.

This project comes with a boilerplate including the following:

- `/blockchain` - the only directory you will be working in
- `/client` - the directory containing the code for the clientside code
- `/node` - the directory containing the server-side code managing a node in the network

**Useful Resources**

- [Rust Course](https://www.freecodecamp.org/news/rust-in-replit/)
- [Rust and WASM](https://rustwasm.github.io/docs/book/)

**Types**

```typescript
type Account = {
  address: string;
  staked: number;
  tokens: number;
}

type Block = {
  data: Account[],
  hash: string,
  id: number,
  nonce: number,
  previous_hash: string,
  timestamp: number,
  next_miner: Account['address'],
  next_validators: Account['address'][],
}

enum Event = {
  AddAccount
  Punish,
  Reward,
  Stake,
  Transfer,
  Unstake,
}

type NodeState = {
  chain: Block[];
  network: Account['address'][];
  transaction_pool: Transaction[];
}

type Transaction = {
  address: Account['address'];
  event: Event;
}
```

**User Stories:**

- Your blockchain uses Proof of Stake consensus
- Your blockchain uses `wasm-pack` to compile the Rust code to JavaScript for Nodejs
- Your blockchain exports an `initialise_chain` function that returns `Result<JsValue, JsError>`
  - This function accepts a `String` with the address of the initialising node
- Your blockchain exports a `mine_block` function that returns `Result<JsValue, JsError>`
  - This function accepts a `JsValue` with the `NodeState` type
- Your blockchain exports a `validate_block` function that returns `Result<bool, JsError>`
  - This function accepts a `JsValue` with the `Block[]` type
- Your blockchain passes all unit and integration tests

### --tests--

You should create a new Rust library named `blockchain`.

```js
await new Promise((resolve) => setTimeout(resolve, 1564));
assert(true);
```

Your `blockchain` library should pass all unit tests.

```js
// Execute `cargo test --lib`, and pipe output to tests client
```

Your `blockchain` library should pass all integration tests.

```js
// Execute `wasm-pack test --chrome`, and pipe output to tests client
```

## 2
