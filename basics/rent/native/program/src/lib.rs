use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction, system_program,
    sysvar::Sysvar,
};

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    let n1 = u32::from_be_bytes(instruction_data[0..4].try_into().unwrap());
    let n2 = u32::from_be_bytes(instruction_data[4..8].try_into().unwrap());
    let v1 = n1 % 7777;
    let v2 = n2 % n1;
    msg!("n1: {}, n2: {}, v1: {}, v2: {}", n1, n2, v1, v2);

    Ok(())
}
