use slug::slugify;

/// Converts a string (e.g. article title) into a slug-safe format
pub fn to_slug(input: &str) -> String {
    slugify(input)
}
