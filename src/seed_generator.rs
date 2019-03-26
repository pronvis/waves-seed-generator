use super::waves;

use rand::seq::SliceRandom;
use rand::thread_rng;

static CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

pub struct AddressWithSeed {
    pub seed: String,
    pub address: String
}

pub fn generate_seed_with_address() -> AddressWithSeed {
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