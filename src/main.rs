use clap::Parser;
use supabase_rust_gen::{Cli, Generator, GeneratorConfig, SchemaFetcher};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    let cli = Cli::parse();
    
    // Fetch schema
    let fetcher = SchemaFetcher::new(cli.url.clone(), cli.anon_key.clone())?;
    let schema = fetcher.fetch_schema().await?;
    
    // Generate code
    let config = GeneratorConfig {
        with_rls: cli.with_rls,
        with_queries: cli.with_queries,
        output_path: cli.output,
    };
    
    let generator = Generator::new(schema, config);
    generator.generate()?;
    
    println!("Generated types successfully!");
    Ok(())
}