# Security reporting

Use the repository's **Security** tab for private vulnerability reporting when
available. If private reporting is unavailable, open an issue requesting a
private reporting channel without including exploit details or sensitive data.
Do not post account tokens, private keys, real configurations or packet captures.

Include affected versions/platforms, impact, reproduction steps using synthetic
data, and any proposed fix. Do not test against other people's services/accounts.
This project has not completed all live service and device acceptance; do not
treat development builds as a production security guarantee.

The certificate/key files under `tests/fixtures/` are intentionally public test
material. Production does not trust their CA. They must never be deployed.
Secret clearing, replay/authentication checks, vendor provenance and bounded
packet ownership are protected by regression checks. Linked code inspection is
a regression tripwire, not a formal security audit.
