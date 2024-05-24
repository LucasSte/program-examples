use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    program::set_return_data,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // TODO: Build with my custom toolchain!
    let a = u64::from_be_bytes(instruction_data[0..8].try_into().unwrap());
    let b = u64::from_be_bytes(instruction_data[8..16].try_into().unwrap());

    let res = a % b;

    let data = res.to_be_bytes();
    set_return_data(&data);
    Ok(())
}
