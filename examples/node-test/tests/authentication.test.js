import { Authentication } from '../../../pkg/node'
import { assert, util } from 'chai'

describe('Authentication', function () {
	it('getCipher matches', () => {
		const email = 'debug@twetch.com'
		const password = 'stronk-password'

		const response = Authentication.getCipher(email, password)

		assert.equal(
			response.getCipher(),
			'f064d740b65941152755829e2b48578b259bc9bfc8c3af7b0d93a5ca677f259d'
		)
		assert.equal(
			response.getEmailHash(),
			'1ae0ee429ffca864413b59edd5612c1a86b097411280a6dfa376d91c6eba5a20'
		)
		assert.equal(
			response.getPasswordHash(),
			'73e011ce27c1f00ab11ac306f9eefd5091ef65de8dc67876eda65a5926e7f849'
		)
	})
})
