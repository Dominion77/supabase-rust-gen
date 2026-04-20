use crate::schema::ColumnDefinition;
use convert_case::{Case, Casing};

pub struct TypeMapper {
    use_chrono: bool,
    use_uuid: bool,
}

impl TypeMapper {
    pub fn new(use_chrono: bool, use_uuid: bool) -> Self {
        Self {
            use_chrono,
            use_uuid,
        }
    }

    pub fn map_column(&self, col: &ColumnDefinition) -> String {
        let base_type = self.map_pg_type(&col.pg_type);
        
        if col.is_nullable {
            format!("Option<{}>", base_type)
        } else {
            base_type
        }
    }

    fn map_pg_type(&self, pg_type: &str) -> String {
        match pg_type {
            "text" | "varchar" | "char" | "bpchar" => "String".to_string(),
            "int2" | "smallint" => "i16".to_string(),
            "int4" | "integer" => "i32".to_string(),
            "int8" | "bigint" => "i64".to_string(),
            "float4" | "real" => "f32".to_string(),
            "float8" | "double precision" => "f64".to_string(),
            "numeric" | "decimal" => "f64".to_string(),
            "bool" | "boolean" => "bool".to_string(),
            "json" | "jsonb" => "serde_json::Value".to_string(),
            "bytea" => "Vec<u8>".to_string(),
            "date" => "String".to_string(),
            "time" => "String".to_string(),
            "timetz" => "String".to_string(),
            "timestamptz" | "timestamp" => {
                if self.use_chrono {
                    "chrono::DateTime<chrono::Utc>".to_string()
                } else {
                    "String".to_string()
                }
            }
            "uuid" => {
                if self.use_uuid {
                    "uuid::Uuid".to_string()
                } else {
                    "String".to_string()
                }
            }
            s if s.ends_with("[]") => {
                let inner = &s[..s.len() - 2];
                format!("Vec<{}>", self.map_pg_type(inner))
            }
            _ => "String".to_string(),
        }
    }

    pub fn table_to_struct_name(table_name: &str) -> String {
        table_name.to_case(Case::Pascal)
    }
}