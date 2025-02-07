#![feature(bigint_helper_methods)]
pub use primitive_types::*;

mod constants;

mod arith;
pub use arith::*;

mod sqr_cios_opt;
pub use sqr_cios_opt::*;

mod sqr_cios_opt_unr_1;
pub use sqr_cios_opt_unr_1::*;

mod sqr_cios_opt_unr_2;
pub use sqr_cios_opt_unr_2::*;

mod sqr_cios_opt_unr_3;
pub use sqr_cios_opt_unr_3::*;

mod mul_cios_opt;
pub use mul_cios_opt::*;

mod mul_cios_opt_unr_1;
pub use mul_cios_opt_unr_1::*;

mod sqr_cios_ord_unr;
pub use sqr_cios_ord_unr::*;

mod bar;
pub use bar::*;

mod compress;
pub use compress::*;


#[cfg(test)]
mod tests {
    use constants::*;
    use crate::*;

    const U512_P: U512 = unsafe { std::mem::transmute::<[u64; 8], U512>([U64_P[0], U64_P[1], U64_P[2], U64_P[3], 0u64, 0u64, 0u64, 0u64]) };
    const U256_P: U256 = unsafe { std::mem::transmute::<[u64; 4], U256>(U64_P) };
    const U256_INVR: U256 = unsafe { std::mem::transmute::<[u128; 2], U256>(U128_INVR) };

    fn to_u256(x: [u64; 4]) -> U256 {
        unsafe { std::mem::transmute::<[u64; 4], U256>(x) }
    }

    fn from_u256(x: U256) -> [u64; 4] {
        unsafe { std::mem::transmute::<U256, [u64; 4]>(x) }
    }

    fn calc_mul_ref(x: [u64; 4], y: [u64; 4]) -> [u64; 4] {
        let x_u256 = unsafe { std::mem::transmute::<[u64; 4], U256>(x) } % (U256_P << 2);
        let y_u256 = unsafe { std::mem::transmute::<[u64; 4], U256>(y) } % (U256_P << 2);
        let tmp = unsafe { std::mem::transmute::<U512, [u64; 8]>(x_u256.full_mul(y_u256) % U512_P) };
        let tmp = unsafe { std::mem::transmute::<[u64; 4], U256>([tmp[0], tmp[1], tmp[2], tmp[3]]) };
        let tmp = unsafe { std::mem::transmute::<U512, [u64; 8]>(tmp.full_mul(U256_INVR) % U512_P) };
        [tmp[0], tmp[1], tmp[2], tmp[3]]
    }

    fn calc_sqr_ref(x: [u64; 4]) -> [u64; 4] {
        let x_u256 = unsafe { std::mem::transmute::<[u64; 4], U256>(x) } % (U256_P << 2);
        let tmp = unsafe { std::mem::transmute::<U512, [u64; 8]>(x_u256.full_mul(x_u256) % U512_P) };
        let tmp = unsafe { std::mem::transmute::<[u64; 4], U256>([tmp[0], tmp[1], tmp[2], tmp[3]]) };
        let tmp = unsafe { std::mem::transmute::<U512, [u64; 8]>(tmp.full_mul(U256_INVR) % U512_P) };
        [tmp[0], tmp[1], tmp[2], tmp[3]]
    }

    fn mod_p(x: [u64; 4]) -> [u64; 4] {
        let tmp = unsafe { std::mem::transmute::<[u64; 4], U256>(x) };  
        unsafe { std::mem::transmute::<U256, [u64; 4]>(tmp % U256_P) }
    }
    
    fn mod_2p(x: [u64; 4]) -> [u64; 4] {
        let tmp = unsafe { std::mem::transmute::<[u64; 4], U256>(x) };  
        unsafe { std::mem::transmute::<U256, [u64; 4]>(tmp % (U256_P << 1)) }
    }
    
    fn mod_3p(x: [u64; 4]) -> [u64; 4] {
        let tmp = unsafe { std::mem::transmute::<[u64; 4], U256>(x) };  
        unsafe { std::mem::transmute::<U256, [u64; 4]>(tmp % ((U256_P << 1) + U256_P)) }
    }
    
