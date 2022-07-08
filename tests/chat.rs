#[cfg(test)]
mod chat_tests {
    use twetch_sdk::chat::conversation::Conversation;

    #[tokio::test]
    async fn create_conversation() {
        let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjp7ImlkIjoiMSJ9LCJpYXQiOjE2NTQ3MzY2Nzl9.9-gp1qgDuN2i3InOUj8TWLYmwyZWdBzM2yWV78uWpyQ".to_string();

        println!("token {:?}", token);

        let conversation = Conversation::create(token.clone(), ["1".to_string()].to_vec())
            .await
            .unwrap();

        println!("{:?}", conversation);

        let message = conversation
            .create_message(
                token.clone(),
                "https://myorders.co/tracking/33347406/92612903099748541400011858".to_string(),
            )
            .await
            .unwrap();

        println!("{:?}", message);
    }
}
