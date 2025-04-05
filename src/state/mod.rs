use pinocchio::{ProgramResult, account_info::AccountInfo, pubkey::Pubkey};
use utils::{DataLen, Initialized, load_acc_mut_unchecked};

use crate::instruction::{InitializeMyStateIxData, UpdateMyStateIxData};

pub mod utils;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
    Uninitialized,
    Initialized,
    Updated,
}

#[repr(C)] //keeps the struct layout the same across different architectures
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MyState {
    pub is_initialized: bool,
    pub owner: Pubkey,
    pub state: State,
    pub data: [u8; 32],
    pub update_count: u32,
}

impl DataLen for MyState {
    const LEN: usize = core::mem::size_of::<MyState>();
}

impl Initialized for MyState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl MyState {
    pub fn initialize(
        my_stata_acc: &AccountInfo,
        ix_data: &InitializeMyStateIxData,
    ) -> ProgramResult {
        let my_state =
            unsafe { load_acc_mut_unchecked::<MyState>(my_stata_acc.borrow_mut_data_unchecked()) }?;

        my_state.owner = ix_data.owner;
        my_state.state = State::Initialized;
        my_state.data = ix_data.data;
        my_state.update_count = 0;
        my_state.is_initialized = true;

        Ok(())
    }

    pub fn update(&mut self, ix_data: &UpdateMyStateIxData) -> ProgramResult {
        self.data = ix_data.data;
        self.update_count += 1;

        Ok(())
    }
}
