PGPASSWORD='password' psql -h localhost -U rust_server_user database <<EOF
	update subscription set validation_token = null
	where validation_token = '5lhI4yC6t2Vn/T/5oKdrDM8urmKOJoj+UeFuXfvlcZmYoBoKO5FzQl0bKFyZNgaGqVGCoQXi53hDDHc/EMbXvw==';
EOF
