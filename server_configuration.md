To create a server, create a subdirectory under the `servers` directory. The name of this directory will
be the name pelecan uses to identify the server. Inside this directory, create two files: `enabled_modules`
and `config`. 

`config` should be formatted as follows:

```bash
# server_id is the id of the virtual server at the given address to connect to.
# If you are only running a single Mumble server, use 1.
server_id=1
# server_address is the endpoint at which your Mumble server is listening for
# gRPC connections. You should use the value below if you are running pelecan
# and the Mumble server on the same device over the default port without TLS.
server_address="http://127.0.0.1:50051"
```

`enabled_modules` should contain the name of a single subdirectory of the `/modules` directory. If `/modules`
contained directories `module_1`, `module_2` and `module_3`, in order to enable modules 2 and 3 for a server
called 'server_1' the contents of `servers/server_1/enabled_modules` would be:

```bash
module_2
module_3
```
