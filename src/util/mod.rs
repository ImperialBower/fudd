pub mod random_ordering;
pub mod uci;

/// How to convert a String into a static str.
#[must_use]
pub fn str_from_string(value: String) -> &'static str {
    Box::leak(value.into_boxed_str())
}
