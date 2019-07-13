create extension citext;

\set rust_server_password `sed 's/^[ \t]*//;s/[ \t]*$//' < /keys/.rust-db-key`
create role rust_server_user login password :'rust_server_password';

\set node_client_password `sed 's/^[ \t]*//;s/[ \t]*$//' < /keys/.node-db-key`
create role node_client_user login password :'node_client_password';

-- this will exist from the node-codegen
-- create type site_name_enum as enum(${allowed_names});

create table subscription (
	email citext check (email ~* '^.+@.+\..+$'),
	site_name site_name_enum not null,

	primary key (email, site_name)

	validation_token text unique check (character_length(validation_token) = 86),
	unsubscribed_with text,

	foreign key (site_name, unsubscribed_with) references unsubscribe_tokens(site_name, unsubscribe_token)
);

-- create index on validation_token

create function is_unsubscribed(subscription) returns boolean as $$
  select $1.unsubscribed_with is not null;
$$ language sql immutable;

create function is_validated(subscription) returns boolean as $$
  select $1.validation_token is null;
$$ language sql immutable;

create table unsubscribe_tokens (
	unsubscribe_token text not null check (character_length(validation_token) = 86),
	site_name text not null check (site_name in ('crowdsell', 'blog')),
	description text not null
);


alter table emails enable row level security;

grant select (validation_token) on table emails to rust_server_user;
create policy rust_select_email on emails for select to rust_server_user
	using (true);

grant insert (email, validation_token) on table emails to rust_server_user;
create policy rust_insert_email on emails for insert to rust_server_user
	with check (validation_token is not null);
	-- with check (character_length(validation_token) = 86);

grant update (validation_token) on table emails to rust_server_user;
create policy rust_update_email on emails for update to rust_server_user
	using (validation_token is not null)
	with check (validation_token is null);



grant select * on table emails to node_client_user;
grant select * on table unsubscribe_tokens to node_client_user;