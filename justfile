dev:
	curl http://localhost:5050/5/yo

test_signup:
	curl http://localhost:5050/subscribe -v -H "Content-Type: application/json" \
		-d '{ "email": "dude@gmail.com" }'

test_verify:
	curl http://localhost:5050/verify -v -H "Content-Type: application/json" \
		-d '{ "validation_token": "hAAmluFDdEuU5BVueKmPNaz5mrzMjv8R4AlfNOPpYjFUKiOkK6oqVgxXPMusum0nSevP6wAW9n2XJZ9JLMh7Zg==" }'

test_unsubscribe:
	curl http://localhost:5050/unsubscribe -v -H "Content-Type: application/json" \
		-d '{ "email": "ZHVkZUBnbWFpbC5jb20=", "unsubscribed_with": "xqhScscXPNsDpQXEi0hiuqiFQsuGZyeKkyK2Bi+Wg5lrsE5T8rdVcLvAUS2OGo3ORNpNY9ReWlKRXpPblNocFw==" }'
