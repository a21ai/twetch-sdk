import { Wallet, Networks, TxBuilder } from '../../../pkg/node'
import { assert, util } from 'chai'

import fetch, {Headers} from 'node-fetch'

if (!globalThis.fetch) {
  globalThis.fetch = fetch
  globalThis.Headers = Headers
}

const SEED = 'book fit fly ketchup also elevator scout mind edit fatal where rookie'

describe('Wallet', function () {
	it('wallet_xpub', () => {
		const wallet = new Wallet(SEED)

		assert.equal(
			wallet.wallet_xpub().to_string(),
			'xpub6Dv27qDns95XnA2rjSunSvYwaJa3z614xq79X3rMYRzpZLjnb6j6La1rvoXLLvrBWC5BABJ6pBSnkdspj94g262wRh8M4MQmEiMSQ9s7QDn'
		)
		assert.equal(
			wallet.xpub().to_string(),
			'xpub661MyMwAqRbcFYHX2uE6zQHyLdBvVdv5tddZi5nRApwmrzPpeGQb5zJFfp8jMxqv4HpYMZ8Xre7WaEfNK2612a7wTY21ASzDuoXfXHRJHXG'
		)
		assert.equal(
			wallet.account_public_key().to_hex(),
			'02dbe5e772d01b7bd461e3cb4960afabe7e3db6652e9eec7b70b8c10c4baf29e64'
		)
		assert.equal(wallet.account_address().to_string(), '199sSk4VNnoWM4AMRUVdhPLL9jBBrRbWzN')
		assert.equal(wallet.display_address(Networks.BSV), '199sSk4VNnoWM4AMRUVdhPLL9jBBrRbWzN')
		assert.equal(wallet.display_address(Networks.TBSV), 'mofpjo9UBpEm8Ady93U1XJYf1imtp7Qbin')
	})

	it('tx_builder', async () => {
		const wallet = new Wallet(SEED)

		console.log({ wallet })

		const payload = {
			network: Networks.BSV,
			outputs: [
				{
					sats: 100,
					to: '1',
					encrypt_args: false
				}
			],
			resolve_change: false,
			auto_fund: false
		}

		const builder = new TxBuilder()
		const added = builder.add_output(
				{
					sats: 100,
					to: '1',
					encrypt_args: false
				}
			);

		console.log({ builder, added })

		const response = await TxBuilder.build(builder, wallet)

		console.log({ response })
	})
})

//#[tokio::test]
//async fn tx_builder() -> Result<()> {
//let wallet = Wallet::new(SEED.to_string());

//let builder = TxBuilder {
//network: Networks::BSV,
//contract: None,
//contract_sats: None,
//extended_tx: None,
//outputs: vec![
//TxBuilderOutput {
//sats: 100,
//address: None,
//to: Some("@1".to_string()),
//args: None,
//encrypt_args: false,
//},
//TxBuilderOutput {
//sats: 100,
//address: None,
//to: Some("1".to_string()),
//args: None,
//encrypt_args: false,
//},
//TxBuilderOutput {
//sats: 100,
//address: None,
//to: Some("12tDncQvFZaZzqanupmtXpDUm42Wd4Cn4W".to_string()),
//args: None,
//encrypt_args: false,
//},
//TxBuilderOutput {
//sats: 100,
//address: None,
//to: Some("harry@relayx.io".to_string()),
//args: None,
//encrypt_args: false,
//},
//TxBuilderOutput {
//sats: 100,
//address: None,
//to: Some("type@handcash.io".to_string()),
//args: None,
//encrypt_args: false,
//},
//],
//change_address: None,
//resolve_change: true,
//auto_fund: false,
//};

//let built = wallet.build_tx(&builder).await?;
//let tx = built.tx;

//assert_eq!(
//tx.get_output(0).unwrap().get_script_pub_key_hex(),
//"76a91414a8036c8b3d910a7e24d46067048d8761274b5588ac"
//);
//assert_eq!(
//tx.get_output(1).unwrap().get_script_pub_key_hex(),
//"76a91414a8036c8b3d910a7e24d46067048d8761274b5588ac"
//);
//assert_eq!(
//tx.get_output(2).unwrap().get_script_pub_key_hex(),
//"76a91414a8036c8b3d910a7e24d46067048d8761274b5588ac"
//);
//assert_eq!(
//tx.get_output(3).unwrap().get_script_pub_key_hex(),
//"76a91414a8036c8b3d910a7e24d46067048d8761274b5588ac"
//);
//assert_eq!(tx.get_output(4).unwrap().get_script_pub_key_hex().len(), 50);
//assert_eq!(built.payment_destinations.len(), 1);
