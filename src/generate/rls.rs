use crate::schema::ParsedSchema;
use anyhow::Result;

pub fn generate(schema: &ParsedSchema) -> Result<String> {
    // RLS policy generation is complex and depends on actual policies
    // This is a placeholder that generates sensible defaults- DYOR
    let mut output = String::new();
    
    output.push_str("\n// Row Level Security helpers\n");
    output.push_str("// Note: Actual RLS policies must be inspected via SQL\n");
    output.push_str("// These are generated stubs based on common patterns\n\n");
    
    for table in &schema.tables {
        output.push_str(&format!(
            r#"/// Check if user can SELECT from `{}`
pub async fn can_select_{}(client: &Postgrest, user_id: &str) -> Result<bool, postgrest::Error> {{
    // Implement based on your RLS policies
    Ok(true)
}}

/// Check if user can INSERT into `{}`
pub async fn can_insert_{}(client: &Postgrest, user_id: &str) -> Result<bool, postgrest::Error> {{
    // Implement based on your RLS policies
    Ok(true)
}}

"#,
            table.name,
            table.name.to_lowercase(),
            table.name,
            table.name.to_lowercase()
        ));
    }
    
    Ok(output)
}