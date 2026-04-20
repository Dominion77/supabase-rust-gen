mod structs;
mod rls;
mod queries;

use crate::schema::ParsedSchema;
use crate::types::TypeMapper;
use anyhow::Result;
use std::path::PathBuf;

pub struct GeneratorConfig {
    pub with_rls: bool,
    pub with_queries: bool,
    pub output_path: PathBuf,
}

pub struct Generator {
    schema: ParsedSchema,
    config: GeneratorConfig,
    type_mapper: TypeMapper,
}

impl Generator {
    pub fn new(schema: ParsedSchema, config: GeneratorConfig) -> Self {
        Self {
            schema,
            config,
            type_mapper: TypeMapper::new(
                cfg!(feature = "chrono"),
                cfg!(feature = "uuid"),
            ),
        }
    }

    pub fn generate(&self) -> Result<()> {
        let structs_code = structs::generate(&self.schema, &self.type_mapper)?;
        let rls_code = if self.config.with_rls {
            rls::generate(&self.schema)?
        } else {
            String::new()
        };
        let queries_code = if self.config.with_queries {
            queries::generate(&self.schema)?
        } else {
            String::new()
        };

        let output = format!(
            "{}\n{}\n{}",
            structs_code, rls_code, queries_code
        );

        crate::output::write_formatted(&self.config.output_path, &output)?;
        Ok(())
    }
}