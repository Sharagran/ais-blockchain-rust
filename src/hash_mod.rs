use sha2::{Digest, Sha256};
// import commonly used items from the prelude:
use rand::prelude::*;

pub fn random_number() -> i32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(0..i32::MAX);
}

pub fn generate_hash(number: i32, hash: &str, name: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    // https://github.com/hoodie/concatenation_benchmarks-rs
    let mut x = String::with_capacity(83);
    x.push_str(&number.to_string());
    x.push_str(hash);
    x.push_str(name);

    hasher.update(x);
    let result = hasher.finalize();
    return result.into();
}

// h1 < h2/2
pub fn is_smaller(h1: &[u8], h2: &[u8]) -> bool {
    let mut h2_div2 = h2.to_owned();
    h2_div2.rotate_right(1);
    h2_div2[0] = 0;
    return h1 < h2;
}

