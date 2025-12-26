use httpclient::{Client, InMemoryError};
use plaid::model::{CountryCode, LinkTokenCreateRequestUser, Products};
use plaid::request::link_token_create::LinkTokenCreateRequired;
use std::str::FromStr;
use strum_macros::{Display, EnumString};
pub struct PlaidClient {
    client: plaid::PlaidClient,
    pub name: String,
}

#[derive(Debug, EnumString, Display, PartialEq)]
#[strum(serialize_all = "snake_case")]
pub enum PlaidEnv {
    Sandbox,
    Production,
}

pub struct PlaidLinkTokenError {
    pub message: String,
}
impl PlaidClient {
    pub fn new(env: PlaidEnv, name: String, client_id: String, secret: String) -> Self {
        // Format the base URL
        let base_url = format!("https://{}.plaid.com", env.to_string());

        let client = Client::new().base_url(base_url.as_str());
        let auth = plaid::PlaidAuth::ClientId {
            client_id,
            secret,
            version: "2020-09-14".to_string(),
        };
        let plaid_client = plaid::PlaidClient::new(client, auth);
        Self {
            client: plaid_client,
            name: name,
        }
    }

    pub async fn get_link_token_for_transactions(
        &self,
        username: String,
    ) -> Result<String, PlaidLinkTokenError> {
        let link_token_response = self
            .client
            .link_token_create(LinkTokenCreateRequired {
                client_name: "Personal Budget",
                country_codes: vec![CountryCode::Us],
                language: "en",
                user: LinkTokenCreateRequestUser {
                    address: None,
                    client_user_id: username,
                    date_of_birth: None,
                    email_address: None,
                    email_address_verified_time: None,
                    id_number: None,
                    legal_name: None,
                    name: None,
                    phone_number: None,
                    phone_number_verified_time: None,
                    ssn: None,
                },
            })
            .products(vec![Products::Transactions])
            .await;
        link_token_response
            .map_err(|_| PlaidLinkTokenError {
                message: "Failed to create link token".to_string(),
            })
            .map(|r| r.link_token)
    }
}
