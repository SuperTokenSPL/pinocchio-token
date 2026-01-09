<h1 align="center">
  <code>pinocchio-token</code>
</h1>
<p align="center">
  <img width="200" alt="pinocchio-token" src="logo.png" />
</p>
<p align="center">
  <strong>The First Wave of pTokens on Solana Mainnet</strong>
</p>

<p align="center">
  <a href="https://x.com/supertokenspl"><img src="https://img.shields.io/badge/X-@supertokenspl-blue?logo=x" /></a>
</p>

<p align="right">
<i>No bloat, no weight</i><br />
<i>Just bytes that flow</i><br />
<i>Through Solana's veins</i><br />
<i>Watch the pTokens grow</i><br />
<i>First wave is here</i><br />
<i>The future is clear</i><br />
<i>Pinocchio tokens have no fear</i>
</p>

## Overview

Pinocchio-token is the **first pToken deployed to Solana Mainnet** &mdash; a lightweight, zero-dependency program built with the blazing fast [pinocchio](https://github.com/anza-xyz/pinocchio) framework. This marks the beginning of a new era of hyper-optimized Solana programs.

**🚀 Mainnet Program ID:** `NkPE8JBLhrdvTBnktRBS8bMkZqgjtNvHXjAZ9nGhSPL`

## Why pTokens?

pTokens represent a new standard for Solana programs:

* **Zero external dependencies** &mdash; nothing to slow you down
* **Minimal compute units** &mdash; every CU counts
* **Tiny binary size** &mdash; lean and mean
* **`no_std` by default** &mdash; pure efficiency

This isn't just another token program. This is the future of Solana development.

## Features

* 🎯 Ultra-lightweight program entrypoint
* ⚡ Optimized for minimal CU consumption
* 🔗 Composable with any Solana program (Meteora, Jupiter, Raydium, etc.)
* 🛡️ Built on battle-tested pinocchio primitives

## Instructions

| Instruction | Discriminator | Description |
|-------------|---------------|-------------|
| `Ping` | `[0]` | Simple interaction &mdash; logs "Ping!" |
| `Interact` | `[1, ...data]` | Composable instruction for multi-program transactions |

## Getting Started

### Interacting with the Program

```typescript
import { TransactionInstruction, PublicKey } from "@solana/web3.js";

const PINOCCHIO_TOKEN = new PublicKey("NkPE8JBLhrdvTBnktRBS8bMkZqgjtNvHXjAZ9nGhSPL");

// Ping the program
const pingIx = new TransactionInstruction({
  programId: PINOCCHIO_TOKEN,
  keys: [{ pubkey: wallet.publicKey, isSigner: true, isWritable: false }],
  data: Buffer.from([0])
});

// Compose with other programs (e.g., Meteora DBC)
const interactIx = new TransactionInstruction({
  programId: PINOCCHIO_TOKEN,
  keys: [
    { pubkey: wallet.publicKey, isSigner: true, isWritable: false },
    // Add any additional accounts for context
  ],
  data: Buffer.from([1, /* your data */])
});

// Add to your transaction
transaction.add(meteoraInstruction, pingIx);
```

### Building from Source

```bash
cargo build-sbf
```

## The Vision

This is just the beginning. The first wave of pTokens on mainnet signals a shift toward:

* **Efficiency-first development** &mdash; no more bloated dependencies
* **Composability** &mdash; programs that play nice with the entire Solana ecosystem
* **Innovation** &mdash; pushing the boundaries of what's possible on-chain

Join us. Build with us. The wave is rising. 🌊

## Links

* [Pinocchio Framework](https://github.com/anza-xyz/pinocchio)
* [Follow us on X](https://x.com/supertokenspl)

## License

The code is licensed under the [Apache License Version 2.0](LICENSE)
