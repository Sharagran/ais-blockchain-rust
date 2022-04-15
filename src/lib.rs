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
            test::black_box(hash_mod::generate_hash(
                hash_mod::random_number(),
                &String::from("000004f243aa198b585a378c208e2bc5bb04ddf089e5605725875fe0981d7c0c"),
                &String::from("FilipMaas"),
            ));
        });
    }

    #[bench]
    fn bench_is_smaller(b: &mut Bencher) {
        b.iter(|| {
            // Use `test::black_box` to prevent compiler optimizations from disregarding
            // Unused values
            test::black_box(hash_mod::is_smaller(
                "000000000000000000000000000000000000000000000000000000000000000c".as_bytes(),
                "000000000000000000000000000000000000000000000000000000000000000f".as_bytes(),
            ));
        });
    }

    #[test]
    fn test_is_smaller() {
        is_smaller(
            String::from("000000000000000000000000000000000000000000000000000000000000007f"),
            String::from("00000000000000000000000000000000000000000000000000000000000000fe"),
        ); //smaller
        is_smaller(
            String::from("000000000000000000000000000000000000000000000000000000000000007f"),
            String::from("00000000000000000000000000000000000000000000000000000000000000ff"),
        ); //even
        is_smaller(
            String::from("000a97e5e8f14e6c8c31c16781b25daa154df1638cf41f77da0066f779907155"),
            String::from("0001e838165c41adf44d758b2ce17388a95f8a2fb7997a53d9b5667343e38f49"),
        ); //bigger

        //check all combinations
        for i in 0..16 {
            for j in 0..16 {
                let hex1 = format!("{:x}", i) + &format!("{:x}", j);
                for k in 0..16 {
                    for l in 0..16 {
                        let hex2 = format!("{:x}", k) + &format!("{:x}", l);
                        is_smaller(hex1.clone(), hex2);
                    }
                }
            }
        }

    }

    fn is_smaller(hex1: String, hex2: String) {
        assert_eq!(
            hash_mod::is_smaller(&hex1.as_bytes(), &hex2.as_bytes()),
            validate_is_smaller(hex1, hex2)
        ); //smaller
    }

    use std::process::Command;
    fn validate_is_smaller(hex1: String, hex2: String) -> bool {
        //execute python code
        let mut python_script = Command::new("python");
        python_script
            .arg("compare_hex_values.py")
            .arg(hex1)
            .arg(hex2);
        let output = python_script.output().expect("failed to execute process");
        let output_string = String::from_utf8_lossy(&output.stdout);
        return output_string.contains("True");
    }
}
