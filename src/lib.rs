#![no_std]

#[cfg(all(not(feature = "no-entrypoint"), not(test)))]
mod entrypoint;

pub mod error;
pub mod instruction;
pub mod state;

#[cfg(test)]
pub mod tests;

pinocchio_pubkey::declare_id!("4ibrEMW5F6hKnkW4jVedswYv6H6VtwPN6ar6dvXDN1nT");
