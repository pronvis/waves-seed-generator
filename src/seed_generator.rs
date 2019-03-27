use super::waves;

use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use rand::distributions::Uniform;
use std::cell::RefCell;

const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

use  lazy_static;
lazy_static! {
    static ref CHARS_LENGTH: usize = CHARS.len();
    static ref DISTRIBUTION: Uniform<usize> = Uniform::new(0, *CHARS_LENGTH);
}

pub struct AddressWithSeed {
    pub seed: Vec<u8>,
    pub address: String
}

pub fn generate_seed_with_address() -> AddressWithSeed {
    let seed: Vec<u8> = random_seed();
    let address = waves::address_from_seed(&seed);
    AddressWithSeed{seed, address}
}

fn random_seed() -> Vec<u8> {
    let mut rng = thread_rng();

    let indexes: Vec<usize> = (0..64).map(|_| rng.sample(*DISTRIBUTION)).collect();
    indexes.iter().map(|i| CHARS[*i]).collect()
}