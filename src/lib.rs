//!Simple HTTP client with built-in HTTPS support.
//!Currently it's in heavy development and may frequently change.
//!
//!## Example
//!Basic GET request
//!```
//!use http_req::request;
//!
//!fn main() {
//!    let mut writer = Vec::new(); //container for body of a response
//!    let res = request::get("https://doc.rust-lang.org/", &mut writer).unwrap();
//!
//!    println!("Status: {} {}", res.status_code(), res.reason());
//!}
//!```
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

// re-export module to properly feature gate sgx and regular std environment
#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub mod sgx_reexport_prelude {
    pub use rustls_sgx as rustls;
    pub use unicase_sgx as unicase;
    pub use webpki_roots_sgx as webpki_roots;
    pub use webpki_sgx as webpki;
}

pub mod error;
pub mod request;
pub mod response;
pub mod tls;
pub mod uri;

mod chunked;
