pub mod servers;
pub mod modules;
pub mod persistent_trait;
pub mod data;

pub use murmur_grpc::*;
pub use persistent_trait::Persistent;
pub use data::Data;

fn main() {
    // NUM THREADS
    let thread_pool = ThreadPoolBuilder::new().num_threads(num_threads).build().unwrap();
    // ADD SERVERS TO THREAD POOL //
    std::thread::park();
}
