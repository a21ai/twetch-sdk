import { PrivateKey, PublicKey, ECIES, Script } from '../../../pkg/node'
import { assert, util } from 'chai'
import Aes from 'aes-js'

import bsv from 'bsv'

function aesCBCEncrypt(plainText, kE, iV) {
	const plainBytes = Aes.padding.pkcs7.pad(Aes.utils.hex.toBytes(plainText))
	const aes = new Aes.ModeOfOperation.cbc(kE, iV)
	const encryptedBytes = aes.encrypt(plainBytes)
	const encryptedHex = Aes.utils.hex.fromBytes(encryptedBytes)
	return encryptedHex
}

function aesCBCDecrypt(encryptedHex, kE, iV) {
	const encryptedBytes = Aes.utils.hex.toBytes(encryptedHex).slice(37, -32)
	const aes = new Aes.ModeOfOperation.cbc(kE, iV)
	const plainBytes = Aes.padding.pkcs7.strip(aes.decrypt(encryptedBytes))
	const plainText = Aes.utils.hex.fromBytes(plainBytes)
	return plainText
}

function eciesEphemeralEncrypt(plainText, publicKey, r) {
	const rN = r.bn
	const k = bsv.PublicKey(publicKey).point
	const P = k.mul(rN)
	const hash = bsv.crypto.Hash.sha512(bsv.PublicKey(P).toBuffer())
	const iV = hash.slice(0, 16)
	const kE = hash.slice(16, 32)
	const kM = hash.slice(32, 64)
	const encryptedText = aesCBCEncrypt(plainText, kE, iV)
	const encryptedBytes = Buffer.from(encryptedText, 'hex')
	const msgBuf = Buffer.concat([Buffer.from('BIE1'), r.publicKey.toDER(true), encryptedBytes])
	const hmac = bsv.crypto.Hash.sha256hmac(msgBuf, kM)
	return {
		cipherText_js: Buffer.concat([msgBuf, hmac]).toString('hex'),
		hash_js: hash.toString('hex')
	}
}

function eciesEphemeralDecrypt(encryptedHex, hash) {
	const buf = Buffer.from(hash, 'hex')
	const iV = buf.slice(0, 16)
	const kE = buf.slice(16, 32)

	return aesCBCDecrypt(encryptedHex, kE, iV)
}

describe('encryption', () => {
	it('eciesEphemeralEncrypt', () => {
		//const args = [
		//'1LoveF7qQijpjascPytHor2uSEEjHHH8YB',
		//'1447f87d79e395f75e3cd5ef7edf822fbf4aaf1c2e8b22f1cf8791575bb756bf',
		//'twetch',
		//'8e48e49e-3332-4a52-b69d-016ecd006b0a',
		//'|',
		//'15PciHG22SNLQJXMoSUaWVi7WSqc7hCfva',
		//'BITCOIN_ECDSA',
		//'12tDncQvFZaZzqanupmtXpDUm42Wd4Cn4W',
		//'IMCdzg9LJfrDNJdFPB1qhd5nCIjohz+rikE6cSDk9K/mBFZhRluA7C9vwDrbuwF02JQgkiMXl4NungrnoUeJMrY='
		//]
		const args = [
			'19HxigV4QyBv3tHpQVcUEQyq1pzZVdoAut',
			'sup',
			'text/plain',
			'text',
			'twetch_twtext_1628731249452.txt',
			'|',
			'1PuQa7K62MiKCtssSLKy1kh56WWU7MtUR5',
			'SET',
			'twdata_json',
			'null',
			'url',
			'null',
			'comment',
			'null',
			'mb_user',
			'null',
			'reply',
			'null',
			'type',
			'post',
			'timestamp',
			'null',
			'app',
			'twetch',
			'invoice',
			'a637579f-37d2-4e38-b40c-b95f469ef4f8',
			'|',
			'15PciHG22SNLQJXMoSUaWVi7WSqc7hCfva',
			'BITCOIN_ECDSA',
			'12tDncQvFZaZzqanupmtXpDUm42Wd4Cn4W',
			'H8G5Siy7lgWO9xNaVmz/Z2LNrHk53Gu1BlTagT4BkcGrcJZlM34gP5ADhCASnFiYxo+O+R1EqMfLbqjgAm0m9lU='
		]
		const randPriv_js = bsv.PrivateKey.fromRandom()
		const wif = 'L1BSMMgzBFNks4F4MWBzSya3duwPdd6crGyHsGxXV52bu6fTA37E'

		const start_js = new Date()

		const script_js = new bsv.Script()
		for (let each of args) {
			script_js.add(Buffer.from(each))
		}
		const scriptHex_js = script_js.toHex()
		const priv_js = new bsv.PrivateKey.fromString(wif)
		const pub_js = priv_js.toPublicKey()
		const { cipherText_js, hash_js } = eciesEphemeralEncrypt(scriptHex_js, pub_js, randPriv_js)

		const end_js = new Date()

		const asm_wasm = args.map((e) => Buffer.from(e).toString('hex')).join(' ')
		const scriptHex_wasm = Buffer.from(Script.fromASMString(asm_wasm).toHex(), 'hex')
		const priv_wasm = PrivateKey.fromWIF(wif)
		const pub_wasm = priv_wasm.getPublicKey()
		const randPriv_wasm = PrivateKey.fromWIF(randPriv_js.toString())
		const cipherText = ECIES.encrypt(scriptHex_wasm, randPriv_wasm, pub_wasm, false)
		const cipherKeys = ECIES.deriveCipherKeys(randPriv_wasm, pub_wasm)
		const hash_wasm = Buffer.concat([
			cipherKeys.get_iv(),
			cipherKeys.get_ke(),
			cipherKeys.get_km()
		]).toString('hex')
		const cipherText_wasm = Buffer.from(cipherText.toBytes()).toString('hex')

		const end_wasm = new Date()

		console.log(`js runtime: ${end_js.getTime() - start_js.getTime()}ms`)
		console.log(`wasm runtime: ${end_wasm.getTime() - end_js.getTime()}ms`)

		const asm = `0 OP_RETURN 747765746368 ${cipherText_wasm}`
		const final_script_js = bsv.Script.fromASM(asm)
		const final_script_wasm = Script.fromASMString(asm).toHex()

		const encryptedHex = final_script_js.chunks[3].buf.toString('hex')
		const buf = Buffer.from(hash_wasm, 'hex')
		const iV = buf.slice(0, 16)
		const kE = buf.slice(16, 32)

		const decryptedHex = aesCBCDecrypt(encryptedHex, kE, iV)

		assert.equal(priv_js.toString(), priv_wasm.toWIF())
		assert.equal(randPriv_js.toString(), randPriv_wasm.toWIF())
		assert.equal(pub_js.toString(), pub_wasm.toHex())
		assert.equal(cipherText_js, cipherText_wasm)
		assert.equal(hash_js, hash_wasm)
		assert.equal(scriptHex_js, scriptHex_wasm.toString('hex'))
		assert.equal(script_js.toASM(), asm_wasm)
		assert.equal(final_script_wasm, final_script_js.toHex())
		assert.equal(encryptedHex, cipherText_js)
		assert.equal(encryptedHex, cipherText_wasm)
		assert.equal(decryptedHex, scriptHex_js)
		assert.equal(decryptedHex, scriptHex_wasm.toString('hex'))
	})
})
