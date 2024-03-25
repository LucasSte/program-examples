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

struct Testing {
    a: u128,
    b: u128
}

// TODO: This test must be included in programs/sbf
pub fn entry(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8])
             -> ProgramResult {
    

    let mut x: [u8; 16] = [0; 16];
    msg!("x ptr: {:p}", &x);
    x.copy_from_slice(&data[..16]);
    x[11] = 0x37;
    x[10] = 0x39;

    msg!("{:?}", x);

    let buffer: [u8; 3916] = [1; 3916];

    let y = lol(x);

    assert_eq!(y[15], 17);
    msg!("{:p} (should start with 0x2000..)", &buffer);
    msg!("{:?} (should be 1)", buffer[0]);
    assert_eq!(buffer[0], 1);
    assert_eq!(x[11], 0x37);
    assert_eq!(x[10], 0x39);

    msg!("{:?}", x);

    // Optimized away. Read from data.
    let x_i128 : u128 = u128::MAX - 2;
    let y_i128 = test_i128(x_i128);
    assert_eq!(y_i128, u128::MAX);
    
    // Optimized away. Read from data.
    let x_256 = Testing {a: u128::MAX - 2, b: u128::MAX - 3};
    let y_256 = modify_256(x_256);

    // These two functions are optimized away. To avoid that, 
    // Read the i64 from data.
    let res = many_args(51, -89, 85, -777, 500, 35, 100, 45);
    assert_eq!(res, -1420);
    
    // Optimized away. Read from data.
    let res = many_args_stack_space(51, -89, 85, -777, 500, 35, 100, 45);
    assert_eq!(res, -1420);

    Ok(())
}

#[inline(never)]
extern fn lol(mut x: [u8; 16]) -> [u8; 16] {
    for i in 0..1 {
        assert!(x[i] != 1 as u8);
    }
    msg!("{:?}", x);
    assert_eq!(x[11], 0x37);
    assert_eq!(x[10], 0x39);
    x[15] = 17;
    return x;
}

#[inline(never)]
extern fn test_i128(x: u128) -> u128 {
    let y = x + 2;
    return y;
}

#[inline(never)]
extern fn modify_256(x: Testing) -> Testing {
    let y = Testing {
        a: x.a + 2,
        b: x.b + 3
    };
    return y;
}

#[inline(never)]
// (51, -89, 85, -777, 500, 35, 100, 45);
extern fn many_args(a: i64, b: i64, c: i64, d: i64, e: i64, f: i64, g: i64, h: i64) -> i64 {
    let i = a + b; // 51 - 89 = -38
    let j = i - c; // -38 -85 = -123
    let k = j + d; // -123 - 777 = -900
    let l = k - e; // -900 - 500 = -1400
    let m = l + f; // -1400 + 35 = 
    let n = m - g;
    let o = n + h;
    return o;
}

#[inline(never)]
extern fn many_args_stack_space(a: i64, b: i64, c: i64, d: i64, e: i64, f: i64, g: i64, h: i64) -> i64 {
    let mut s: [i64; 3] = [1, 2, 3]; 
    let i = a + b; // 51 - 89 = -38
    let j = i - c; // -38 -85 = -123
    let k = j + d; // -123 - 777 = -900
    let l = k - e; // -900 - 500 = -1400
    let m = l + f; // -1400 + 35 = 
    let n = m - g;
    let o = n + h;
    let p = o + s[0];
    let q = p + s[1];
    let r = q - s[2];
    return r;
}


// Test more than 5 arguments, and argument whose size is greater than 128
