#![feature(bigint_helper_methods)]

mod constants;

mod sqr_cios_opt_unr_2;
pub use sqr_cios_opt_unr_2::sqr_cios_opt_unr_2;

mod sqr_cios_opt_unr_1;
pub use sqr_cios_opt_unr_1::sqr_cios_opt_unr_1;

mod sqr_cios_ord_unr;
pub use sqr_cios_ord_unr::sqr_cios_ord_unr;

mod mul_cios_opt_unr_1;
pub use mul_cios_opt_unr_1::mul_cios_opt_unr_1;


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test() {
        assert_eq!(sqr_cios_opt_unr_2([7043351708507219244, 12973988534611502911, 2286018893667355461, 3390003277958482388]), [939962812878190028, 8706293248576803058, 16205989205945120409, 3883740201024575067]); 
        for _ in 0..1000000 {
            let input: [u64; 4] = [
                rand::random::<u64>(),
                rand::random::<u64>(),
                rand::random::<u64>(),
                rand::random::<u64>(),
            ];
            println!("{:?}", input);
            assert_eq!(sqr_cios_opt_unr_2(input), sqr_cios_opt_unr_1(input));    
        }

        // let input: [u64; 4] = [20074282081074489, 415449875894500508, 9499724717238044174, 12059022386397940636];
        // assert_eq!(sqr_cios_opt_asm(input), sqr_cios_opt_unr(input)); 
    }
}