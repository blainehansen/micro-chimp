create table emails (
	email citext unique check (email ~* '^.+@.+\..+$'),
	-- site_name site_name_enum not null,
	site_name text not null check (site_name in ('crowdsell', 'blog')),
	-- unsubscribe_token text unique
	validation_token text unique
);
