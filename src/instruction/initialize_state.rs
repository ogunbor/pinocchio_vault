use pinocchio::{
    ProgramResult, account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey,
};

use crate::{
    error::MyProgramError,
    state::{
        MyState,
        utils::{DataLen, load_ix_data},
    },
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InitializeMyStateIxData {
    pub owner: Pubkey,
    pub data: [u8; 32],
}

impl DataLen for InitializeMyStateIxData {
    const LEN: usize = core::mem::size_of::<InitializeMyStateIxData>(); // 32 bytes for Pubkey + 32 bytes for data
}

pub fn process_initilaize_state(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [payer_acc, state_acc, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !payer_acc.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if !state_acc.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    let ix_data = load_ix_data::<InitializeMyStateIxData>(data)?;

    if ix_data.owner.ne(payer_acc.key()) {
        return Err(MyProgramError::InvalidOwner.into());
    }

    MyState::initialize(state_acc, ix_data)?;

    Ok(())
}
