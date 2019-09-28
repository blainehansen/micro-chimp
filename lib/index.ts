import { AxiosStatic, AxiosError } from 'axios'

export enum MicroChimpResult {
	SUCCESS,
	INVALID_EMAIL,
	UNPROCESSABLE,
	UNKNOWN_ERROR,
}

function convert_error(error: AxiosError) {
	console.error(error.message)

	if (error.response === undefined)
		return MicroChimpResult.UNKNOWN_ERROR

	switch (error.response.status) {
		case 400: return MicroChimpResult.INVALID_EMAIL
		case 422: return MicroChimpResult.UNPROCESSABLE
		case 500: return MicroChimpResult.UNKNOWN_ERROR
		default: return MicroChimpResult.UNKNOWN_ERROR
	}
}


export class MicroChimpClient {
	readonly site_url: string
	readonly site_name: string
	constructor(
		readonly client: AxiosStatic,
		// readonly subdomain = 'subscriptions',
	) {
		const site_url = window.location.host
		this.site_url = site_url
		this.site_name = site_url.replace(/\./g, '_').toLowerCase()
	}

	private format_url(route: 'new-email' | 'verify-email' | 'unsubscribe') {
		// return `https://${this.subdomain}.${this.site_url}/${route}`
		return `https://subscriptions.${this.site_url}/${route}`
	}

	new_email(email: string): Promise<MicroChimpResult> {
		return this.client.post(this.format_url('new-email'), {
			email,
			site_name: this.site_name
		})
			.then(() => MicroChimpResult.SUCCESS)
			.catch(convert_error)
	}

	verify_email(validation_token: string): Promise<MicroChimpResult> {
		return this.client.post(this.format_url('verify-email'), {
			validation_token,
		})
			.then(() => MicroChimpResult.SUCCESS)
			.catch(convert_error)
	}

	unsubscribe(encoded_email: string, unsubscribed_with_token: string): Promise<MicroChimpResult> {
		return this.client.post(this.format_url('unsubscribe'), {
			email: encoded_email,
			site_name: this.site_name,
			unsubscribed_with: unsubscribed_with_token,
		})
			.then(() => MicroChimpResult.SUCCESS)
			.catch(convert_error)
	}
}
