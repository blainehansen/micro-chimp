create extension citext;

\set rust_server_password `sed 's/^[ \t]*//;s/[ \t]*$//' < /keys/.rust-db-key`
create role rust_server_user login password :'rust_server_password';

\set node_client_password `sed 's/^[ \t]*//;s/[ \t]*$//' < /keys/.node-db-key`
create role node_client_user login password :'node_client_password';
