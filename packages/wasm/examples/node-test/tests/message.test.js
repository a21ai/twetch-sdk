import { BSM, PrivateKey, P2PKHAddress } from '../../../pkg/node'

import { assert, util } from 'chai'

import Message from 'bsv/message'
import bsv from 'bsv'

const packUInt16LE = (num) => {
	const buf = new Buffer(2)
	buf.writeUInt16LE(num, 0)
	return buf
}

const varint = (n) => {
	if (n < 0xfd) return new Buffer([n])
	else if (n <= 0xffff) {
		let buff = new Buffer(3)
		buff[0] = 0xfd
		buff.writeUInt16LE(n, 1)
		return buff
	} else if (n <= 0xffffffff) {
		let buff = new Buffer(5)
		buff[0] = 0xfe
		buff.writeUInt32LE(n, 1)
		return buff
	} else {
		let buff = new Buffer(9)
		buff[0] = 0xff
		packUInt16LE(n).copy(buff, 1)
		return buff
	}
}

describe('Message', function () {
	it('bitcoin-signed-message', () => {
		const message = 'Hello World'

		const priv_js = new bsv.PrivateKey.fromString(
			'L1BSMMgzBFNks4F4MWBzSya3duwPdd6crGyHsGxXV52bu6fTA37E'
		)
		const pub_js = priv_js.toPublicKey()
		const address_js = priv_js.toAddress()

		const priv_wasm = PrivateKey.fromWIF(priv_js.toString())
		const pub_wasm = priv_wasm.getPublicKey()
		const address_wasm = P2PKHAddress.fromPubKey(pub_wasm)

		const bitcoinSignedMessage = (message) => {
			const signature = BSM.signMessage(priv_wasm, Buffer.from(message, 'utf8')).toCompactBytes()
			return Buffer.from(signature, 'hex').toString('base64');
		}

		const signature_wasm = bitcoinSignedMessage(message);
		const signature_js = Message.sign(message, priv_js);

		assert.equal(pub_js.toString(), pub_wasm.toHex())
		assert.equal(address_js.toString(), address_wasm.toString())
		assert.equal(signature_js, signature_wasm)
	})
})
