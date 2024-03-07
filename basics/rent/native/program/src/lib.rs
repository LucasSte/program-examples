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
    // let sparkle_heart = [240, 159, 146, 150];
    // let result_str = std::str::from_utf8(&sparkle_heart).unwrap();
    // let pp = "💖";
    // let n = result_str.as_bytes();
    // let n2 = pp.as_bytes();
    //  if "💖" == result_str {
    //     let v = n[0] % n[1];
    //     msg!("equal: {}", v);
    // } else {
    //     let v = n[1] % n[2];
    //     msg!("Not equal: {}", v);
    // }

    let data = u32::from_ne_bytes(instruction_data[0..4].try_into().unwrap());
    if data == 0xfcaffee0u32 {
        let v = instruction_data[0] % instruction_data[1];
        msg!("equal: {}", v);
    } else {
        let v = instruction_data[1] % instruction_data[2];
        msg!("Not equal: {}", v);
    }

    Ok(())
}
