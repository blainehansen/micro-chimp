create table subscription (
	email citext check (email ~* '^.+@.+\..+$'),
	-- site_name site_name_enum not null,
	site_name text check (site_name in ('crowdsell', 'blog')),

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
