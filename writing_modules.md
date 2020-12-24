Example modules can be seen [here](https://github.com/chili-b/pelecan-modules)

Example module configuration
```bash
# Event handling functions

user_connected=(
  'handle_user_connected'
)
user_disconnected=(
  'handle_user_disconnected'
)
user_state_changed=(
  'user_state_changed_handler'
)
user_text_message=(
  'text_command_parser'
)
channel_created=(
  'channel_created_handler'
  'channel_renamer'
)
channel_removed=(
  'channel_removed_handler'
)
channel_state_changed=(
  'channel_state_changed_handler'
)
chat_filters=(
  'chat_filter_1'
  'chat_filter_2'
  'coffee_filter'
)
authenticators=(
  'username_filter'
)
declare -A context_actions
context_actions['context_action_1']='context_action_1_handler_1 context_action_1_handler_2'
context_actions['context_action_2']='context_action_2_handler'

# Default values for the variables the module will use

volatile_data=(
  'some_counter: usize = 20'
  'foo: String = String::from("bar")'
)

persistent_data=(
  'this_will_be_saved_to_disk: i32 = 0'
)

# Cargo dependencies

dependencies=(
  'some_crate_i_need = "1.0"'
  'another_crate_i_need = "^0.7"'
)
```
