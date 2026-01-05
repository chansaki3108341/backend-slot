#[derive(Clone, Debug)]
pub struct Language {
    pub name: String,
    pub color: String,
}

impl Language {
    pub fn new(name: impl Into<String>, color: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            color: color.into(),
        }
    }
}

/// Built-in curated list (current behavior).
const BUILTIN_LANGUAGE_SPECS: &[(&str, &str)] = &[
    ("Rust", "red"),
    ("Go", "cyan"),
    ("Python", "blue"),
    ("Node.js", "green"),
    ("Java", "yellow"),
    ("C#", "magenta"),
    ("PHP", "purple"),
    ("Ruby", "red"),
    ("Elixir", "magenta"),
    ("Kotlin", "orange"),
];

pub fn builtin_languages() -> Vec<Language> {
    BUILTIN_LANGUAGE_SPECS
        .iter()
        .map(|(name, color)| Language::new(*name, *color))
        .collect()
}

pub fn builtin_color_for(name: &str) -> &'static str {
    for (n, c) in BUILTIN_LANGUAGE_SPECS {
        if n.eq_ignore_ascii_case(name) {
            return c;
        }
    }
    "white"
}

pub fn is_supported_color(color: &str) -> bool {
    matches!(
        color,
        "red" | "blue" | "green" | "yellow" | "magenta" | "cyan" | "purple" | "orange" | "white"
    )
}
