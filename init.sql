create extension citext;

-- \set rust_server_password `sed 's/^[ \t]*//;s/[ \t]*$//' < /keys/.go-db-key`
-- create role rust_server_user login password :'rust_server_password';

-- create type site_name as enum('crowdsell', 'blog');

create table emails (
	email citext unique check (email ~* '^.+@.+\..+$'),
	site site_name not null,
	validation_token text unique
);

-- alter table emails enable row level security;

-- grant select (validation_token) on table emails to rust_server_user;
-- create policy go_select_email on emails for select to rust_server_user
-- 	using (true);

-- grant insert (email, validation_token) on table emails to rust_server_user;
-- create policy go_insert_email on emails for insert to rust_server_user
-- 	with check (character_length(validation_token) = 86);

-- grant update (validation_token) on table emails to rust_server_user;
-- create policy go_update_email on emails for update to rust_server_user
-- 	using (validation_token is not null)
-- 	with check (validation_token is null);
