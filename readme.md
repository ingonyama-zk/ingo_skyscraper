Standalone implementation of Skyscraper 2-to-1 hash function for BN254 scalar field elements.
See full paper at https://eprint.iacr.org/2025/058.pdf
Implementation assumes each field element is represented as an array of four u64.
Implememtation is optimized for a 64b CPU assuming it is executed on a single thread (not using SIMD).
Top function is compress().
To run the benchmark type "cargo bench".
