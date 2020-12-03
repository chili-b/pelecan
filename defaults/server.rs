use crate::*;
mod data;
use data::Data;

// MODULES //

pub fn murmur_interface() -> MurmurInterface<&'static str, Data> {
    // SERVER NAME
    // SERVER ID
    // SERVER ADDR
    MurmurInterface {
        t: DataMutex::new(Data::new(name)),
        addr: server_addr,
        server_id: server_id,
        auto_reconnect: true,
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
        server_connected: vec![
        ],
        server_disconnected: vec![
        ],
    }
}
