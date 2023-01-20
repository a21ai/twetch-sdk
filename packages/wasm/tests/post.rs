#[cfg(test)]
mod post_tests {
    extern crate wasm_bindgen_test;

    use twetch_sdk::post::{commands::PayCommand, Post};
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!();

    #[test]
    #[wasm_bindgen_test]
    fn estimate_usd_test() {
        let exchange_rate = 100.00f64;

        assert_eq!(
            Post::from_description("hello world".to_string()).estimate_usd(exchange_rate),
            0.02_f64
        );
        assert_eq!(
            Post::from_description("some words in front /pay @1 $1 some words in back".to_string())
                .estimate_usd(exchange_rate),
            1.02_f64
        );
        assert_eq!(
            Post::from_description("/pay @1 $hbeckeri harry@twetch.com $2.18".to_string())
                .estimate_usd(exchange_rate),
            2.21_f64
        );
        assert_eq!(
            Post::from_description("/pay @1 $hbeckeri harry@twetch.com 1 BSV".to_string())
                .estimate_usd(exchange_rate),
            100.02_f64
        );
        assert_eq!(
            PayCommand::from_string(&"/pay @1 $hbeckeri harry@twetch.com 1 BSV".to_string())
                .unwrap()
                .users,
            ["@1", "$hbeckeri", "harry@twetch.com"]
        );
        assert_eq!(
            PayCommand::from_string(&"/pay @1 @2 @3 1 BSV".to_string())
                .unwrap()
                .users,
            ["@1", "@2", "@3"]
        );
        assert_eq!(
            Post::from_description("@1 @2 @3 @4".to_string()).estimate_usd(exchange_rate),
            0.04_f64
        );
    }
}