    fn mod_4p(x: [u64; 4]) -> [u64; 4] {
        let tmp = unsafe { std::mem::transmute::<[u64; 4], U256>(x) };  
        unsafe { std::mem::transmute::<U256, [u64; 4]>(tmp % (U256_P << 2)) }
    }
    
    fn mod_5p(x: [u64; 4]) -> [u64; 4] {
        let tmp = unsafe { std::mem::transmute::<[u64; 4], U256>(x) };  
        unsafe { std::mem::transmute::<U256, [u64; 4]>(tmp % ((U256_P << 2) + U256_P)) }
    }
    

    #[test]
    fn test_mul() {
        let mul = |x, y| -> [u64; 4] { mul_cios_opt(x, y) };
        // zero
        assert_eq!([0u64; 4], mul([0u64; 4], [0u64; 4]));
        // +3p rand -> +4p
        for _ in 0..1000000 {
            let x: [u64; 4] = mod_4p([rand::random::<u64>(), rand::random::<u64>(), rand::random::<u64>(), rand::random::<u64>()]);
            let y: [u64; 4] = mod_4p([rand::random::<u64>(), rand::random::<u64>(), rand::random::<u64>(), rand::random::<u64>()]);
            let ref_ = calc_mul_ref(x, y);
            let tmp = mul(x, y);
            assert_eq!(tmp, mod_5p(tmp));
            let res_ = mod_p(tmp);
            assert_eq!(ref_, res_);
        }
        // +2p rand -> +2p
        for _ in 0..1000000 {
            let x: [u64; 4] = mod_3p([rand::random::<u64>(), rand::random::<u64>(), rand::random::<u64>(), rand::random::<u64>()]);
            let y: [u64; 4] = mod_3p([rand::random::<u64>(), rand::random::<u64>(), rand::random::<u64>(), rand::random::<u64>()]);
            let ref_ = calc_mul_ref(x, y);
            let tmp = mul(x, y);
            assert_eq!(tmp, mod_3p(tmp));
            let res_ = mod_p(tmp);
            assert_eq!(ref_, res_);
        }
        // +1p rand -> +1p
        for _ in 0..1000000 {
            let x: [u64; 4] = mod_2p([rand::random::<u64>(), rand::random::<u64>(), rand::random::<u64>(), rand::random::<u64>()]);
            let y: [u64; 4] = mod_2p([rand::random::<u64>(), rand::random::<u64>(), rand::random::<u64>(), rand::random::<u64>()]);
            let ref_ = calc_mul_ref(x, y);
            let tmp = mul(x, y);
            assert_eq!(tmp, mod_2p(tmp));
            let res_ = mod_p(tmp);
            assert_eq!(ref_, res_);
        }
    }

    #[test]
    fn test_sqr() {
        let sqr = |x| -> [u64; 4] { sqr_cios_opt_unr_2(x) };
        // zero
        assert_eq!([0u64; 4], sqr([0u64; 4]));
        // +3p rand -> +4p
        for _ in 0..1000000 {
            let x: [u64; 4] = mod_4p([rand::random::<u64>(), rand::random::<u64>(), rand::random::<u64>(), rand::random::<u64>()]);
            let ref_ = calc_sqr_ref(x);
            let tmp = sqr(x);
            assert_eq!(tmp, mod_5p(tmp));
            let res_ = mod_p(tmp);
            assert_eq!(ref_, res_);
        }
        // +2p rand -> +2p
        for _ in 0..1000000 {
            let x: [u64; 4] = mod_3p([rand::random::<u64>(), rand::random::<u64>(), rand::random::<u64>(), rand::random::<u64>()]);
            let ref_ = calc_sqr_ref(x);
            let tmp = sqr(x);
            assert_eq!(tmp, mod_3p(tmp));
            let res_ = mod_p(tmp);
            assert_eq!(ref_, res_);
        }
        // +1p rand -> +1p
        for _ in 0..1000000 {
            let x: [u64; 4] = mod_2p([rand::random::<u64>(), rand::random::<u64>(), rand::random::<u64>(), rand::random::<u64>()]);
            let ref_ = calc_sqr_ref(x);
            let tmp = sqr(x);
            assert_eq!(tmp, mod_2p(tmp));
            let res_ = mod_p(tmp);
            assert_eq!(ref_, res_);
        }
        // +2p into a daisy chain of sqr
        let mut x = U64_3P;
        x[0] = 0u64;
        for _ in 0..100000 {
            let ref_ = calc_sqr_ref(x);
            x = sqr(x);
            assert_eq!(x, mod_2p(x));
            let res_ = mod_p(x);
            assert_eq!(ref_, res_);
        }
        // +3p into a daisy chain of sqr
        let mut x = U64_4P;
        x[0] = rand::random::<u64>();
        for _ in 0..100000 {
            let ref_ = calc_sqr_ref(x);
            x = sqr(x);
            assert_eq!(x, mod_4p(x));
            let res_ = mod_p(x);
            assert_eq!(ref_, res_);
        }
    }

