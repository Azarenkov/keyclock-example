use actix_web_middleware_keycloak_auth::{AlwaysReturnPolicy, DecodingKey, KeycloakAuth};
use std::env;

pub fn connect_keyclock() -> KeycloakAuth<AlwaysReturnPolicy> {
    let keycloak_pk_env =
        env::var("KEYCLOAK_PK").expect("Environment variable KEYCLOAK_PK must be set");

    let keycloak_pk = format!(
        "-----BEGIN PUBLIC KEY-----{}-----END PUBLIC KEY-----",
        keycloak_pk_env
    );

    let keycloak_auth = KeycloakAuth {
        detailed_responses: true,
        passthrough_policy: AlwaysReturnPolicy,
        keycloak_oid_public_key: DecodingKey::from_rsa_pem(keycloak_pk.as_bytes())
            .expect("Invalid RSA public key provided in KEYCLOAK_PK"),
        required_roles: vec![],
    };
    keycloak_auth
}
