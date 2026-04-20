use clap::Parser;
use std::path::PathBuf;

/// Generate Rust types from a Supabase PostgREST schema
///
/// Connects to your Supabase project and generates type-safe Rust structs
/// for all tables in the public schema. Optionally generates RLS policy
/// helpers and query builder functions.
///
/// Environment variables:
///   SUPABASE_URL        - Your project URL (e.g., https://project.supabase.co)
///   SUPABASE_ANON_KEY   - Your project's anon/public API key
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Supabase project URL (e.g., https://project.supabase.co)
    #[arg(short, long, env = "SUPABASE_URL")]
    pub url: String,

    /// Supabase anonymous/public API key
    #[arg(short, long, env = "SUPABASE_ANON_KEY")]
    pub anon_key: String,

    /// Output file path (default: src/supabase_types.rs)
    #[arg(short, long, default_value = "src/supabase_types.rs")]
    pub output: PathBuf,

    /// Generate Row Level Security helper functions
    #[arg(long)]
    pub with_rls: bool,

    /// Generate postgrest-rs query builder functions
    #[arg(long)]
    pub with_queries: bool,
}