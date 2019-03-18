type ResponseFuture = Box<Future<Item=Response<Body>, Error=hyper::Error> + Send>;


fn handle_new_email(req: Request<Body>) -> ResponseFuture {
	Box::new(req.into_body().concat2().map(|b| {
		info!("just beginning");
		let r: NewEmail = match serde_json::from_slice(b.as_ref()) {
			Ok(r) => r,
			Err(e) => {
				return
					if e.is_syntax() || e.is_data() { empty_status(StatusCode::UNPROCESSABLE_ENTITY) }
					else { empty_status(StatusCode::BAD_REQUEST) }
			},
		};

		info!("about to validate");
		if !checkmail::validate_email(&r.email) {
			return empty_status(StatusCode::BAD_REQUEST);
		}

		if !["crowdsell", "blog"].contains(&r.service.as_str()) {
			// pretending everything's okay
			return empty_status(StatusCode::NO_CONTENT);
		}

		// generate validation token
		info!("getting token");
		let validation_token =
			if let Some(t) = generate_random_token() { t }
			else { return empty_status(StatusCode::INTERNAL_SERVER_ERROR) };

		// insert the new row, with the email and a non-null validation token
		info!("opening");
		let conn =
			if let Ok(c) = Connection::open("primer.db") { c }
			else { return empty_status(StatusCode::INTERNAL_SERVER_ERROR) };

		info!("execute");
		let rows_affected =
			match conn.execute(
				"INSERT INTO users (email, validation_token) VALUES (?1, ?2)",
				params![&r.email, &validation_token],
			) {
				Ok(rows) => rows,
				Err(SqlError::SqliteFailure(SqlErrorStruct{ code: SqlErrorCode::ConstraintViolation, .. }, _)) => {
					return empty_status(StatusCode::NO_CONTENT)
				},
				_ => {
					return empty_status(StatusCode::INTERNAL_SERVER_ERROR)
				}
			};

		info!("execute");
		println!("{:?}", rows_affected);
		// create a validation url
		// %s%s/recover-password?t=%s`, serverProtocol, serverDomain, validationToken

		// create body

		// send validation email

		empty_status(StatusCode::NO_CONTENT)
	}))
}


fn email_service(req: Request<Body>) -> ResponseFuture {
	match (req.method(), req.uri().path()) {
		(&Method::POST, "/new-email") => handle_new_email(req),
		// (&Method::POST, "/validate") => Box::new(handle_validate(req)),
		_ => Box::new(future::ok(empty_status(StatusCode::NOT_FOUND)))
	}
}
