use std::env;
use getopts::Options;
use waves_address_generator::generate_seeds_by_par_iter;
use waves_address_generator::generate_seeds_in_threads;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("f", "fast", "with all CPU");
    opts.optflagopt("m", "manually", "set number of threads", "3");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let mut worlds: HashMap<String, usize> = HashMap::new();
    for looking in matches.free.iter() {
        worlds.insert(looking.to_string(), looking.len());
    }

    if matches.opt_present("f") {
        return generate_seeds_by_par_iter(&worlds);
    } else if matches.opt_present("m") {
        match matches.opt_str("m") {
            None => {
                print_usage(&program, opts);
                return;
            },
            Some(number) => {
                let _threads_count: u32 = number.parse().unwrap();
                return generate_seeds_in_threads(_threads_count, &worlds);
            },
        }
    } else {
        print_usage(&program, opts);
        return;
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}