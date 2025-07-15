# Rust JWT Validator with JWKS Support

This project demonstrates how to validate JWT tokens signed with RSA keys fetched dynamically from a JWKS endpoint in Rust using Actix Web.

---

## Features

- Fetch JWKS (JSON Web Key Set) from a remote HTTP endpoint
- Select the correct public key by `kid` from JWT header
- Verify JWT signature using `jsonwebtoken` crate
- Validate standard claims: `sub`, `aud`, `iss`, `exp`
- Simple Actix Web endpoint `/secure` that requires a valid JWT bearer token
---

## Prerequisites

- Rust and Cargo installed: https://rustup.rs/
- A running JWKS server exposing keys at: `http://localhost:3000/.well-known/jwks.json`
- A JWT token signed with a private key matching the JWKS public keys
---

## Running the JWKS Mock Server

Run the simple Node.js mock server:

1. Install dependencies:

   ```bash
   npm i
   ```

3. Run the server:

   ```bash
   node server.js
   ```

This will:

* Expose the JWKS at:
  `http://localhost:3000/.well-known/jwks.json`
* Provide a test JWT token at:
  `http://localhost:3000/token`
---

## Running the Rust JWT Validator Server

1. Run the Rust server:

   ```bash
   cargo run
   ```

You should see:

```
Server running at http://localhost:8080
```

---

## Testing the `/secure` Endpoint

Send an HTTP GET request with the JWT token in the `Authorization` header:

```bash
curl -X GET http://localhost:8080/secure \
  -H "Authorization: Bearer <YOUR_JWT_TOKEN>"
```

* Replace `<YOUR_JWT_TOKEN>` with a valid JWT string signed with the private key corresponding to the JWKS.
* If the token is valid, you will get a JSON response of the claims.
* If invalid or missing, you'll get `401 Unauthorized` with an error message.

---
