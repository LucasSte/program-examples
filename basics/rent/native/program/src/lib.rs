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


entrypoint!(entry);
pub fn entry(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8])
             -> ProgramResult {
    

    let mut x: [u8; 16] = [0; 16];
    msg!("x ptr: {:p}", &x);
    x.copy_from_slice(&data[..16]);
    x[11] = 0x37;
    x[10] = 0x39;

    msg!("{:?}", x);

    let buffer: [u8; 3916] = [1; 3916];

    lol(x);

    msg!("{:p} (should start with 0x2000..)", &buffer);
    msg!("{:?} (should be 1)", buffer[0]);
    assert_eq!(buffer[0], 1);
    assert_eq!(x[11], 0x37);
    assert_eq!(x[10], 0x39);

    msg!("{:?}", x);

    // //msg!("b: {:?}", buffer);
    for i in 0..3916 {
        assert_eq!(buffer[i], 1);
    }
    Ok(())
}

#[inline(never)]
extern fn lol(x: [u8; 16]) {
    for i in 0..1 {
        assert!(x[i] != 1 as u8);
    }
    msg!("{:?}", x);
    assert_eq!(x[11], 0x37);
    assert_eq!(x[10], 0x39);
}
