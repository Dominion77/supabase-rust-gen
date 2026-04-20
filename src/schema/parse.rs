use super::OpenApiSpec;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct ParsedSchema {
    pub tables: Vec<TableDefinition>,
}

#[derive(Debug, Clone)]
pub struct TableDefinition {
    pub name: String,
    pub columns: Vec<ColumnDefinition>,
}

#[derive(Debug, Clone)]
pub struct ColumnDefinition {
    pub name: String,
    pub pg_type: String,
    pub is_nullable: bool,
    pub description: Option<String>,
}

impl ParsedSchema {
    pub fn from_openapi(spec: OpenApiSpec) -> Result<Self> {
        let mut tables = Vec::new();

        for (def_name, definition) in spec.definitions {
            // Skip non-table definitions (e.g., enums, composite types)
            if definition.def_type != "object" {
                continue;
            }

            let properties = match definition.properties {
                Some(props) => props,
                None => continue,
            };

            let required = definition.required.unwrap_or_default();
            let mut columns = Vec::new();

            for (col_name, prop) in properties {
                let pg_type = Self::infer_pg_type(&prop);
                let is_nullable = !required.contains(&col_name);

                columns.push(ColumnDefinition {
                    name: col_name,
                    pg_type,
                    is_nullable,
                    description: prop.description.clone(),
                });
            }

            tables.push(TableDefinition {
                name: def_name,
                columns,
            });
        }

        Ok(ParsedSchema { tables })
    }

    fn infer_pg_type(prop: &super::Property) -> String {
        match (&prop.prop_type, &prop.format) {
            (Some(t), Some(f)) if t == "string" && f == "date-time" => "timestamptz".to_string(),
            (Some(t), Some(f)) if t == "string" && f == "uuid" => "uuid".to_string(),
            (Some(t), Some(f)) if t == "string" && f == "jsonb" => "jsonb".to_string(),
            (Some(t), _) if t == "integer" => "int8".to_string(),
            (Some(t), _) if t == "number" => "numeric".to_string(),
            (Some(t), _) if t == "boolean" => "bool".to_string(),
            (Some(t), _) if t == "array" => {
                if let Some(items) = &prop.items {
                    format!("{}[]", Self::infer_pg_type(items))
                } else {
                    "text[]".to_string()
                }
            }
            (Some(t), _) => t.clone(),
            _ => "text".to_string(),
        }
    }
}