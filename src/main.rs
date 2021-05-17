use rand::prelude::*;

fn main() {
    let mut rng = thread_rng();

    macro_rules! out {
        ($t:ty) => {
            println!("Random {0}: 0x{1:x} = {1}", stringify!($t), rng.gen::<$t>());
        };
        ($t:ty, $($tt:ty),*) => {
            out!($t);
            out!($($tt),*);
        };
    }
    
    out!(u8, u16, u32, u64, u128);
    out!(i8, i16, i32, i64, i128);
}
