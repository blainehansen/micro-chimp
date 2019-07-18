PGPASSWORD='asdf' psql -h localhost -U user database <<EOF
	insert into unsubscribe_token (site_name, token, description) values
		('crowdsell_io'::site_name_enum, '6eOplSNiFgbAKhvQL//Kyvm+qXUgbMDHm1ZouUaBm6lzUndNI36DhSJuoiLrrGdVXFEIa8DpI9DviImYaSxk7A==', 'stuff');
EOF
