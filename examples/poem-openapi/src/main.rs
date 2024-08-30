use passage_flex::PassageFlex;
use poem::{listener::TcpListener, Route};
use poem_openapi::{payload::Json, Object, OpenApi, OpenApiService};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

// Represents a user in the system
#[derive(Debug, Serialize, Deserialize, Object)]
struct User {
    /// A unique immutable string to identify the user
    id: String,
    /// The email address of the user
    email: String,
    /// The authentication token for this user (if authenticated)
    auth_token: Option<String>,
}

// Request payload for user registration
#[derive(Debug, Deserialize, Object)]
struct RegisterRequest {
    /// The email address of the user to register
    email: String,
}

// Request payload for user authentication
#[derive(Debug, Deserialize, Object)]
struct AuthenticateRequest {
    /// The email address of the user to authenticate
    email: String,
}

// Response payload containing the transaction ID
#[derive(Debug, Serialize, Object)]
struct TransactionResponse {
    /// The transaction ID returned by Passage
    transaction_id: String,
}

// Request payload for nonce verification
#[derive(Debug, Deserialize, Object)]
struct VerifyRequest {
    /// The nonce to verify
    nonce: String,
}

// Response payload containing the auth token
#[derive(Debug, Serialize, Object)]
struct VerifyResponse {
    /// The authentication token for the user
    auth_token: String,
}

// Request query parameters simulating an authenticated user
#[derive(Debug, Deserialize, Object)]
struct AuthenticatedRequest {
    /// The authentication token of the user
    auth_token: String,
}

// Response payload containing the list of passkeys for a user
#[derive(Debug, Serialize, Object)]
struct UserPasskeysResponse {
    /// The list of passkeys registered for the user
    passkeys: Vec<Passkey>,
}

// Request payload for revoking a passkey
#[derive(Debug, Deserialize, Object)]
struct RevokePasskeyRequest {
    /// The ID of the passkey to revoke
    passkey_id: String,
}

// Represents a passkey registered by a user
#[derive(Debug, Serialize, Deserialize, Object)]
struct Passkey {
    /// The display name for the passkey
    pub friendly_name: String,
    /// The ID of the passkey
    pub id: String,
}

// Shared state for managing users (in-memory for simplicity)
struct AppState {
    users: Mutex<Vec<User>>,
    passage_client: PassageFlex, // Passage client instance
}

// The API struct containing all the endpoints
struct Api {
    state: Arc<AppState>,
}

#[OpenApi]
impl Api {
    /// Register a new user
    #[oai(path = "/auth/register", method = "post")]
    async fn register_user(
        &self,
        req: Json<RegisterRequest>,
    ) -> poem::Result<Json<TransactionResponse>> {
        // Generate a unique identifier for the user
        let user_id = Uuid::new_v4().to_string();

        // Create and store the user
        let user = User {
            email: req.0.email.clone(),
            id: user_id.clone(),
            auth_token: None,
        };

        // Store the user in shared state (simulates saving to a database)
        let mut users = self.state.users.lock().await;
        users.push(user);

        // Create a register transaction using the Passage SDK
        let transaction_id = self
            .state
            .passage_client
            .create_register_transaction(user_id.clone(), req.0.email.clone())
            .await
            .map_err(|e| {
                poem::Error::from_string(
                    format!("Passage SDK error: {}", e),
                    poem::http::StatusCode::INTERNAL_SERVER_ERROR,
                )
            })?; // Convert SDK error into poem error

        // Return the transaction ID in the response
        Ok(Json(TransactionResponse { transaction_id }))
    }

    /// Authenticate an existing user
    #[oai(path = "/auth/login", method = "post")]
    async fn authenticate_user(
        &self,
        req: Json<AuthenticateRequest>,
    ) -> poem::Result<Json<TransactionResponse>> {
        // Simulate finding the user in the database
        let user_email = req.0.email.clone();
        // In a real implementation, you'd query your database here
        // For simplicity, we'll just search the in-memory list
        let user_id = {
            let users = self.state.users.lock().await;
            users
                .iter()
                .find(|user| user.email == user_email)
                .map(|user| user.id.clone())
        };

        if let Some(user_id) = user_id {
            // Create an authentication transaction using the Passage SDK
            let transaction_id = self
                .state
                .passage_client
                .create_authenticate_transaction(user_id)
                .await
                .map_err(|e| match e {
                    passage_flex::Error::UserHasNoPasskeys => poem::Error::from_string(
                        "User has no passkeys".to_owned(),
                        poem::http::StatusCode::BAD_REQUEST,
                    ),
                    _ => poem::Error::from_string(
                        "Internal server error".to_owned(),
                        poem::http::StatusCode::INTERNAL_SERVER_ERROR,
                    ),
                })?;
            Ok(Json(TransactionResponse { transaction_id }))
        } else {
            Err(poem::Error::from_string(
                "User not found".to_owned(),
                poem::http::StatusCode::NOT_FOUND,
            ))
        }
    }

