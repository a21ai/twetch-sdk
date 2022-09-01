#[cfg(test)]
mod authentication_tests {
    use anyhow::Result;
    use twetch_sdk::AuthToken;

    #[test]
    fn auth_token() -> Result<()> {
        let token = AuthToken::new("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjp7ImlkIjoiODA1MTUifSwiaWF0IjoxNjU5NjQ1NTMxfQ.W1ZpuZOdt7i2vXcuAoYrxw3PqAynIsLKwOcsUlH9v7M".to_string())?;
        assert_eq!(token.user_id, "80515".to_string());
        Ok(())
    }
}
