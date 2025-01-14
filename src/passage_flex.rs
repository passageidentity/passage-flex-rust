use crate::auth::Auth;
use crate::openapi::apis::configuration::Configuration;
use crate::user::User;

pub struct PassageFlex {
    app_id: String,
    pub auth: Auth,
    pub user: User,
}

const SERVER_URL: &str = "https://api.passage.id";

impl PassageFlex {
    /// Creates a new instance of the `PassageFlex` client.
    ///
    /// # Arguments
    ///
    /// * `app_id` - The Passage application ID.
    /// * `api_key` - The Passage API key.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use passage_flex::PassageFlex;
    ///
    /// let passage_flex = PassageFlex::new(
    ///     std::env::var("PASSAGE_APP_ID").unwrap(),
    ///     std::env::var("PASSAGE_API_KEY").unwrap(),
    /// );
    /// ```
    pub fn new(app_id: String, api_key: String) -> Self {
        if app_id.is_empty() {
            panic!("A Passage App ID is required. Please include (app_id: YOUR_APP_ID, api_key: YOUR_APP_ID).");
        }

        if api_key.is_empty() {
            panic!("A Passage API key is required. Please include (app_id: YOUR_APP_ID, api_key: YOUR_APP_ID).");
        }

        let mut headers = reqwest::header::HeaderMap::with_capacity(1);
        headers.insert(
            "Passage-Version",
            reqwest::header::HeaderValue::from_static(concat!(
                "passage-flex-rust ",
                env!("CARGO_PKG_VERSION")
            )),
        );

        let mut configuration = Configuration::new();
        configuration.bearer_access_token = Some(api_key);
        configuration.client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to create reqwest client for Passage");

        let auth = Auth::new(configuration.clone());
        let user = User::new(configuration.clone());

        let mut client = Self { app_id, auth, user };
        // Set the default server URL
        client.set_server_url(SERVER_URL.to_string());

        client
    }

    fn set_server_url(&mut self, server_url: String) {
        self.user.configuration.base_path = format!("{}/v1/apps/{}", server_url, self.app_id);
        self.auth.configuration.base_path = format!("{}/v1/apps/{}", server_url, self.app_id);
    }
}

#[cfg(test)]
mod test;
