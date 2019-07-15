PGPASSWORD='password' psql -h localhost -U rust_server_user database <<EOF
	insert into subscription (email, site_name, validation_token) values
	('dude@gmail.com', 'crowdsell_io'::site_name_enum, '5lhI4yC6t2Vn/T/5oKdrDM8urmKOJoj+UeFuXfvlcZmYoBoKO5FzQl0bKFyZNgaGqVGCoQXi53hDDHc/EMbXvw==');
EOF
