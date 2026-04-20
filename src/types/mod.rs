mod mapping;

pub use mapping::TypeMapper;

pub struct RustType {
    pub type_name: String,
    pub needs_serde_bound: bool,
}