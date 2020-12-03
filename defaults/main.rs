pub mod servers;
pub mod persistent_trait;

pub use murmur_grpc::*;
pub use persistent_trait::Persistent;

fn main() {
    // ADD SERVERS TO THREAD POOL //
    std::thread::park();
}
