pub mod servers;
pub mod persistent_trait;

pub use murmur_grpc::*;
pub use persistent_trait::Persistent;

fn main() {
    env_logger::init();
    let mut manager = ClientManager::new();
    // ADD CONNECTION //
    std::thread::park();
}
