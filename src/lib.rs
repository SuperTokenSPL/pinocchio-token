//! Pinocchio helpers to invoke Token program instructions.
//!
//! This crate contains [`pinocchio`](https://crates.io/crates/pinocchio) helpers
//! to perform cross-program invocations (CPIs) for SPL Token instructions.
//!
//! Each instruction defines a `struct` with the accounts and parameters required.
//! Once all values are set, you can call directly `invoke` or `invoke_signed` to
//! perform the CPI.
//!
//! This is a `no_std` crate.

#![no_std]

pub mod instructions;
pub mod state;

use core::mem::MaybeUninit;

use pinocchio::{AccountView, Address, ProgramResult};
use solana_program_error::ProgramError;
use solana_program_log::log;

pinocchio::program_entrypoint!(process_instruction);
pinocchio::default_allocator!();
pinocchio::nostd_panic_handler!();

solana_address::declare_id!("NkPE8JBLhrdvTBnktRBS8bMkZqgjtNvHXjAZ9nGhSPL");

const UNINIT_BYTE: MaybeUninit<u8> = MaybeUninit::<u8>::uninit();

#[inline(always)]
fn write_bytes(destination: &mut [MaybeUninit<u8>], source: &[u8]) {
    let len = destination.len().min(source.len());
    // SAFETY:
    // - Both pointers have alignment 1.
    // - For valid (non-UB) references, the borrow checker guarantees no overlap.
    // - `len` is bounded by both slice lengths.
    unsafe {
        core::ptr::copy_nonoverlapping(source.as_ptr(), destination.as_mut_ptr() as *mut u8, len);
    }
}

/// Program entrypoint
pub fn process_instruction(
    _program_id: &Address,
    accounts: &[AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.is_empty() {
        return Err(ProgramError::InvalidInstructionData);
    }

    match instruction_data[0] {
        0 => process_ping(accounts),
        1 => process_interact(accounts, &instruction_data[1..]),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

/// Process Ping instruction
fn process_ping(accounts: &[AccountView]) -> ProgramResult {
    if accounts.is_empty() {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let signer = &accounts[0];
    if !signer.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    log("Ping!");
    Ok(())
}

/// Process Interact instruction
fn process_interact(accounts: &[AccountView], data: &[u8]) -> ProgramResult {
    if accounts.is_empty() {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let signer = &accounts[0];
    if !signer.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    log("Interact!");

    if !data.is_empty() {
        log("Has data");
    }

    Ok(())
}
