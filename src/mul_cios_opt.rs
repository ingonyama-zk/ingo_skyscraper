// rolled version of acar cios
use crate::constants::*;


#[inline]
pub fn mul_cios_opt(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
    let p = U64_P;
    let mu0 = U64_MU0;
    let mut res = [0u64; 4];
    let mut car1;
    let mut car2;

    for i in 0..4 {
        // case j = 0
        // (res[0], car2) = carrying_mul_add(a[0], a[i], res[0], 0);
        if i == 0 {
            (res[0], car2) = carrying_mul_add_slim(a[0], b[i], res[0]);
        } else {
            (res[0], car2) = carrying_mul_add(a[0], b[i], res[0], 0);
        }

        let m = res[0].wrapping_mul(mu0);

        (_, car1) = carrying_mul_add(m, p[0], res[0], 0);
        // car1 = carrying_mul_true(m, p[0], res[0] != 0); // doesn't work for edge cases like when a==0 or one of the digits is 2**32 etc.

        // case j = 1, ..., 3
        for j in 1..4 {
            let temp_mult = (a[j] as u128).wrapping_mul(b[i] as u128);
            
            let mut temp_res = 0u128;
            // temp_res = temp_mult.wrapping_add(res[j] as u128).wrapping_add(car2 as u128);
            if i == 0// || ((i == 4 - 1) && (j == 4 - 1)) // this only works for inputs < p
            {
                let temp_add = res[j].wrapping_add(car2);
                temp_res = temp_mult.wrapping_add(temp_add as u128);
            } else {
                let temp_add = (res[j] as u128).wrapping_add(car2 as u128);
                temp_res = temp_mult.wrapping_add(temp_add);
            }
            res[j] = temp_res as u64;
            car2 = (temp_res >> 64) as u64;

            (res[j - 1], car1) = carrying_mul_add(m, p[j], res[j], car1);
        }

        res[3] = car1.wrapping_add(car2);
    }
    res
}

#[inline]
const fn carrying_mul_add(lhs: u64, rhs: u64, add: u64, carry: u64) -> (u64, u64) {
    let wide = (lhs as u128)
        .wrapping_mul(rhs as u128)
        .wrapping_add(add as u128)
        .wrapping_add(carry as u128);
    (wide as u64, (wide >> 64) as u64)
}

#[inline]
const fn carrying_mul_add_slim(lhs: u64, rhs: u64, add: u64) -> (u64, u64) {
    let wide = (lhs as u128)
        .wrapping_mul(rhs as u128);
    let slim = (wide as u64).wrapping_add(add);
    (slim, (wide >> 64) as u64)
}

#[inline]
const fn carrying_mul_true(lhs: u64, rhs: u64, cond: bool) -> u64 {
    let wide = (lhs as u128).wrapping_mul(rhs as u128);
    if cond == true {
        return ((wide >> 64) as u64).wrapping_add(1);
    } else {
        return (wide >> 64) as u64;
    }
}