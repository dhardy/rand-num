extern crate rand;

use rand::*;

fn main() {
    let mut rng = OsRng::new().unwrap();
    
    println!("Random u32: 0x{0:x} = {0}", rng.next_u32());
    println!("Random u64: 0x{0:x} = {0}", rng.next_u64());
}
