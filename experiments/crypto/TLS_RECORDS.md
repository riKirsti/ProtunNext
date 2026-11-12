# Isolated AWS-LC TLS record-cipher experiment

`stage5-aws-lc-records.patch` is an explicit builder-only experiment, not a
production dependency, runtime feature or selected backend. Its SHA-256 is
`3cf2ac9ad7eef47777bd72a4846ddb86d9d2f4660594309d2cdeb9a99e69c5e8`.

The patch replaces only the implementations behind the existing ordered TLS
cipher-suite identities with AWS-LC 1.19.0. Ring continues to supply key
exchange, signature verification, signing and randomness. TLS versions, suite
order, roots, server-name checks, ALPN policy and Stealth's coupled inner-
WireGuard identity policy remain unchanged. Packet AEAD remains ring. Inner
HTTPS load and fixed peer/server binaries remain baseline implementations.

Select baseline merged PR #9 (`4c7cc53a0932fa31579729b603d80a9b9e9ecabf`),
`tun_mode=vnet`, `client_cores=1`, `parallel_receive=false`, `tls_records=true`
in the existing native/LTS download workflow. The builder validates the digest,
applies the patch only inside the isolated candidate archive, and records it
before computing runtime/effective source identities. Default builds and the
baseline archive are untouched. Do not combine it with Stage 4's prototype.

**Compatibility is unresolved:** rustls auto-selects a default only when one
built-in provider is enabled. Adding its second provider makes implicit
`ClientConfig::builder()` ambiguous unless the embedding application installs
a default. Proton Local Agent 0.13.0 `ConfigBuilder::build` currently uses that
implicit API. The production API also accepts externally constructed TLS
connections. A record-speed result cannot establish compatibility or authorize
silently changing a process-global provider. Consequently this archive-only
prototype is limited to explicit-provider NoLocalAgent comparisons; it cannot
ship as implemented, even if a speed benefit is measured. Rejection/deferment
must distinguish this compatibility constraint from measured performance.

Source: [rustls provider selection](https://docs.rs/rustls/0.23.46/rustls/crypto/struct.CryptoProvider.html).
