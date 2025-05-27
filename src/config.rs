use std::env;

pub struct Config {
    pub port: String,
    pub keyclock_base_url: String,
    pub keyclock_realm: String,
    pub keyclock_client_id: String,
    pub redirect_uri: String,
    pub client_secret: String,
}

impl Default for Config {
    fn default() -> Self {
        let port = env::var("PORT").expect("PORT must be set");
        let keyclock_base_url =
            env::var("KEYCLOAK_BASE_URL").expect("KEYCLOAK_BASE_URL must be set");
        let keyclock_realm = env::var("KEYCLOAK_REALM").expect("KEYCLOAK_REALM must be set");
        let keyclock_client_id =
            env::var("KEYCLOAK_CLIENT_ID").expect("KEYCLOAK_CLIENT_ID must be set");
        let redirect_uri = env::var("REDIRECT_URI").expect("REDIRECT_URI must be set");
        let client_secret =
            env::var("KEYCLOAK_CLIENT_SECRET").expect("KEYCLOAK_CLIENT_SECRET must be set");

        Self {
            port,
            keyclock_base_url,
            keyclock_realm,
            keyclock_client_id,
            redirect_uri,
            client_secret,
        }
    }
}
