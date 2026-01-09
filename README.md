<p align="center">
 <img alt="pinocchio-token" src="logo.png" height="100"/>
</p>
<h3 align="center">
  <code>pinocchio-token</code>
</h3>
<p align="center">
  <strong>The First Wave of pTokens on Solana Mainnet</strong>
</p>
<p align="center">
  <a href="https://x.com/supertokenspl"><img src="https://img.shields.io/badge/X-@supertokenspl-blue?logo=x" /></a>
</p>

## Overview

This crate contains [`pinocchio`](https://crates.io/crates/pinocchio) helpers to perform cross-program invocations (CPIs) for SPL Token instructions.

Each instruction defines a `struct` with the accounts and parameters required. Once all values are set, you can call directly `invoke` or `invoke_signed` to perform the CPI.

This is a `no_std` crate.

**Mainnet Program ID:** `NkPE8JBLhrdvTBnktRBS8bMkZqgjtNvHXjAZ9nGhSPL`

> **Note:** The API defined in this crate is subject to change.

## Examples

Initializing a mint account:
```rust
// This example assumes that the instruction receives a writable `mint`
// account; `authority` is a `Address`.
InitializeMint {
    mint,
    rent_sysvar,
    decimals: 9,
    mint_authority: authority,
    freeze_authority: Some(authority),
}.invoke()?;
```

Performing a transfer of tokens:
```rust
// This example assumes that the instruction receives writable `from` and `to`
// accounts, and a signer `authority` account.
Transfer {
    from,
    to,
    authority,
    amount: 10,
}.invoke()?;
```

Minting tokens:
```rust
MintTo {
    mint,
    account,
    mint_authority,
    amount: 1_000_000,
}.invoke()?;
```

Burning tokens:
```rust
Burn {
    account,
    mint,
    authority,
    amount: 500_000,
}.invoke()?;
```

## Instructions

| Instruction | Description |
|-------------|-------------|
| `InitializeMint` | Initialize a new mint |
| `InitializeMint2` | Initialize mint without rent sysvar |
| `InitializeAccount` | Initialize a new token account |
| `InitializeAccount2` | Initialize account without rent sysvar |
| `InitializeAccount3` | Initialize account with owner |
| `InitializeMultisig` | Initialize a multisig account |
| `InitializeMultisig2` | Initialize multisig without rent sysvar |
| `Transfer` | Transfer tokens between accounts |
| `TransferChecked` | Transfer with decimal verification |
| `Approve` | Approve a delegate |
| `ApproveChecked` | Approve with decimal verification |
| `Revoke` | Revoke a delegate |
| `SetAuthority` | Set a new authority |
| `MintTo` | Mint new tokens |
| `MintToChecked` | Mint with decimal verification |
| `Burn` | Burn tokens |
| `BurnChecked` | Burn with decimal verification |
| `CloseAccount` | Close a token account |
| `FreezeAccount` | Freeze a token account |
| `ThawAccount` | Thaw a frozen account |
| `SyncNative` | Sync native SOL balance |

## Building

```bash
cargo build-sbf
```

## Links

* [Pinocchio Framework](https://github.com/anza-xyz/pinocchio)
* [Follow us on X](https://x.com/supertokenspl)

## License

The code is licensed under the [Apache License Version 2.0](LICENSE)
