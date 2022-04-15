use sha2::{Digest, Sha256};
// import commonly used items from the prelude:
use rand::prelude::*;

pub fn random_number() -> i32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(0..i32::MAX);
}

pub fn generate_hash(number: i32, hash: &str, name: &str) -> String {
    let mut hasher = Sha256::new();

    // https://github.com/hoodie/concatenation_benchmarks-rs
    let mut x = String::with_capacity(83);
    x.push_str(&number.to_string());
    x.push_str(hash);
    x.push_str(name);

    hasher.update(x);
    let result = hasher.finalize();

    let hex_string = format!("{:x}", result);
    return hex_string;
}


pub fn generate_hash256(number: i32, hash: &str, name: &str) -> [u8; 32] {
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


pub fn is_smaller2() {

}


// h1 < h2/2
pub fn is_smaller(h1: &String, h2: &String) -> bool {
    // for i in (0..256).step_by(128) {
    //     let h1_value = h1.as_slice().to_hex();
    // }
    // let x = i128::BITS;
    // let tmp:i128 = 0xFFFFFFFFFFFFFFFF;

    // let mut x = 7;
    // unsafe {
    //     asm!(
    //         "mov {tmp}, {x}",
    //         "shl {tmp}, 1",
    //         "shl {x}, 2",
    //         "add {x}, {tmp}",
    //         x = in(reg) h1,
    //         tmp = out(reg) _,
    //     );
    // }

    let h1_bytes = &h1.as_bytes();
    let h2_bytes = h2.as_bytes();

    for i in 0..h1_bytes.len() {
        let x = byte_to_hex(h1_bytes[i]);
        let mut y = byte_to_hex(h2_bytes[i]);
        y = y >> 1;


        if x < y {
            return true; // smaller
        } else if x > y {
            return false; // bigger
            }
        }
    

    return false; // even
}



fn byte_to_hex(byte: u8) -> u32 {
    match byte {
        b'0' => 0,
        b'1' => 1,
        b'2' => 2,
        b'3' => 3,
        b'4' => 4,
        b'5' => 5,
        b'6' => 6,
        b'7' => 7,
        b'8' => 8,
        b'9' => 9,
        b'a' => 10,
        b'b' => 11,
        b'c' => 12,
        b'd' => 13,
        b'e' => 14,
        b'f' => 15,
        _ => panic!("Invalid hex value"),
    }
}
