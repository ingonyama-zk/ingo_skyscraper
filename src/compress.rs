use crate::constants::*;
use crate::arith::*;
use crate::sqr_cios_opt_unr_1::*;
use crate::sqr_cios_opt_unr_2::*;
use crate::bar::*;


pub fn compress(l: [u64; 4], r: [u64; 4]) -> [u64; 4] {
    let sqr_fun = |x| -> [u64; 4] { sqr_cios_opt_unr_2(x) };
    let a = l;
    debug_assert!(overflowing_sub(l, U64_P).1); // assert \in[0P]
    debug_assert!(overflowing_sub(r, U64_P).1); // assert \in[0P]
    // stage 0
    let sqr = sqr_fun(l);
    debug_assert!(overflowing_sub(sqr, U64_2P).1); // assert \in[1P]
    let (l, r) = (wrapping_add(r, sqr), l);
    debug_assert!(overflowing_sub(l, U64_3P).1); // assert \in[2P]
    debug_assert!(overflowing_sub(r, U64_P).1);  // assert \in[0P]
    // stage 1
    let sqr = sqr_fun(l);
    debug_assert!(overflowing_sub(sqr, U64_2P).1); // assert \in[1P] - SHOULD BE [2P]?
    let (l, r) = (x0p_plus_sqr1p_plus_rc_eq0p(r, sqr, 0), l);
    debug_assert!(overflowing_sub(l, U64_P).1);  // assert \in[0P]
    debug_assert!(overflowing_sub(r, U64_3P).1); // assert \in[2P]
    // stage 2
    let bar = bar_u8(l);
    let (l, r) = (x2p_plus_bar0p_plus_rc_eq0p(r, bar, 1), l);
    debug_assert!(overflowing_sub(l, U64_P).1); // assert \in[0P]
    debug_assert!(overflowing_sub(r, U64_P).1); // assert \in[0P]
    // stage 3
    let bar = bar_u8(l);   
    let (l, r) = (x0p_plus_bar0p_plus_rc_eq0p(r, bar, 2), l);
    debug_assert!(overflowing_sub(l, U64_P).1); // assert \in[0P]
    debug_assert!(overflowing_sub(r, U64_P).1); // assert \in[0P]
    // stage 4
    let sqr = sqr_fun(l);
    debug_assert!(overflowing_sub(sqr, U64_2P).1); // assert \in[1P]
    let (l, r) = (x0p_plus_sqr1p_plus_rc_eq0p(r, sqr, 3), l);
    debug_assert!(overflowing_sub(l, U64_P).1); // assert \in[0P]
    debug_assert!(overflowing_sub(r, U64_P).1); // assert \in[0P]

    // stage 5
    let sqr = sqr_fun(l);
    debug_assert!(overflowing_sub(sqr, U64_2P).1); // assert \in[1P]
    let (l, r) = (x0p_plus_sqr1p_plus_rc_eq0p(r, sqr, 4), l);
    debug_assert!(overflowing_sub(l, U64_P).1); // assert \in[0P]
    debug_assert!(overflowing_sub(r, U64_P).1); // assert \in[0P]
    // stage 6
    let bar = bar_u8(l);   
    let (l, r) = (x0p_plus_bar0p_plus_rc_eq0p(r, bar, 5), l);
    debug_assert!(overflowing_sub(l, U64_P).1); // assert \in[0P]
    debug_assert!(overflowing_sub(r, U64_P).1); // assert \in[0P]
    // stage 7
    let bar = bar_u8(l);   
    let (l, r) = (x0p_plus_bar0p_plus_rc_eq0p(r, bar, 6), l);
    debug_assert!(overflowing_sub(l, U64_P).1); // assert \in[0P]
    debug_assert!(overflowing_sub(r, U64_P).1); // assert \in[0P]
    // stage 8
    let sqr = sqr_fun(l);
    debug_assert!(overflowing_sub(sqr, U64_2P).1); // assert \in[1P]
    let (l, r) = (x0p_plus_sqr1p_plus_rc_eq0p(r, sqr, 7), l);
    debug_assert!(overflowing_sub(l, U64_P).1); // assert \in[0P]
    debug_assert!(overflowing_sub(r, U64_P).1); // assert \in[0P]
    // stage 9
    let sqr = sqr_fun(l);
    debug_assert!(overflowing_sub(sqr, U64_2P).1); // assert \in[1P]
    x0p_plus_sqr1p_plus_y0p_eq0p(r, sqr, a)
}

// x + sqr + y
pub fn x0p_plus_sqr1p_plus_y0p_eq0p(x: [u64; 4], sqr: [u64; 4], y: [u64; 4]) -> [u64; 4] {
    debug_assert!(overflowing_sub(x, U64_P).1);    // assert \in[0P]
    debug_assert!(overflowing_sub(sqr, U64_2P).1); // assert \in[1P]
    debug_assert!(overflowing_sub(y, U64_P).1);    // assert \in[0P]
    let x_plus_sqr = wrapping_add(x, sqr);   // x+sqr \in[2P]
    debug_assert!(overflowing_sub(x_plus_sqr, U64_3P).1); // assert \in[2P]
    let tmp = wrapping_add(x_plus_sqr, y); // x+sqr+y \in [3P]
    debug_assert!(overflowing_sub(tmp, U64_4P).1);   // assert \in[3P]
    // reduce to [0P]
    return reduce_3p(tmp);
}

