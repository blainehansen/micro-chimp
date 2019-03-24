create extension citext;

\set rust_server_password `sed 's/^[ \t]*//;s/[ \t]*$//' < /keys/.go-db-key`
create role rust_server_user login password :'rust_server_password';
