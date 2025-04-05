use pinocchio::program_error::ProgramError;

pub mod initialize_state;
pub mod update_state;

pub use initialize_state::*;
pub use update_state::*;

#[repr(u8)]
pub enum MyProgramInstrution {
    InitializeState,
    UpdateState,
}

impl TryFrom<&u8> for MyProgramInstrution {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(MyProgramInstrution::InitializeState),
            1 => Ok(MyProgramInstrution::UpdateState),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