    #[test]
    fn test_reduce() {
        for _ in 0..100000 {
            let x: [u64; 4] = mod_4p([rand::random::<u64>(), rand::random::<u64>(), rand::random::<u64>(), rand::random::<u64>()]);
            assert_eq!(mod_p(x), reduce_3p(x));
        }
    }

    #[test]
    fn test_addsub() {
        for _ in 0..100000 {
            let x: [u64; 4] = [rand::random::<u64>(), rand::random::<u64>(), rand::random::<u64>(), rand::random::<u64>()];
            let y: [u64; 4] = [rand::random::<u64>(), rand::random::<u64>(), rand::random::<u64>(), rand::random::<u64>()];
            let tmp = to_u256(x).overflowing_add(to_u256(y));
            assert_eq!((from_u256(tmp.0), tmp.1), overflowing_add(x, y));
            let tmp = to_u256(x).overflowing_sub(to_u256(y));
            assert_eq!((from_u256(tmp.0), tmp.1), overflowing_sub(x, y));
        }
    }

    #[test]
    fn test_rc_consts() {
        for i in 0..8 {
            let x = RC[i];
            let y = _1P_MINUS_RC[i];
            assert_eq!(U64_P, wrapping_add(x, y));
        }
    }

    #[test]
    fn test_bar() {
        assert_eq!(reduce_5p(bar_u8([0u64;4])), [0u64;4]);
        assert_eq!(reduce_5p(bar_u8([1u64, 0u64, 0u64, 0u64])), [0u64, 0u64, 2u64, 0u64]);
        assert_eq!(reduce_5p(bar_u8([2u64, 0u64, 0u64, 0u64])), [0u64, 0u64, 4u64, 0u64]);
        assert_eq!(reduce_5p(bar_u8([2u64, 0u64, 0u64, 0u64])), [0u64, 0u64, 4u64, 0u64]);
        assert_eq!(reduce_5p(bar_u8([713806376701769424, 17414291447527608673, 5073048685038849813, 655013393976292318])), [11341249794547965734, 8160123590369554932, 3580046016084765488, 3005794368565783321]);
    }

    #[test]
    #[should_panic]
    fn test_bar_dynamics_should_fail() { // this test needs to always fail
        for _ in 0..100 {
            let x = mod_p([rand::random::<u64>(), rand::random::<u64>(), rand::random::<u64>(), rand::random::<u64>()]);
            let y = bar_u8(x);
            assert_eq!(y, mod_5p(y));    
        }
    }

    #[test]
    fn test_compress() {
        // correctness
        assert_eq!(compress([222647740394868259, 1954084163509096643, 7169380306955695398, 3443405857474191768],
                            [650100192727553127, 2847352847332889852, 4016598436723263545, 1563325641941659433]), 
                            [18095061023341165257, 7738479748118643198, 13857889271559191300, 570841294491851342]);
        // dynamics
        let x = [0u64; 4];
        let tmp = compress(x, x);
        for _ in 0..1000000 {
            let x: [u64; 4] = mod_p([rand::random::<u64>(), rand::random::<u64>(), rand::random::<u64>(), rand::random::<u64>()]);
            let y: [u64; 4] = mod_p([rand::random::<u64>(), rand::random::<u64>(), rand::random::<u64>(), rand::random::<u64>()]);
            // println!("{:?}", (x,y));
            let tmp = compress(x, y);
        }
    }
}