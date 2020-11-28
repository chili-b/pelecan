#!/bin/bash

function enable_module {
	module="$1"
	server="$2"
	if test -d modules/$module; then
		echo "found module '${module}'"
		cp -r modules/$module pelecan/src/servers/${server}/
		insert_line "MODULES" "mod ${module};" pelecan/src/servers/${server}/mod.rs
		reset_module_variables
		source modules/$module/module
		# add dependencies
		for dependency in "${dependencies[@]}"; do
			add_dependency "$dependency"
		done
		for volatile_data_variable in "${volatile_data[@]}"; do
			add_data_variable 'VOLATILE' "$volatile_data_variable" "$server"
		done
		for persistent_data_variable in "${persistent_data[@]}"; do
			add_data_variable 'PERSISTENT' "$persistent_data_variable" "$server"
		done
		# user connected
		insert_module_functions "$module" "USER CONNECTED" "$server" "${user_connected[@]}"
		# user disconnected
		insert_module_functions "$module" "USER DISCONNECTED" "$server" "${user_disconnected[@]}"
		# user state changed
		insert_module_functions "$module" "USER STATE CHANGED" "$server" "${user_state_changed[@]}"
		# user text message
		insert_module_functions "$module" "USER TEXT MESSAGE" "$server" "${user_text_message[@]}"
		# channel created
		insert_module_functions "$module" "CHANNEL CREATED" "$server" "${channel_created[@]}"
		# channel removed
		insert_module_functions "$module" "CHANNEL REMOVED" "$server" "${channel_removed[@]}"
		# channel state changed
		insert_module_functions "$module" "CHANNEL STATE CHANGED" "$server" "${channel_state_changed[@]}"
		# chat filters
		insert_module_functions "$module" "CHAT FILTERS" "$server" "${chat_filters[@]}"
		# authenticators
		insert_module_functions "$module" "AUTHENTICATORS" "$server" "${authenticators[@]}"
		# context actions
		module_file="pelecan/src/servers/${server}/mod.rs"
		for context_action_name in "${!context_actions[@]}"; do
			context_action_function="${module}::${context_action_name}"
			if ! grep -q "$context_action_function" pelecan/src/servers/${server}/mod.rs; then
				insert_line "CONTEXT ACTIONS" \
					"(${context_action_function}(), vec![\\n// CONTEXT ACTION ${context_action_function}\\n])," \
					"${module_file}"
			fi
			for context_action_handler in ${context_actions[$context_action_name]}; do
				insert_line "CONTEXT ACTION ${context_action_function}" \
					"$module::$context_action_handler," \
					"${module_file}"
			done
		done
	else
		echo "no module named '${module}'"
	fi
}

function insert_module_functions {
	module="$1"
	pattern="$2"
	server="$3"
	shift; shift; shift
	function_array=("$@")
	module_file="pelecan/src/servers/${server}/mod.rs"
	for function in "${function_array[@]}"; do
		if ! test -z "$function"; then
			insert_line "$pattern" "${module}::${function}," "$module_file"
		fi
	done
}

function add_data_variable {
	data_type="$1"
	variable="$2"
	server="$3"
	var_name=$(echo $variable | cut -f 1 -d : | xargs echo)
	var_type=$(echo $variable | cut -f 2 -d : | cut -f 1 -d = | xargs echo)v
	var_value=$(echo $variable | cut -f 2 -d = | xargs echo)
	insert_line "${data_type} DATA FIELDS" "${var_name}: ${var_type}," "pelecan/src/servers/${server}/data.rs"
	insert_line "${data_type} DATA VALUES" "${var_name}: ${var_value}," "pelecan/src/servers/${server}/data.rs"
}

function add_dependency {
	dependency=$1
	name=$(echo "$dependency" | cut -f 1 -d = | xargs echo)
	if ! grep -q $name pelecan/Cargo.toml; then
		insert_line 'DEPENDENCIES' "$dependency" pelecan/Cargo.toml
	else
		echo "${name} is already a dependency, so not added"
	fi
}

function insert_line {
	pattern="$1"
	line="$2"
	file="$3"
	line_num=$(grep -n -m 1 "\b${pattern}\b" "$file" | cut -f 1 -d :)
	sed -i "${line_num} i ${line}" "$file"
}

function reset_module_variables {
	unset user_connected
	unset user_disconnected
	unset user_state_changed
	unset user_text_message
	unset channel_created
	unset channel_removed
	unset channel_state_changed
	unset chat_filters
	unset authenticators
	unset context_actions
	unset dependencies
	unset volatile_data
	unset persistent_data
}

function reset_server_variables {
	unset server_id
	unset server_address
}

cp defaults/Cargo.toml pelecan/Cargo.toml

cp defaults/main.rs pelecan/src/main.rs

cp defaults/persistent_trait.rs pelecan/src/persistent_trait.rs

rm -r pelecan/src/modules/* 2&> /dev/null
touch pelecan/src/modules/mod.rs

rm -r pelecan/src/servers/* 2&> /dev/null
touch pelecan/src/servers/mod.rs

dir=$(pwd)
cd "$(dirname "$0")"

source config
insert_line "NUM THREADS" "let num_threads = ${number_of_threads};" pelecan/src/main.rs

for server_name in $(ls servers); do
	module_file="pelecan/src/servers/${server_name}/mod.rs"
	server=servers/$server_name
	if test -d $server && test -f $server/config && test -f $server/enabled_modules; then
		mkdir pelecan/src/servers/"$server_name"
		cp defaults/server.rs pelecan/src/servers/"$server_name"/mod.rs
		cp defaults/data.rs pelecan/src/servers/"$server_name"/data.rs
		reset_server_variables
		source "$server/config"
		echo "pub mod ${server_name};" >> pelecan/src/servers/mod.rs
		insert_line "SERVER NAME" "let name = \"${server_name}\";" "${module_file}"
		insert_line "SERVER ID" "let server_id = ${server_id};" "${module_file}"
		insert_line "SERVER ADDR" "let server_addr = \"${server_address}\";" "${module_file}"
		insert_line "ADD SERVERS TO THREAD POOL" "add_connection_to_thread_pool(&thread_pool, servers::${server_name}::murmur_interface(), None);" "pelecan/src/main.rs"
		while IFS= read -r line; do
			enable_module "$line" "$server_name"
		done < "$server/enabled_modules"
	fi
done

cd pelecan
cargo build --release
killall pelecan
cp target/release/pelecan ../bin/pelecan
cd ../
strip bin/pelecan

cd $dir
