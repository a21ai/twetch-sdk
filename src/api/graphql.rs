use anyhow::Result;
use serde_json::{json, Value};

pub struct GraphqlApi {
    pub url: String,
    pub token: String,
}

impl GraphqlApi {
    pub fn new(url: String, token: String) -> GraphqlApi {
        GraphqlApi { url, token }
    }

    pub fn client(&self) -> reqwest::RequestBuilder {
        let client = reqwest::Client::new();
        client
            .post(&self.url)
            .header("Authorization", format!("Bearer {}", self.token))
    }

    pub async fn graphql(&self, query: String, variables: Option<Value>) -> Result<Value> {
        let payload = json!({
            "operationName": null,
            "variables": variables,
            "query": query
        });

        println!("api payload {:?}", payload);

        let res = self
            .client()
            .json(&payload)
            .send()
            .await?
            .json::<Value>()
            .await?;

        println!("res {:?}", res);

        let data = res.get("data").unwrap().clone();

        println!("data {:?}", data);

        Ok(data)
    }

    pub async fn create_message(&self, payload: Value) -> Result<Value> {
        let query = format!(
            "mutation createMessage($payload: MessageInput!) {{ createMessage(input: {{ message: $payload }}) {{ messageEdge {{ node {{ id }} }} }} }}",
        );

        let res = self
            .graphql(query, Some(payload))
            .await?
            .get("createMessage")
            .unwrap()
            .clone();
        Ok(res)
    }

    pub async fn create_conversation(&self, payload: String) -> Result<Value> {
        let query = format!(
            "mutation createConversation($payload: String!) {{ createConversation(input: {{ payload: $payload }}) {{ id }} }}",
        );

        let res = self
            .graphql(
                query,
                Some(json!({ "payload": serde_json::to_value(payload).unwrap() })),
            )
            .await?
            .get("createConversation")
            .unwrap()
            .clone();
        Ok(res)
    }

    pub async fn list_pubkeys(&self, user_ids: Vec<String>) -> Result<Value> {
        let user_ids_string = serde_json::to_value(user_ids).unwrap().to_string();
        let query = format!(
            "query ListPubkeys {{ allUsers(filter: {{ id: {{ in: {} }} }}) {{ nodes {{ publicKey }} }} }} ",
            user_ids_string
        );
        let res = self
            .graphql(query, None)
            .await?
            .get("allUsers")
            .unwrap()
            .get("nodes")
            .unwrap()
            .clone();
        Ok(res)
    }
}
