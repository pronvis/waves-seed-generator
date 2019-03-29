mod waves;
pub mod seed_generator;

use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use std::thread::JoinHandle;

use rayon::prelude::*;
use seed_generator::AddressWithSeed;

pub fn generate_seeds_by_par_iter(worlds: &HashMap<String, usize>) {
    println!("starts with input: {:?}", worlds);

    rayon::iter::repeat(0)
        .map(|_| seed_generator::generate_seed_with_address())
        .filter(|seed_with_address| seed_generator::check_address(worlds, &seed_with_address.address) == true)
        .for_each(print_result);
}

pub fn generate_seeds_in_threads(threads_count: u32, worlds: &HashMap<String, usize>) {

    let mut children: Vec<JoinHandle<()>> = vec![];
    for _i in 0..threads_count {
        let handler = spawn_thread_to_generate_seeds(worlds);
        children.push(handler);
    };
    thread::sleep(Duration::from_secs(1));
    println!("----------");

    for child in children {
        let _ = child.join();
    }
}

fn spawn_thread_to_generate_seeds(worlds: &HashMap<String, usize>) -> JoinHandle<()> {
    let mut map_copy: HashMap<String, usize> = HashMap::new();
    HashMap::clone_from(&mut map_copy, &worlds);

    thread::spawn(move || {
        let thread_id = thread::current().id();
        println!("{:?} thread starts with input: {:?}", thread_id, map_copy);
        loop {
            let seed_with_address = seed_generator::generate_seed_with_address_for_worlds(&map_copy);
            print_result(seed_with_address);
        }
    })
}

fn print_result(seed_with_address: AddressWithSeed) -> () {
    println!("seed: {}\naddress: {}\n----------", seed_with_address.seed, seed_with_address.address);
}
