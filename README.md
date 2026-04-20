# supabase-rust-gen

Generate type-safe Rust structs from your Supabase database schema. Like `supabase-js` type generation, but for the Rust ecosystem.

[![Crates.io](https://img.shields.io/crates/v/supabase-rust-gen.svg)](https://crates.io/crates/supabase-rust-gen)
[![Documentation](https://docs.rs/supabase-rust-gen/badge.svg)](https://docs.rs/supabase-rust-gen)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

## Why?

Manually writing Rust structs for your Supabase tables is tedious and error-prone. Column names change, types drift, and nullable fields get missed. `supabase-rust-gen` eliminates this by:

- **Connecting directly** to your Supabase project's PostgREST endpoint
- **Reading the OpenAPI spec** to understand your exact schema
- **Generating idiomatic Rust** with proper Serde derives
- **Handling edge cases** like JSONB, arrays, nullable fields, and PostgreSQL types

## Features

-  **Zero-config for basic usage** — just point at your project
- **RLS-aware** — optionally generates security policy helpers
-  **Query builder support** — generates `postgrest-rs` functions with `--with-queries`
-  **Feature-flagged dependencies** — use `String` by default, opt into `chrono` and `uuid`
-  **Type-accurate mapping** — `int8` → `i64`, `jsonb` → `serde_json::Value`, etc.
-  **Self-documenting output** — includes column descriptions as doc comments
-  **Async-first** — built on `tokio` and `reqwest`

## Installation

```bash
cargo install supabase-rust-gen
```

## Quick Start

```bash
# Set environment variables (or pass via CLI)
export SUPABASE_URL="https://your-project.supabase.co"
export SUPABASE_ANON_KEY="eyJhbGciOiJIUzI1NiIs..."

# Generate types
supabase-rust-gen --output src/supabase_types.rs

# With all features enabled
supabase-rust-gen --with-rls --with-queries --output src/supabase_types.rs
```

## Usage

### CLI Options

```
supabase-rust-gen [OPTIONS] --url <URL> --anon-key <KEY>

Options:
  -u, --url <URL>          Supabase project URL [env: SUPABASE_URL]
  -k, --anon-key <KEY>     Supabase anonymous key [env: SUPABASE_ANON_KEY]
  -o, --output <PATH>      Output file path [default: src/supabase_types.rs]
      --with-rls           Generate RLS helper functions
      --with-queries       Generate postgrest-rs query functions
  -h, --help               Print help
  -V, --version            Print version
```

### Generated Code Example

Given a Supabase table:

```sql
CREATE TABLE posts (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  title TEXT NOT NULL,
  content TEXT,
  published BOOLEAN DEFAULT false,
  metadata JSONB,
  created_at TIMESTAMPTZ DEFAULT now()
);
```

Running `supabase-rust-gen` produces:

```rust
//! Auto-generated Supabase types - DO NOT EDIT MANUALLY

use serde::{Deserialize, Serialize};

/// Row from the `posts` table
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: String,
    pub title: String,
    pub content: Option<String>,
    pub published: Option<bool>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: Option<String>,
}
```

With `--with-queries` flag, you also get:

```rust
impl Post {
    /// Select rows from the `posts` table
    pub async fn select_posts(client: &Postgrest) -> Result<Vec<Self>> {
        let resp = client.from("posts").select("*").execute().await?;
        Ok(resp.json().await?)
    }

    /// Insert a row into the `posts` table
    pub async fn insert_posts(client: &Postgrest, row: &Self) -> Result<Self> {
        let resp = client
            .from("posts")
            .insert(serde_json::to_string(row)?)
            .execute()
            .await?;
        Ok(resp.json().await?)
    }
}
```

## Feature Flags

The generated code supports optional features that you can enable in your project's `Cargo.toml`:

```toml
[dependencies]
supabase-rust-gen = "0.1"  # CLI tool

# In your project that uses the generated code:
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Optional: better temporal types
chrono = { version = "0.4", features = ["serde"] }

# Optional: proper UUID handling
uuid = { version = "1.6", features = ["serde"] }

# Optional: for query functions
postgrest = "0.10"
anyhow = "1.0"
```

When these features are present, the generator upgrades types:

| PostgreSQL | Default | With Feature |
|------------|---------|--------------|
| `timestamptz` | `String` | `chrono::DateTime<Utc>` |
| `uuid` | `String` | `uuid::Uuid` |

## Type Mapping

| PostgreSQL Type | Rust Type | Notes |
|----------------|-----------|-------|
| `text`, `varchar` | `String` | |
| `int2` | `i16` | |
| `int4` | `i32` | |
| `int8` | `i64` | |
| `float4` | `f32` | |
| `float8` | `f64` | |
| `numeric` | `f64` | Precision may be lost |
| `bool` | `bool` | |
| `json`, `jsonb` | `serde_json::Value` | |
| `bytea` | `Vec<u8>` | |
| `date`, `time` | `String` | Use `chrono` for better types |
| `timestamptz` | `String` or `DateTime<Utc>` | Feature-gated |
| `uuid` | `String` or `Uuid` | Feature-gated |
| `ARRAY` | `Vec<T>` | Recursive mapping |
| `NULL` column | `Option<T>` | |

## Real-World Example

```rust
use postgrest::Postgrest;
use anyhow::Result;

// Include the generated types
include!("supabase_types.rs");

#[tokio::main]
async fn main() -> Result<()> {
    let client = Postgrest::new("https://your-project.supabase.co/rest/v1")
        .insert_header("apikey", "your-anon-key");

    // Type-safe querying
    let posts = Post::select_posts(&client).await?;
    
    for post in posts {
        println!("{} - {}", post.id, post.title);
    }

    // Type-safe insertion
    let new_post = Post {
        id: uuid::Uuid::new_v4().to_string(),
        title: "Hello Rust".to_string(),
        content: Some("Generated types are great!".to_string()),
        published: Some(true),
        metadata: None,
        created_at: None,
    };
    
    let inserted = Post::insert_posts(&client, &new_post).await?;
    println!("Inserted post ID: {}", inserted.id);

    Ok(())
}
```

## How It Works

1. **Schema Fetching**: Calls `GET /rest/v1/` with your API key to retrieve the OpenAPI specification
2. **Type Parsing**: Extracts table definitions, column types, and nullability constraints
3. **Type Mapping**: Converts PostgreSQL types to appropriate Rust types based on enabled features
4. **Code Generation**: Uses `quote` to build valid Rust AST, then formats with `prettyplease`
5. **Output**: Writes a single, self-contained Rust file ready for inclusion

## Comparison with Other Tools

| Tool | Language | Output | RLS Support | Query Building |
|------|----------|--------|-------------|----------------|
| `supabase-js` | TypeScript | Types only | ❌ | ❌ |
| `supabase-py` | Python | Types + Queries | ❌ | ✅ |
| **supabase-rust-gen** | **Rust** | **Types + Queries** | **✅** | **✅** |

## Limitations & Tradeoffs

- **Numeric precision**: `numeric` maps to `f64`, which may lose precision for very large numbers
- **Custom types**: Enums and composite types currently map to `String` (support planned)
- **RLS introspection**: RLS policies require SQL access; current helpers are stubs
- **Array dimensions**: Only 1D arrays are supported; multi-dimensional map to `Vec<serde_json::Value>`

## Contributing

Contributions are welcome! Areas that need attention:

- [ ] Enum type support from `pg_type` introspection
- [ ] Composite/row type generation
- [ ] Better RLS policy extraction (requires SQL connection)
- [ ] Custom type mapping overrides via config file
- [ ] Support for `--schema` flag to specify non-public schemas

```bash
git clone https://github.com/yourusername/supabase-rust-gen
cd supabase-rust-gen
cargo test
```

## License

MIT OR Apache-2.0 (your choice)

## Related Projects

- [postgrest-rs](https://github.com/supabase-community/postgrest-rs) - Rust client for PostgREST
- [supabase-js](https://github.com/supabase/supabase-js) - Official JavaScript client
- [supabase_flutter](https://github.com/supabase/supabase-flutter) - Flutter/Dart client