// x + sqr + rc
pub fn x0p_plus_sqr1p_plus_rc_eq0p(x: [u64; 4], sqr: [u64; 4], rc_idx: usize) -> [u64; 4] {
    debug_assert!(overflowing_sub(x, U64_P).1);    // assert \in[0P]
    debug_assert!(overflowing_sub(sqr, U64_2P).1); // assert \in[1P]
    let x_plus_sqr = wrapping_add(x, sqr);   // x+sqr \in[2P]
    debug_assert!(overflowing_sub(x_plus_sqr, U64_3P).1); // assert \in[2P]
    let (tmp, b) = overflowing_sub(x_plus_sqr, _1P_MINUS_RC[rc_idx]); // x+sqr+rc \in [2P], if not underflow
    if b { // underflow
        // guaranteed to be [0P]
        return wrapping_add(x_plus_sqr, RC[rc_idx]);
    } else { // [2P]
        debug_assert!(overflowing_sub(tmp, U64_3P).1); // assert \in[2P]
        // reduce to [0P]
        return reduce_2p(tmp);
    }
}

// x + bar + rc
pub fn x2p_plus_bar0p_plus_rc_eq0p(x: [u64; 4], bar: [u64; 4], rc_idx: usize) -> [u64; 4] {
    debug_assert!(overflowing_sub(x, U64_3P).1); // assert \in[2P]
    // assume bar\in[5P]
    let msb0 = (bar[3] >> 62) as u8;
    let msb1 = ((bar[3] << 2) >> 63) != 0;
    // bar+rc \in[2P]
    let bar_plus_rc;
    if msb0 == 0 {        // 00x
        bar_plus_rc = wrapping_add(bar, RC[rc_idx]);
    } else if msb0 == 1 { // 01x
        bar_plus_rc = wrapping_sub(bar, _1P_MINUS_RC[rc_idx]);
    } else if msb0 == 2 { // 10x
        bar_plus_rc = wrapping_sub(bar, _2P_MINUS_RC[rc_idx]);
    } else if !msb1 {     // 110
        bar_plus_rc = wrapping_sub(bar, _3P_MINUS_RC[rc_idx]);
    } else {              // 111
        bar_plus_rc = wrapping_sub(bar, _4P_MINUS_RC[rc_idx]);
    }
    debug_assert!(overflowing_sub(bar_plus_rc, U64_3P).1); // assert \in[2P]
    // bar+rc+x \in[4P]
    let tmp = wrapping_add(bar_plus_rc, x);
    debug_assert!(overflowing_sub(tmp, U64_5P).1); // assert \in[4P]
    // reduce to [0P]
    reduce_4p(tmp)
}

pub fn x0p_plus_bar0p_plus_rc_eq0p(x: [u64; 4], bar: [u64; 4], rc_idx: usize) -> [u64; 4] {
    debug_assert!(overflowing_sub(x, U64_P).1); // assert \in[0P]
    // assume bar\in[5P]
    let msb0 = (bar[3] >> 62) as u8;
    let msb1 = ((bar[3] << 2) >> 63) != 0;
    // bar+rc \in[2P]
    let bar_plus_rc;
    if msb0 == 0 {        // 00x
        bar_plus_rc = wrapping_add(bar, RC[rc_idx]);
    } else if msb0 == 1 { // 01x
        bar_plus_rc = wrapping_sub(bar, _1P_MINUS_RC[rc_idx]);
    } else if msb0 == 2 { // 10x
        bar_plus_rc = wrapping_sub(bar, _2P_MINUS_RC[rc_idx]);
    } else if !msb1 {     // 110
        bar_plus_rc = wrapping_sub(bar, _3P_MINUS_RC[rc_idx]);
    } else {              // 111
        bar_plus_rc = wrapping_sub(bar, _4P_MINUS_RC[rc_idx]);
    }
    debug_assert!(overflowing_sub(bar_plus_rc, U64_3P).1); // assert \in[2P]
    // bar+rc+x \in[4P]
    let tmp = wrapping_add(bar_plus_rc, x);
    debug_assert!(overflowing_sub(tmp, U64_4P).1); // assert \in[3P]
    // reduce to [0P]
    reduce_3p(tmp)
}

pub fn x0p_plus_bar0p_plus_rc_eq2p(x: [u64; 4], bar: [u64; 4], rc_idx: usize) -> [u64; 4] {
    debug_assert!(overflowing_sub(x, U64_P).1); // assert \in[0P]
    // assume bar\in[5P]
    let msb0 = (bar[3] >> 62) as u8;
    let msb1 = ((bar[3] << 2) >> 63) != 0;
    // bar+rc \in[2P]
    let bar_plus_rc;
    if msb0 == 0 {        // 00x
        bar_plus_rc = wrapping_add(bar, RC[rc_idx]);
    } else if msb0 == 1 { // 01x
        bar_plus_rc = wrapping_sub(bar, _1P_MINUS_RC[rc_idx]);
    } else if msb0 == 2 { // 10x
        bar_plus_rc = wrapping_sub(bar, _2P_MINUS_RC[rc_idx]);
    } else if !msb1 {     // 110
        bar_plus_rc = wrapping_sub(bar, _3P_MINUS_RC[rc_idx]);
    } else {              // 111
        bar_plus_rc = wrapping_sub(bar, _4P_MINUS_RC[rc_idx]);
    }
    debug_assert!(overflowing_sub(bar_plus_rc, U64_3P).1); // assert \in[2P]
    // bar+rc+x \in[4P]
    let mut tmp = wrapping_add(bar_plus_rc, x);
    debug_assert!(overflowing_sub(tmp, U64_4P).1); // assert \in[3P]
    // reduce to [2P]
    if (tmp[3] >> 63) != 0 {
        tmp = wrapping_sub(tmp, U64_2P);
    }
    tmp
}
