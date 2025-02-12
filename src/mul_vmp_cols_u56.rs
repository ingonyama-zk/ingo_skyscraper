use crate::constants::*;


#[inline]
pub fn mul_vmp_cols_u56(_a: [u64; 4], _b: [u64; 4]) -> [u64; 4] {
    const N: usize = 5;
    let b56 = 0xffffffffffffff;
    let a = [_a[0] & b56, ((_a[1] << 8) | (_a[0] >> 56)) & b56, ((_a[2] << 16) | (_a[1] >> 48)) & b56, ((_a[3] << 24) | (_a[2] >> 40)) & b56, _a[3] >> 32];
    let b = [_b[0] & b56, ((_b[1] << 8) | (_b[0] >> 56)) & b56, ((_b[2] << 16) | (_b[1] >> 48)) & b56, ((_b[3] << 24) | (_b[2] >> 40)) & b56, _b[3] >> 32];
    let p = U56_P;
    let mu0 = U56_MU0;
    let mu0_ = U32_MU0;
    let mut t = [0u128; N];
    let mut m = [0u64; N];
    // first N columns
    for k in 0..N {
        // accumulate
        t[0] = mul_add(a[k], b[0], t[0]);
        for i in 0..k {
            let j = k - i;
            t[0] = mul_add(a[i], b[j], t[0]);
            t[0] = mul_add(m[i], p[j], t[0]);    
        }
        // reduce
        if k != N - 1 {
            m[k] = mu0.wrapping_mul(t[0] as u64) & b56;
            t[0] = mul_add(m[k], p[0], t[0]);
            t[0] = t[0] >> 56;
        } else {
            m[k] = mu0_.wrapping_mul(t[0] as u32) as u64;
            t[0] = mul_add(m[k], p[0], t[0]);
        }
    }
    // last N-1 columns
    for k in 0..N-1 {
        // accumulate
        for i in k + 1..N {
            let j = N + k - i;
            t[k + 1] = mul_add(a[i], b[j], t[k + 1]);
            t[k + 1] = mul_add(m[i], p[j], t[k + 1]);
        }
    }
    // res0
    let (res0, c) = ((t[0] >> 32) as u64).overflowing_add((t[1] << 24) as u64);
    // res1
    let (temp, c) = ((t[0] >> 96) as u64).overflowing_add((t[2] << 16) as u64 | c as u64);
    debug_assert!(c == false);
    // ignore temp overflow (assume c==0)
    let (res1, c) = ((t[1] >> 40) as u64).overflowing_add(temp);
    // res2
    let (temp, c) = ((t[1] >> 104) as u64).overflowing_add((t[3] << 8) as u64 | c as u64);
    debug_assert!(c == false);
    // ignore temp overflow (assume c==0)
    let (res2, c) = ((t[2] >> 48) as u64).overflowing_add(temp);
    // res3
    let (temp, _) = ((t[2] >> 112) as u64).carrying_add(t[4] as u64, c);
    let res3 = ((t[3] >> 56) as u64).wrapping_add(temp);
    // return
    [res0, res1, res2, res3]
}

#[inline]
const fn mul_add(lhs: u64, rhs: u64, add: u128) -> u128 {
(lhs as u128).wrapping_mul(rhs as u128).wrapping_add(add)
}