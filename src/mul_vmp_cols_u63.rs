use crate::constants::*;


#[inline]
pub fn mul_vmp_cols_u63(_a: [u64; 4], _b: [u64; 4]) -> [u64; 4] {
    const N: usize = 5;
    let b63: u64 = 0x7fffffffffffffff;
    let b60: u64 = 0xfffffffffffffff;
    let b4: u64 = 0xf;
    let a = [_a[0] & b63, ((_a[1] << 1) | (_a[0] >> 63)) & b63, ((_a[2] << 2) | (_a[1] >> 62)) & b63, ((_a[3] << 3) | (_a[2] >> 61)) & b63, _a[3] >> 60];
    let b = [_b[0] & b63, ((_b[1] << 1) | (_b[0] >> 63)) & b63, ((_b[2] << 2) | (_b[1] >> 62)) & b63, ((_b[3] << 3) | (_b[2] >> 61)) & b63, _b[3] >> 60];
    let p = U63_P;
    let mu0 = U63_MU0;
    let mut t = [0u128; 4];
    let mut m = [0u64; 4];
    let mut temp: u128;
    // k=0
    t[0] = mul_add(a[0], b[0], t[0]);
    m[0] = mu0.wrapping_mul(t[0] as u64) & b63;
    t[0] = mul_add(m[0], p[0], t[0]);
    t[0] = t[0] >> 63;
    // k=1
    t[0] = mul_add(a[0], b[1], t[0]);
    t[0] = mul_add(a[1], b[0], t[0]);
    t[0] = mul_add(m[0], p[1], t[0]);
    m[1] = mu0.wrapping_mul(t[0] as u64) & b63;
    t[0] = mul_add(m[1], p[0], t[0]);
    t[0] = t[0] >> 63;
    // k=2
    t[0] = mul_add(a[0], b[2], t[0]);
    t[0] = mul_add(a[1], b[1], t[0]);
    t[0] = mul_add(a[2], b[0], t[0]);
    temp = (m[0] as u128).wrapping_mul(p[2] as u128);
    temp = mul_add(m[1], p[1], temp);
    m[2] = mu0.wrapping_mul((t[0] as u64).wrapping_add(temp as u64)) & b63;
    temp = mul_add(m[2], p[0], temp);
    t[0] = (t[0] >> 63).carrying_add(temp >> 63, ((temp as u64) & b63) > 0).0;
    // k=3
    t[0] = mul_add(a[0], b[3], t[0]);
    t[0] = mul_add(a[1], b[2], t[0]);
    t[0] = mul_add(a[2], b[1], t[0]);
    t[0] = mul_add(a[3], b[0], t[0]);
    temp = (m[0] as u128).wrapping_mul(p[3] as u128);
    temp = mul_add(m[1], p[2], temp);
    temp = mul_add(m[2], p[1], temp);
    m[3] = mu0.wrapping_mul((t[0] as u64).wrapping_add(temp as u64)) & b63;
    temp = mul_add(m[3], p[0], temp);
    t[0] = (t[0] >> 63).carrying_add(temp >> 63, ((temp as u64) & b63) > 0).0;
    // k=4
    t[0] = mul_add(a[1], b[3], t[0]);
    t[0] = mul_add(a[2], b[2], t[0]);
    t[0] = mul_add(a[3], b[1], t[0]);
    t[0] = mul_add(m[1], p[3], t[0]);
    t[0] = mul_add(m[2], p[2], t[0]);
    t[0] = mul_add(m[3], p[1], t[0]);
    // k=5
    t[1] = mul_add(a[2], b[3], t[1]);
    t[1] = mul_add(a[3], b[2], t[1]);
    t[1] = mul_add(m[2], p[3], t[1]);
    t[1] = mul_add(m[3], p[2], t[1]);
    // k=6
    t[2] = mul_add(a[3], b[3], t[2]);
    t[2] = mul_add(m[3], p[3], t[2]);

    // differed
    t[0] = t[0].wrapping_add(m[0] as u128).wrapping_add((m[0] << 1) as u128); // t[0] = mul_add(m[0], p[4], t[0]);
    t[1] = t[1].wrapping_add(m[1] as u128).wrapping_add((m[1] << 1) as u128); // t[1] = mul_add(m[1], p[4], t[1]);
    t[2] = t[2].wrapping_add(m[2] as u128).wrapping_add((m[2] << 1) as u128); // t[2] = mul_add(m[2], p[4], t[2]);
    t[3] = (m[3] as u128).wrapping_add((m[3] << 1) as u128); // t[3] = mul_add(m[3], p[4], t[3]);
    let mut ttt0: u128;
    let mut ttt1: u128;
    let c0: bool;
    let c1: bool;
    (ttt0, c0) = t[0].overflowing_add(t[1] << 63);
    (ttt0, c1) = ttt0.overflowing_add(t[2] << 126);
    let c = (c0 as u8).wrapping_add(c1 as u8); // todo check if this is ever 2?
    ttt1 = (t[1] >> 65).wrapping_add(t[2] >> 2).wrapping_add(t[3] << 61| c as u128);

    let bbb0 = (_b[1] as u128) << 64 | _b[0] as u128;
    let bbb1 = (_b[3] as u128) << 64 | _b[2] as u128;
    if a[4] & 1 != 0 {
        let c0: bool;
        (ttt0, c0) = ttt0.overflowing_add(bbb0);
        ttt1 = ttt1.carrying_add(bbb1, c0).0;
    }
    if a[4] & 2 != 0 {
        let c0: bool;
        (ttt0, c0) = ttt0.overflowing_add(bbb0 << 1);
        ttt1 = ttt1.carrying_add((bbb1 << 1) | (bbb0 >> 127), c0).0;
    }

    let aaa0 = (_a[1] as u128) << 64 | _a[0] as u128;
    let aaa1 = ((_a[3] & b60) as u128) << 64 | _a[2] as u128;
    if b[4] & 1 != 0 {
        let c0: bool;
        (ttt0, c0) = ttt0.overflowing_add(aaa0);
        ttt1 = ttt1.carrying_add(aaa1, c0).0;
    }
    if b[4] & 2 != 0 {
        let c0: bool;
        (ttt0, c0) = ttt0.overflowing_add(aaa0 << 1);
        ttt1 = ttt1.carrying_add((aaa1 << 1) | (aaa0 >> 127), c0).0;
    }

    let tttt = ttt0 as u8;
    let m4 = tttt.wrapping_add(tttt << 1).wrapping_add(tttt << 2).wrapping_add(tttt << 3); 
    let ppp0 = (U64_P[1] as u128) << 64 | U64_P[0] as u128;
    let ppp1 = (U64_P[3] as u128) << 64 | U64_P[2] as u128;
    if m4 & 1 != 0 {
        let c0: bool;
        (ttt0, c0) = ttt0.overflowing_add(ppp0);
        ttt1 = ttt1.carrying_add(ppp1, c0).0;
    }
    ttt0 = (ttt0 >> 1) | (ttt1 << 127);
    ttt1 = ttt1 >> 1;
    if m4 & 2 != 0 {
        let c0: bool;
        (ttt0, c0) = ttt0.overflowing_add(ppp0);
        ttt1 = ttt1.carrying_add(ppp1, c0).0;
    }
    ttt0 = (ttt0 >> 1) | (ttt1 << 127);
    ttt1 = ttt1 >> 1;
    if m4 & 4 != 0 {
        let c0: bool;
        (ttt0, c0) = ttt0.overflowing_add(ppp0);
        ttt1 = ttt1.carrying_add(ppp1, c0).0;
    }
    ttt0 = (ttt0 >> 1) | (ttt1 << 127);
    ttt1 = ttt1 >> 1;
    if m4 & 8 != 0 {
        let c0: bool;
        (ttt0, c0) = ttt0.overflowing_add(ppp0);
        ttt1 = ttt1.carrying_add(ppp1, c0).0;
    }
    ttt0 = (ttt0 >> 1) | (ttt1 << 127);
    ttt1 = ttt1 >> 1;

    // return
    [ttt0 as u64, (ttt0 >> 64) as u64, ttt1 as u64, (ttt1 >> 64) as u64]
}

#[inline]
const fn mul_add(lhs: u64, rhs: u64, add: u128) -> u128 {
(lhs as u128).wrapping_mul(rhs as u128).wrapping_add(add)
}