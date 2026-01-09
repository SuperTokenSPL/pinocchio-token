//! Simple interaction program for Meteora DBC token launches
//!
//! This program provides a simple interaction point that can be composed
//! with other programs like Meteora DBC in the same transaction.

#![no_std]

use pinocchio::{AccountView, Address, ProgramResult};
use solana_program_error::ProgramError;
use solana_program_log::log;

pinocchio::program_entrypoint!(process_instruction);
pinocchio::default_allocator!();
pinocchio::nostd_panic_handler!();

pub fn process_instruction(
    _program_id: &Address,
    accounts: &[AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.is_empty() {
        return Err(ProgramError::InvalidInstructionData);
    }

    match instruction_data[0] {
        // Ping instruction
        0 => process_ping(accounts),
        // Interact instruction
        1 => process_interact(accounts, &instruction_data[1..]),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

/// Process Ping instruction - simple log
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

/// Process Interact instruction - composable with other programs
fn process_interact(accounts: &[AccountView], data: &[u8]) -> ProgramResult {
    if accounts.is_empty() {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let signer = &accounts[0];
    if !signer.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    log("Interact!");

    // Log data length if present
    if !data.is_empty() {
        log("Has data");
    }

    Ok(())
}
