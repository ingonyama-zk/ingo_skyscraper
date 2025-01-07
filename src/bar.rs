use crate::constants::*;
use crate::arith::*;


pub fn bar_u8(x: [u64; 4]) -> [u64; 4] {
    let x_u8 = unsafe { std::mem::transmute::<[u64; 4], [u8; 32]>(x) };
    let sbox = |v: u8| (v ^ ((!v).rotate_left(1) & v.rotate_left(2) & v.rotate_left(3))).rotate_left(1);
    let y_u8 = x_u8.map(|b| sbox(b));
    let y = unsafe { std::mem::transmute::<[u8; 32], [u64; 4]>(y_u8) };
    [y[2], y[3], y[0], y[1]]
}
