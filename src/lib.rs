mod waves;
mod seed_generator;

use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use std::thread::JoinHandle;

pub fn main_thing(threads_count: u32, looking_for: &[String]) {
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
        loop {
            let seed_with_address = seed_generator::generate_seed_with_address_for_worlds(&map_copy);
            println!("seed: {}\naddress: {}\n----------", seed_with_address.seed, seed_with_address.address);
        }
    })
}


