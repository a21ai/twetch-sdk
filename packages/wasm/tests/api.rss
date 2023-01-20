#[cfg(test)]
mod api_tests {
    use twetch_sdk::api::Api;

    #[tokio::test]
    async fn pubkeys() {
        let token = "".to_string();
        let api = Api { token };

        let response = api
            .list_pubkeys(
                [
                    "1".to_string(),
                    "2".to_string(),
                    "3".to_string(),
                    "4".to_string(),
                ]
                .to_vec(),
            )
            .await;

        println!("{:?}", response);
    }
}
