# Waves seed generator

This program will generate seeds for [Waves](https://github.com/wavesplatform/waves) which public key (address) is ends with what you want.

### usage

```
Options:
    -f, --fast            with all CPU
    -m, --manually [3]    set number of threads
    -h, --help            print this help menu
```

1. `cargo build --release`

2. a. `./target/release/waves-address-generator -f {word1} {word2}`

   b. `./target/release/waves-address-generator -m 4 {word1} {word2}`

For example `./waves-address-generator -f -w xxx yyy` will start generator with consuming all available CPU. It will print only those seeds which address ends with "xxx" or "yyy".