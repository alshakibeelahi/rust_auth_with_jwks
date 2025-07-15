import express from 'express';
import { exportJWK, generateKeyPair, SignJWT } from 'jose';

const app = express();
const PORT = 3000;

let privateKey, publicJwk;

(async () => {
  // Generate RSA key pair
  const { publicKey, privateKey: privKey } = await generateKeyPair('RS256');
  privateKey = privKey;

  // Export public key as JWK
  publicJwk = await exportJWK(publicKey);
  publicJwk.kid = 'my-key-id'; // Assign a key ID
})();

// JWKS endpoint
app.get('/.well-known/jwks.json', (req, res) => {
  res.json({ keys: [publicJwk] });
});

// Token endpoint
app.get('/token', async (req, res) => {
  const jwt = await new SignJWT({
  "sub": "1234567890",
  "aud": "your-app",
  "iss": "your-auth-server",
  "exp": 1721100000,
  "role": "admin",
  "user_id": 1
}
)
    .setProtectedHeader({ alg: 'RS256', kid: 'my-key-id' })
    .setIssuer('your-auth-server')
    .setAudience('your-app')
    .setExpirationTime('2h')
    .setIssuedAt()
    .sign(privateKey);

  res.json({ token: jwt });
});

app.listen(PORT, () => {
  console.log(`Auth server running at http://localhost:${PORT}`);
});
