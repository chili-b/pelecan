pub mod servers;
pub mod persistent_trait;

pub use murmur_grpc::*;
pub use persistent_trait::Persistent;

fn main() {
    // NUM THREADS
    let thread_pool = ThreadPoolBuilder::new().num_threads(num_threads).build().unwrap();
    // ADD SERVERS TO THREAD POOL //
    std::thread::park();
}