    /// Verify the nonce received from Passage and generate an auth token
    #[oai(path = "/auth/verify", method = "post")]
    async fn verify_nonce(&self, req: Json<VerifyRequest>) -> poem::Result<Json<VerifyResponse>> {
        // Verify the nonce using the Passage SDK
        match self.state.passage_client.verify_nonce(req.0.nonce).await {
            Ok(user_id) => {
                // Find the user in the database and set the auth token
                let mut users = self.state.users.lock().await;
                if let Some(user) = users.iter_mut().find(|user| user.id == user_id) {
                    // In a real implementation, you'd generate a secure token here
                    // For simplicity, we'll just use a plaintext UUID
                    let auth_token = Uuid::new_v4().to_string();

                    // Set the user auth token
                    user.auth_token = Some(auth_token.clone());

                    // Return the auth token
                    Ok(Json(VerifyResponse { auth_token }))
                } else {
                    Err(poem::Error::from_string(
                        "User not found".to_owned(),
                        poem::http::StatusCode::NOT_FOUND,
                    ))
                }
            }
            Err(e) => Err(poem::Error::from_string(
                format!("Verification error: {}", e),
                poem::http::StatusCode::UNAUTHORIZED,
            )),
        }
    }

    /// Add a new passkey for an authenticated user
    #[oai(path = "/user/passkeys/add", method = "post")]
    async fn add_passkey(
        &self,
        query: poem::web::Query<AuthenticatedRequest>,
    ) -> poem::Result<Json<TransactionResponse>> {
        // Verify user identity by finding the user based on auth_token
        let users = self.state.users.lock().await;
        if let Some(user) = users.iter().find(|user| {
            user.auth_token
                .as_ref()
                .is_some_and(|t| *t == query.auth_token)
        }) {
            // Create a register transaction using the Passage SDK
            let transaction_id = self
                .state
                .passage_client
                .create_register_transaction(user.id.clone(), user.email.clone())
                .await
                .map_err(|e| {
                    poem::Error::from_string(
                        format!("Failed to create register transaction: {}", e),
                        poem::http::StatusCode::INTERNAL_SERVER_ERROR,
                    )
                })?;

            // Return the transaction ID to the client
            Ok(Json(TransactionResponse { transaction_id }))
        } else {
            Err(poem::Error::from_string(
                "Unauthorized".to_owned(),
                poem::http::StatusCode::UNAUTHORIZED,
            ))
        }
    }

    /// Get the list of passkeys for an authenticated user
    #[oai(path = "/user/passkeys", method = "get")]
    async fn get_user_passkeys(
        &self,
        query: poem::web::Query<AuthenticatedRequest>,
    ) -> poem::Result<Json<UserPasskeysResponse>> {
        // Verify user identity by finding the user based on auth_token
        let users = self.state.users.lock().await;
        if let Some(user) = users.iter().find(|user| {
            user.auth_token
                .as_ref()
                .is_some_and(|t| *t == query.auth_token)
        }) {
            // Retrieve a list of all devices used to register a passkey
            let passkeys = self
                .state
                .passage_client
                .get_devices(user.id.clone())
                .await
                .map(|devices| {
                    devices
                        .into_iter()
                        .map(|device| Passkey {
                            friendly_name: device.friendly_name,
                            id: device.id,
                        })
                        .collect()
                })
                .map_err(|e| {
                    poem::Error::from_string(
                        format!("Failed to retrieve passkeys: {}", e),
                        poem::http::StatusCode::INTERNAL_SERVER_ERROR,
                    )
                })?;

            // Return the list of passkeys to the client
            Ok(Json(UserPasskeysResponse { passkeys }))
        } else {
            Err(poem::Error::from_string(
                "Unauthorized".to_owned(),
                poem::http::StatusCode::UNAUTHORIZED,
            ))
        }
    }

    /// Revoke a passkey for an authenticated user
    #[oai(path = "/user/passkeys/revoke", method = "post")]
    async fn revoke_passkey(
        &self,
        query: poem::web::Query<AuthenticatedRequest>,
        req: Json<RevokePasskeyRequest>,
    ) -> poem::Result<poem_openapi::payload::PlainText<String>> {
        // Verify user identity by finding the user based on auth_token
        let users = self.state.users.lock().await;
        if let Some(user) = users.iter().find(|user| {
            user.auth_token
                .as_ref()
                .is_some_and(|t| *t == query.auth_token)
        }) {
            // Revoke the passkey device using the Passage SDK
            self.state
                .passage_client
                .revoke_device(user.id.clone(), req.0.passkey_id.clone())
                .await
                .map_err(|e| {
                    poem::Error::from_string(
                        format!("Failed to revoke passkey: {}", e),
                        poem::http::StatusCode::INTERNAL_SERVER_ERROR,
                    )
                })?;

            // Return a success response
            Ok(poem_openapi::payload::PlainText("OK".to_string()))
        } else {
            // If the user is not found or unauthorized, return a 401 error
            Err(poem::Error::from_string(
                "Unauthorized".to_owned(),
                poem::http::StatusCode::UNAUTHORIZED,
            ))
        }
    }
}

// Main function to start the server
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Initialize the PassageFlex client
    let passage_client = PassageFlex::new(
        std::env::var("PASSAGE_APP_ID").expect("PASSAGE_APP_ID required"),
        std::env::var("PASSAGE_API_KEY").expect("PASSAGE_API_KEY required"),
    );

    // Initialize shared application state
    let state = Arc::new(AppState {
        users: Mutex::new(Vec::new()),
        passage_client,
    });

    // Create API service
    let api_service =
        OpenApiService::new(Api { state }, "Passkey API", "1.0").server("http://localhost:3000");

    // Create the app route at the root
    let app = Route::new().nest("/", api_service);

    // Run the server
    poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
