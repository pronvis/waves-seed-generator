use std::env;
use waves_address_generator::main_thing;

fn main() {
    let env_args: Vec<String> = env::args().collect();
    let threads_count: u32 = env_args[1].parse().unwrap();
    let looking_for = &env_args[2..];

    main_thing(threads_count, looking_for);
}
