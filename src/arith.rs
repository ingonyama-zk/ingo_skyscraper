use crate::constants::*;


// adds
pub fn wrapping_add(x: [u64; 4], y: [u64; 4]) -> [u64; 4] {
    let x_u128 = unsafe { std::mem::transmute::<[u64; 4], [u128; 2]>(x) };
    let y_u128 = unsafe { std::mem::transmute::<[u64; 4], [u128; 2]>(y) };
    let (lo, c) = x_u128[0].overflowing_add(y_u128[0]);
    let (hi, _) = x_u128[1].carrying_add(y_u128[1], c);
    unsafe { std::mem::transmute::<[u128; 2], [u64; 4]>([lo, hi]) }
}

pub fn overflowing_add(x: [u64; 4], y: [u64; 4]) -> ([u64; 4], bool) {
    let x_u128 = unsafe { std::mem::transmute::<[u64; 4], [u128; 2]>(x) };
    let y_u128 = unsafe { std::mem::transmute::<[u64; 4], [u128; 2]>(y) };
    let (lo, c) = x_u128[0].overflowing_add(y_u128[0]);
    let (hi, c) = x_u128[1].carrying_add(y_u128[1], c);
    (unsafe { std::mem::transmute::<[u128; 2], [u64; 4]>([lo, hi]) }, c)
}

// subs
pub fn wrapping_sub(x: [u64; 4], y: [u64; 4]) -> [u64; 4] {
    let x_u128 = unsafe { std::mem::transmute::<[u64; 4], [u128; 2]>(x) };
    let y_u128 = unsafe { std::mem::transmute::<[u64; 4], [u128; 2]>(y) };
    let (lo, b) = x_u128[0].overflowing_sub(y_u128[0]);
    let (hi, _) = x_u128[1].borrowing_sub(y_u128[1], b);
    unsafe { std::mem::transmute::<[u128; 2], [u64; 4]>([lo, hi]) }
}

pub fn overflowing_sub(x: [u64; 4], y: [u64; 4]) -> ([u64; 4], bool) {
    let x_u128 = unsafe { std::mem::transmute::<[u64; 4], [u128; 2]>(x) };
    let y_u128 = unsafe { std::mem::transmute::<[u64; 4], [u128; 2]>(y) };
    let (lo, b) = x_u128[0].overflowing_sub(y_u128[0]);
    let (hi, b) = x_u128[1].borrowing_sub(y_u128[1], b);
    (unsafe { std::mem::transmute::<[u128; 2], [u64; 4]>([lo, hi]) }, b)
}

// x < 2p
pub fn reduce_1p(x: [u64; 4]) -> [u64; 4] {
    let (xr, c) = overflowing_sub(x, U64_P);
    if c {
        return x;
    } else {
        return xr;
    }
}

// x < 3p
pub fn reduce_2p(x: [u64; 4]) -> [u64; 4] {
    let msb0 = (x[3] >> 63) != 0;
    let msb1 = ((x[3] << 1) >> 63) != 0;
    if msb0 {
        return wrapping_sub(x, U64_2P);
    } else if msb1 {
        return reduce_1p(wrapping_sub(x, U64_P));
    } else {
        return reduce_1p(x);
    }
}

// x < 4p
pub fn reduce_3p(x: [u64; 4]) -> [u64; 4] {
    let msb0 = (x[3] >> 63) != 0;
    let msb1 = ((x[3] << 1) >> 63) != 0;
    if msb0 {
        return reduce_1p(wrapping_sub(x, U64_2P));
    } else if msb1 {
        return reduce_1p(wrapping_sub(x, U64_P));
    } else {
        return reduce_1p(x);
    }
}

// x < 5p
pub fn reduce_4p(x: [u64; 4]) -> [u64; 4] {
    let msb = (x[3] >> 62) as u8;
    if msb == 0 {
        return reduce_1p(x);
    } else {
        let r = if msb == 1 {U64_P} else if msb == 2 {U64_2P} else {U64_3P};
        return reduce_1p(wrapping_sub(x, r));
    }
}

// any x
pub fn reduce_5p(x: [u64; 4]) -> [u64; 4] {
    let msb0 = (x[3] >> 62) as u8;
    let msb1 = ((x[3] << 2) >> 63) != 0;
    if msb0 == 0 {
        return reduce_1p(x);
    } else {
        let r = if msb0 == 1 {U64_P} else if msb0 == 2 {U64_2P} else if !msb1 {U64_3P} else {U64_4P};
        return reduce_1p(wrapping_sub(x, r));
    }
}
