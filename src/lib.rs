#![feature(test)]

extern crate test;

pub mod hash_mod;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

#[bench]
fn bench_rng(b: &mut Bencher) {
    b.iter(|| {
        // Use `test::black_box` to prevent compiler optimizations from disregarding
        // Unused values
        test::black_box(hash_mod::random_number());
    });
}

#[bench]
fn bench_hashgen(b: &mut Bencher) {
    b.iter(|| {
        // Use `test::black_box` to prevent compiler optimizations from disregarding
        // Unused values
        test::black_box(hash_mod::generate_hash(hash_mod::random_number(), &String::from("000004f243aa198b585a378c208e2bc5bb04ddf089e5605725875fe0981d7c0c"), &String::from("FilipMaas")));
    });
}

#[bench]
fn bench_is_smaller(b: &mut Bencher) {
    b.iter(|| {
        // Use `test::black_box` to prevent compiler optimizations from disregarding
        // Unused values
        test::black_box(hash_mod::is_smaller(&String::from("000000000000000000000000000000000000000000000000000000000000000c"), &String::from("000000000000000000000000000000000000000000000000000000000000000f")));
    });
}

#[test]
fn test_is_smaller() {
    assert_eq!(hash_mod::is_smaller(&String::from("000000000000000000000000000000000000000000000000000000000000007f"), &String::from("00000000000000000000000000000000000000000000000000000000000000fe")), true); //smaller
    assert_eq!(hash_mod::is_smaller(&String::from("000000000000000000000000000000000000000000000000000000000000007f"), &String::from("00000000000000000000000000000000000000000000000000000000000000ff")), false); //even
    assert_eq!(hash_mod::is_smaller(&String::from("000a97e5e8f14e6c8c31c16781b25daa154df1638cf41f77da0066f779907155"), &String::from("0001e838165c41adf44d758b2ce17388a95f8a2fb7997a53d9b5667343e38f49")), false); //bigger

    // for i in 10..15 {
    //     let hex = format!("{:x}", i) + "0";
    //     assert_eq!(hash_mod::is_smaller(&String::from(&hex), &String::from("ff")), true);
    //     assert_eq!(hash_mod::is_smaller(&String::from("00"), &String::from(&hex)), true);
    //     assert_eq!(hash_mod::is_smaller(&String::from(&hex), &String::from("00")), false);
    //     assert_eq!(hash_mod::is_smaller(&String::from("ff"), &String::from(&hex)), false);
    //     assert_eq!(hash_mod::is_smaller(&String::from(&hex), &String::from(&hex)), false);
    // }
}

}
