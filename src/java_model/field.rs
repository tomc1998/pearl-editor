use Modifier;

/// A field, containing modifiers, a name, and a type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub modifiers: Vec<Modifier>,
    pub field_type: String,
    pub name: String,
}
