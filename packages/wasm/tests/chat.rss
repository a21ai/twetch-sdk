#[cfg(test)]
mod chat_tests {
    use twetch_sdk::chat::conversation::Conversation;

    #[tokio::test]
    async fn create_conversation() {
        let token = "".to_string();

        println!("token {:?}", token);

        let conversation = Conversation::create(token.clone(), ["1".to_string()].to_vec())
            .await
            .unwrap();

        println!("{:?}", conversation);

        let message = conversation
            .create_message(token.clone(), "hello world".to_string())
            .await
            .unwrap();

        println!("{:?}", message);
    }
}
