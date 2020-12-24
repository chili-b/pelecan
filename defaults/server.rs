use crate::*;
mod data;
use data::Data;
use std::sync::{Arc, Mutex};

// MODULES //

pub fn murmur_interface() -> MurmurInterface<Data> 
{
    // SERVER NAME
    // SERVER ID
    // SERVER ADDR
    MurmurInterface {
        t: Arc::new(Mutex::new(Data::new(name))),
        addr: server_addr.to_string(),
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
