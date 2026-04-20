use crate::schema::{ParsedSchema, TableDefinition};
use crate::types::TypeMapper;
use anyhow::Result;
use quote::quote;

pub fn generate(schema: &ParsedSchema) -> Result<String> {
    let mut tokens = quote! {
        use postgrest::Postgrest;
        use anyhow::Result;
        use serde_json;
    };

    for table in &schema.tables {
        let query_impl = generate_query_functions(table);
        tokens.extend(query_impl);
    }

    Ok(prettyplease::unparse(&syn::parse2(tokens)?))
}

fn generate_query_functions(table: &TableDefinition) -> proc_macro2::TokenStream {
    let struct_name = syn::Ident::new(
        &TypeMapper::table_to_struct_name(&table.name),
        proc_macro2::Span::call_site(),
    );
    
    let table_name = &table.name;
    let select_fn = syn::Ident::new(
        &format!("select_{}", table.name.to_lowercase()),
        proc_macro2::Span::call_site(),
    );
    let insert_fn = syn::Ident::new(
        &format!("insert_{}", table.name.to_lowercase()),
        proc_macro2::Span::call_site(),
    );

    quote! {
        impl #struct_name {
            /// Select rows from the `#table_name` table
            pub async fn #select_fn(
                client: &Postgrest,
            ) -> Result<Vec<Self>, postgrest::Error> {
                let resp = client
                    .from(#table_name)
                    .select("*")
                    .execute()
                    .await?;
                    
                Ok(resp.json().await?)
            }

            /// Insert a row into the `#table_name` table
            pub async fn #insert_fn(
                client: &Postgrest,
                row: &Self,
            ) -> Result<Self, postgrest::Error> {
                let body = serde_json::to_string(row)?;
                let resp = client
                    .from(#table_name)
                    .insert(body)
                    .execute()
                    .await?;
                    
                Ok(resp.json().await?)
            }
        }
    }
}