use blake2::Digest;
use sha3::Keccak256;
use crev_common::blake2b256::Blake2b256;
use crypto::digest::Digest as Crypto_Digest;
use crypto::curve25519::curve25519_base;
use crypto::sha2::Sha256;
use base58::ToBase58;
use base58::FromBase58;

static  PROD_SCHEMA_BYTE: u8 = 87;// for char 'W'

pub fn address_from_seed(seed_bytes: &[u8]) -> String {
    let with_4_zeroes = [&[0; 4], seed_bytes].concat();
    let keccak256 = waves_secure_hash(&with_4_zeroes);
    let public_key = public_key(&keccak256);

    let public_key_str = public_key.to_base58();
    let public_key_58_bytes = public_key_str.from_base58().unwrap();

    waves_address_from_public_key(public_key_58_bytes.as_slice(), PROD_SCHEMA_BYTE)
}

fn waves_secure_hash(input: &[u8]) -> Vec<u8> {
    let blake2b256 = blake2b256(input);
    let keccak256 = Keccak256::digest(blake2b256.as_slice());
    keccak256.to_vec()
}

fn blake2b256(data: &[u8]) -> Vec<u8> {
    let generic_array = Blake2b256::new().chain(data).result();
    generic_array.to_vec()
}

fn public_key(secure_hash: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.input(secure_hash);
    let mut sha256_result: [u8; 32] = [0; 32];
    hasher.result(&mut sha256_result);

    curve25519_base(&sha256_result)
}

fn waves_address_from_public_key(input: &[u8], chain_id: u8) -> String {
    let public_key_hash = waves_secure_hash(input);
    let address_version_with_chain: [u8; 2] = [1, chain_id];
    let without_checksum = [&address_version_with_chain, &public_key_hash[0..20]].concat();
    let checksum_secure_has = waves_secure_hash(&without_checksum);
    let bytes = [&without_checksum, &checksum_secure_has[..4]].concat();
    bytes.to_base58()
}
