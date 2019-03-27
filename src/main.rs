mod waves;
mod seed_generator;

use std::env;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use std::thread::JoinHandle;

use std::time::Instant;

const ADDRESS_SIZE: usize = 35;

fn main() {
    let env_args: Vec<String> = env::args().collect();
    let threads_count: u32 = env_args[1].parse().unwrap();
    let looking_for = &env_args[2..];

    let mut worlds: HashMap<String, usize> = HashMap::new();
    for looking in looking_for.iter() {
        worlds.insert(looking.to_string(), looking.len());
    }

    let mut children: Vec<JoinHandle<()>> = vec![];
    for _i in 0..threads_count {
        let handler = run_some_work(&worlds);
        children.push(handler);
    };
    thread::sleep(Duration::from_secs(1));
    println!("----------");

    for child in children {
        let _ = child.join();
    }
}

fn run_some_work(worlds: &HashMap<String, usize>) -> JoinHandle<()> {
    let mut map_copy: HashMap<String, usize> = HashMap::new();
    HashMap::clone_from(&mut map_copy, &worlds);
    thread::spawn(move || {
        let thread_id = thread::current().id();
        println!("{:?} thread starts with input: {:?}", thread_id, map_copy);
        let mut counter = 0;
        let start = Instant::now();
        let mut curr_time = start.elapsed().as_millis();
        loop {
            some_work(&map_copy);
            if(counter % 50000 == 0) {
                let elapsed_time = start.elapsed().as_millis();
                println!("> {} <   {:?} :: {}", elapsed_time - curr_time, thread_id, counter);
                curr_time = elapsed_time;
            }
            counter += 1;
        }
    })
}

fn some_work(worlds: &HashMap<String, usize>) -> () {
    let seed_with_address = seed_generator::generate_seed_with_address();
    check_address(worlds, &seed_with_address.address);
//    if check_address(worlds, &seed_with_address.address) {
//        println!("seed: {}\naddress: {}\n----------", seed_with_address.seed, seed_with_address.address)
//    }
}

fn check_address(worlds: &HashMap<String, usize>, address: &str) -> bool {
    worlds.iter().any(|kv| check_concrete_address(address, kv.0, kv.1))
}

fn check_concrete_address(address: &str, case: &str, case_len: &usize) -> bool {
    &address[(ADDRESS_SIZE - case_len)..].to_lowercase() == case
}
