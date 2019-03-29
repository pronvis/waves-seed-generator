use super::waves;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::hash_map::HashMap;

const ADDRESS_SIZE: usize = 35;
static CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

pub struct AddressWithSeed {
    pub seed: String,
    pub address: String
}

pub fn generate_seed_with_address_for_worlds(worlds: &HashMap<String, usize>) -> AddressWithSeed {
    loop {
        let seed_with_address = generate_seed_with_address();
        if check_address(worlds, &seed_with_address.address) {
            return seed_with_address;
        }
    }
}

fn check_address(worlds: &HashMap<String, usize>, address: &str) -> bool {
    worlds.iter().any(|kv| check_concrete_address(address, kv.0, kv.1))
}

fn check_concrete_address(address: &str, case: &str, case_len: &usize) -> bool {
    &address[(ADDRESS_SIZE - case_len)..].to_lowercase() == case
}

fn generate_seed_with_address() -> AddressWithSeed {
    let seed = random_seed();
    let address = waves::address_from_seed(&seed);
    AddressWithSeed{seed, address}
}

fn random_seed() -> String {
    let mut rng = thread_rng();
    let seed: Option<String> = (0..64)
        .map(|_| Some(*CHARS.choose(&mut rng)? as char))
        .collect();

    seed.unwrap()
}