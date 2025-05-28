[![Rust](https://img.shields.io/badge/Rust-%23000000.svg?e&logo=rust&logoColor=blue)](#)
[![Keycloak](https://img.shields.io/badge/Keycloak-4D4D4D?logo=keycloak&logoColor=fff&style=flat)](#)
[![Json Web Tokens](https://img.shields.io/badge/json%20web%20tokens-323330?logo=json-web-tokens&logoColor=pink)](#)
[![Docker](https://img.shields.io/badge/Docker-2496ED?logo=docker&logoColor=fff)](#)
[![Git](https://img.shields.io/badge/Git-F05032?logo=git&logoColor=fff)](#)

# Keycloak Integration Example

This project demonstrates integration with Keycloak for authentication and authorization using Rust and Actix-web framework.

## API Endpoints

### Authentication Flow

#### 1. Login Endpoint
- **URL:** `/login`
- **Method:** `GET`
- **Description:** Initiates the authentication flow by redirecting to Keycloak login page
- **Response:** Redirects to Keycloak authentication page

#### 2. Callback Endpoint
- **URL:** `/callback`
- **Method:** `GET`
- **Query Parameters:**
  - `code`: Authorization code from Keycloak
  - `session_state`: Session state
  - `iss`: (Optional) Issuer
- **Description:** Handles the OAuth2 callback from Keycloak
- **Response:**
  - Sets `access_token` cookie
  - Returns HTML page with token information

### Protected Routes

All protected routes are under `/api/v1` and require Bearer token authentication.

#### Protected Resource
- **URL:** `/api/v1/protected`
- **Method:** `GET`
- **Headers Required:**
  - `Authorization: Bearer <access_token>`
- **Description:** Example of a protected resource
- **Response:**
  - Success (200): Returns "Access to protected resource granted!"
  - Unauthorized (401): When token is invalid or missing

#### Logout
- **URL:** `/api/v1/logout`
- **Method:** `POST`
- **Headers Required:**
  - `Authorization: Bearer <access_token>`
- **Description:** Ends the user session
- **Response:**
  - Success (302): Redirects to `/login`
  - Error (400): When token is missing
  - Error (500): When logout fails
