pub struct AuthApi {
    pub url: String,
}

impl AuthApi {
    pub fn new(url: String) -> AuthApi {
        AuthApi { url }
    }
}
