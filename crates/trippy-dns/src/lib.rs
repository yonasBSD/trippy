//! This crate provides a cheaply cloneable, non-blocking, caching, forward
//! and reverse DNS resolver which support the ability to lookup Autonomous
//! System (AS) information.
//!
//! # Example
//!
//! The following example perform a reverse DNS lookup and loop until it is
//! resolved or fails.  The lookup uses the Cloudflare 1.1.1.1 public DNS
//! service.
//!
//! Note that only a single reverse DNS lookup is performed (lazily) regardless
//! of how often the lookup is performed, unless the previous lookup failed
//! with `DnsEntry::Timeout(_)`.
//!
//! ```no_run
//! # fn main() -> anyhow::Result<()> {
//! # use std::net::IpAddr;
//! # use std::str::FromStr;
//! # use std::thread::sleep;
//! # use std::time::Duration;
//! use trippy_dns::{
//!     Config, DnsEntry, DnsResolver, IpAddrFamily, ResolveMethod, Resolved, Resolver, Unresolved,
//! };
//!
//! let config = Config::new(
//!     ResolveMethod::Cloudflare,
//!     IpAddrFamily::Ipv4Only,
//!     Duration::from_secs(5),
//! );
//! let resolver = DnsResolver::start(config)?;
//! let addr = IpAddr::from_str("1.1.1.1")?;
//! loop {
//!     let entry = resolver.lazy_reverse_lookup_with_asinfo(addr);
//!     match entry {
//!         DnsEntry::Pending(ip) => {
//!             println!("lookup of {ip} is pending, sleeping for 1 sec");
//!             sleep(Duration::from_secs(1));
//!         }
//!         DnsEntry::Resolved(Resolved::Normal(ip, addrs)) => {
//!             println!("lookup of {ip} resolved to {addrs:?}");
//!             return Ok(());
//!         }
//!         DnsEntry::Resolved(Resolved::WithAsInfo(ip, addrs, as_info)) => {
//!             println!("lookup of {ip} resolved to {addrs:?} with AS information {as_info:?}");
//!             return Ok(());
//!         }
//!         DnsEntry::NotFound(Unresolved::Normal(ip)) => {
//!             println!("lookup of {ip} did not match any records");
//!             return Ok(());
//!         }
//!         DnsEntry::NotFound(Unresolved::WithAsInfo(ip, as_info)) => {
//!             println!(
//!                 "lookup of {ip} did not match any records with AS information {as_info:?}"
//!             );
//!             return Ok(());
//!         }
//!         DnsEntry::Timeout(ip) => {
//!             println!("lookup of {ip} timed out");
//!             return Ok(());
//!         }
//!         DnsEntry::Failed(ip) => {
//!             println!("lookup of {ip} failed");
//!             return Ok(());
//!         }
//!     }
//! }
//! # Ok(())
//! # }
//! ```
#![warn(clippy::all, clippy::pedantic, clippy::nursery, rust_2018_idioms)]
#![allow(
    clippy::module_name_repetitions,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::option_if_let_else
)]
#![forbid(unsafe_code)]

mod lazy_resolver;
mod resolver;

pub use lazy_resolver::{Config, DnsResolver, IpAddrFamily, ResolveMethod};
pub use resolver::{AsInfo, DnsEntry, Error, Resolved, Resolver, Result, Unresolved};
