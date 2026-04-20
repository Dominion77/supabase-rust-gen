//! # supabase-rust-gen
//!
//! Generate Rust types and query builders from Supabase PostgREST schemas.
//!
//! ## Core Pipeline
//!
//! 1. Fetch OpenAPI schema from `/rest/v1/`
//! 2. Parse PostgreSQL type definitions
//! 3. Map SQL types to Rust types
//! 4. Generate struct definitions with Serde derives
//! 5. Optionally generate RLS helpers and query functions
//! 6. Write formatted output to file

pub mod cli;
pub mod generate;
pub mod output;
pub mod schema;
pub mod types;

pub use cli::Cli;
pub use generate::{Generator, GeneratorConfig};
pub use schema::SchemaFetcher;