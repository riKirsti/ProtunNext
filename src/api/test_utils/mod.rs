//! Opt-in development utilities, separate from VPN control sockets. Login uses
//! the host network only when explicitly invoked; no test invokes real login.
pub mod muon_test_auth;
pub mod test_config_parser;
