extern crate rand;
extern crate env_logger;

use rand::*;

fn main() {
    env_logger::init();
    let mut rng = thread_rng();
    
    println!("Random u32: 0x{0:x} = {0}", rng.next_u32());
    println!("Random u64: 0x{0:x} = {0}", rng.next_u64());
}
