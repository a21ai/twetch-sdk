#[cfg(test)]
mod authentication_tests {
    extern crate wasm_bindgen_test;
    use std::str::from_utf8;

    use twetch_sdk::authentication::Authentication;
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!();

  #[test]
  #[wasm_bindgen_test]
  fn get_cipher_test() {
    let email = "debug@twetch.com".to_string();
    let password = "stronk-password".to_string();

    let response = Authentication::get_cipher(email, password);

    assert_eq!(response.get_cipher(), "f064d740b65941152755829e2b48578b259bc9bfc8c3af7b0d93a5ca677f259d");
    assert_eq!(response.get_email_hash(), "1ae0ee429ffca864413b59edd5612c1a86b097411280a6dfa376d91c6eba5a20");
    assert_eq!(response.get_password_hash(), "73e011ce27c1f00ab11ac306f9eefd5091ef65de8dc67876eda65a5926e7f849");
  }
}

