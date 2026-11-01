These freshly generated localhost certificates and the private key are PUBLIC TEST FIXTURES. They protect no real service and must never be used in deployment. The CA is loaded only by test code, never by the default client. Generated with OpenSSL Ed25519 on 2026-10-31, ten-year validity, SAN localhost, serverAuth EKU. No Proton/user credentials or certificates are present.

`ed25519-private-PUBLIC-TEST-ONLY.der` is the RFC8410 PKCS#8 v1 encoding of the
public test seed `[7; 32]`. `ed25519-client-TEST-ONLY.der` is its self-signed
clientAuth certificate, generated with OpenSSL on 2026-11-01 with ten-year
validity. The account fixture returns it over an isolated in-memory HTTP/TCP
connection to exercise the real certificate issuer state machine, cache and
explicit refresh. It is not a Proton-issued certificate and does not establish
LocalAgent trust. Neither fixture root nor client certificate is added to the
production trust store.
