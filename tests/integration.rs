use supabase_rust_gen::{Generator, GeneratorConfig, SchemaFetcher};

#[tokio::test]
#[ignore] // Requires live Supabase instance
async fn test_fetch_real_schema() {
    let url = std::env::var("TEST_SUPABASE_URL").unwrap();
    let key = std::env::var("TEST_SUPABASE_KEY").unwrap();
    
    let fetcher = SchemaFetcher::new(url, key).unwrap();
    let schema = fetcher.fetch_schema().await.unwrap();
    
    assert!(!schema.tables.is_empty());
}