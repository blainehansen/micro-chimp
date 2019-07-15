create extension citext;

-- \set rust_server_password `sed 's/^[ \t]*//;s/[ \t]*$//' < /keys/.rust-db-key`
-- create role rust_server_user login password :'rust_server_password';
create role rust_server_user login password 'password';

-- \set node_client_password `sed 's/^[ \t]*//;s/[ \t]*$//' < /keys/.node-db-key`
-- create role node_client_user login password :'node_client_password';
create role node_client_user login password 'password';

create table subscription (
	email citext not null check (email ~* '^.+@.+\..+$'),
	site_name site_name_enum not null,

	primary key (email, site_name),

	-- validation_token text unique check (character_length(validation_token) = 86),
	validation_token text unique check (character_length(validation_token) = 88),
	unsubscribed_with text
);

-- TODO create index on validation_token

create function is_unsubscribed(subscription) returns boolean as $$
  select $1.unsubscribed_with is not null;
$$ language sql immutable;

create function is_validated(subscription) returns boolean as $$
  select $1.validation_token is null;
$$ language sql immutable;

create table unsubscribe_token (
	site_name site_name_enum not null,
	-- token text not null check (character_length(token) = 86),
	token text not null check (character_length(token) = 88),
	primary key (site_name, token),
	description text not null
);


alter table subscription
add constraint unsubscribed_with_fk
foreign key (site_name, unsubscribed_with) references unsubscribe_token (site_name, token);

alter table subscription enable row level security;

grant select (validation_token) on table subscription to rust_server_user;
create policy rust_select_email on subscription for select to rust_server_user
	using (true);

grant insert (email, site_name, validation_token) on table subscription to rust_server_user;
create policy rust_insert_email on subscription for insert to rust_server_user
	with check (validation_token is not null);

grant update (validation_token) on table subscription to rust_server_user;
create policy rust_verify_email on subscription for update to rust_server_user
	-- using (validation_token is not null)
	using (true)
	with check (validation_token is null);

grant select (unsubscribed_with, email, site_name) on table subscription to rust_server_user;
grant update (unsubscribed_with) on table subscription to rust_server_user;
create policy rust_unsubscribe_email on subscription for update to rust_server_user
	using (true)
	with check (unsubscribed_with is not null);

-- grant select * on table emails to node_client_user;
-- grant select * on table unsubscribe_tokens to node_client_user;
