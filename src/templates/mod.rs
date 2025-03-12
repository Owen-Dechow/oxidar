mod parser;
mod var;

use std::collections::HashMap;

pub use oxidar_derive::ToTemplateVar;
pub use parser::TemplateParsingError;
pub use var::{TemplateVar, ToTemplateVar};

pub fn resolve_template_string(
    initial: String,
    data: HashMap<&'static str, TemplateVar>,
) -> Result<String, TemplateParsingError> {
    let temp_var = TemplateVar::Indexable(data);
    return parser::parse(initial, &temp_var);
}
