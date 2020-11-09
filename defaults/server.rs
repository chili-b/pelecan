mod data;
use data::Data;
use crate::modules;

// MODULES //

pub fn murmur_interface() -> MurmurInterface<&'static str, Data> {
    let name = "server";
    // SERVER NAME
    let server_id = 1;
    // SERVER ID
    let server_addr = "http://127.0.0.1:50051"
    // SERVER ADDR
    MurmurInterface {
        t: DataMutex::new(Data::load(name)),
        addr: addr,
        server_id: server_id,
        user_connected: vec![
            // USER CONNECTED
        ],
        user_disconnected: vec![
            // USER DISCONNECTED
        ],
        user_state_changed: vec![
            // USER STATE CHANGED
        ],
        user_text_message: vec![
            // USER TEXT MESSAGE
        ],
        channel_created: vec![
            // CHANNEL CREATED
        ],
        channel_removed: vec![
            // CHANNEL REMOVED
        ],
        channel_state_changed: vec![
            // CHANNEL STATE CHANGED    
        ],
        chat_filters: vec![
            // CHAT FILTERS
        ],
        authenticators: vec![
            // AUTHENTICATORS
        ],
        context_actions: vec![
            // CONTEXT ACTIONS
        ],
    }
}
