use std::time::Instant;
use std::thread;

pub mod hash_mod;

pub const THREAD_COUNT:i32 = 12;

// threads: https://doc.rust-lang.org/book/ch16-01-threads.html
// counter: https://doc.rust-lang.org/book/ch16-03-shared-state.html
fn main() {
    assert_eq!(hash_mod::is_smaller(&String::from("000000000000000000000000000000000000000000000000000000000000007f"), &String::from("00000000000000000000000000000000000000000000000000000000000000ff")), false); //even

    let hash1 = String::from("70000000751afc45531920752b4f61cb6dfb06e8e874d1e3130c4aee4050f4b6");

    let start = Instant::now();
    // start 12 workers
    let mut handles = vec![];
    for i in 0..THREAD_COUNT {
        let handle = new_worker(hash1.clone(), i);
        handles.push(handle);
    }

    // wait for all workers to finish
    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

}

fn new_worker(data: String, id:i32) -> thread::JoinHandle<()> {
    let handle = thread::spawn(move || {
        println!("Worker {} started", id);
        let hash = find_smaller_hash(data);
        println!("Worker {} finished", id);
        println!("Hash: {}", hash);
    });

    return handle;
}

fn find_smaller_hash(hash1: String) -> String {
    loop {
        let number = hash_mod::random_number();
        let hash2 = hash_mod::generate_hash(number, &hash1, "FilipMaas");
        if hash_mod::is_smaller(&hash2, &hash1) {
            return hash2;
        }
    }
}

