PGPASSWORD='password' psql -h localhost -U rust_server_user database <<EOF
	update subscription set unsubscribed_with = '6eOplSNiFgbAKhvQL//Kyvm+qXUgbMDHm1ZouUaBm6lzUndNI36DhSJuoiLrrGdVXFEIa8DpI9DviImYaSxk7A=='
	where
		email = 'dude@gmail.com'
		and site_name = 'crowdsell_io'::site_name_enum
	;
EOF
