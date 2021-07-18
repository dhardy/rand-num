A very simple program to print a random number via the [Rand](https://github.com/rust-random/rand) crate.

This uses the [`thread_rng` generator](https://docs.rs/rand/0.8.4/rand/fn.thread_rng.html) seeded by [getrandom](https://github.com/rust-random/getrandom/), thus *should* be secure (unpredictable).

Example output:
```
Random u8: 0x6d = 109
Random u16: 0x809c = 32924
Random u32: 0xea319361 = 3929117537
Random u64: 0x376d4c19009b2592 = 3993932114815559058
Random u128: 0x1363867ea3d085cddaec7ebfff77126 = 1610756074077616292812447987608809766
Random i8: 0xb4 = -76
Random i16: 0x7172 = 29042
Random i32: 0x5eb54b98 = 1588939672
Random i64: 0xc5cbc9fae82c5f8f = -4194036548491255921
Random i128: 0x253939a7c460a46b450db1f698ea0575 = 49478566154204815750172915851867063669
```
