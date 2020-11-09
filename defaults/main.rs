mod servers;
mod modules;
mod persistent_trait;
mod data;

use murmur_grpc::*;
use persistent_trait::Persistent;
use data::Data;

fn main() {
    // NUM THREADS
    let thread_pool = ThreadPoolBuilder::new().num_threads(num_threads).build().unwrap();
    // ADD SERVERS TO THREAD POOL //
    std::thread::park();
}
