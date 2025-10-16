use std::path::Path;

use anyhow::Context;

/// If the given string ends up with the given suffix, returns the portion of
/// the string before the suffix.
pub fn match_trailing<'a>(input: &'a str, suffix: &str) -> Option<&'a str> {
    if input.ends_with(suffix) {
        let end = input.len().saturating_sub(suffix.len());
        Some(&input[..end])
    } else {
        None
    }
}

pub trait PathExt {
    fn file_name_ends_with(&self, suffix: &str) -> bool;
    fn file_name_trim_end<'a>(&'a self, suffix: &str) -> anyhow::Result<&'a str>;
    fn parent_err(&self) -> anyhow::Result<&Path>;
}

impl<P> PathExt for P
where
    P: AsRef<Path>,
{
    fn file_name_ends_with(&self, suffix: &str) -> bool {
        self.as_ref()
            .file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.ends_with(suffix))
            .unwrap_or(false)
    }

    fn file_name_trim_end<'a>(&'a self, suffix: &str) -> anyhow::Result<&'a str> {
        let path = self.as_ref();
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .with_context(|| format!("Path did not have a file name: {}", path.display()))?;

        match_trailing(file_name, suffix)
            .with_context(|| format!("Path did not end in {}: {}", suffix, path.display()))
    }

    fn parent_err(&self) -> anyhow::Result<&Path> {
        let path = self.as_ref();
        path.parent()
            .with_context(|| format!("Path does not have a parent: {}", path.display()))
    }
}

/// Sanitizes a file/directory name to be a valid Roblox instance name.
/// Currently replaces '@' with '|' to allow files with @ symbols to sync properly.
pub fn sanitize_instance_name(name: &str) -> String {
    name.replace('@', "|")
}

// TEMP function until rojo 8.0, when it can be replaced with bool::default (aka false)
pub fn emit_legacy_scripts_default() -> Option<bool> {
    Some(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_at_symbol() {
        assert_eq!(sanitize_instance_name("test@script"), "test|script");
        assert_eq!(sanitize_instance_name("@start"), "|start");
        assert_eq!(sanitize_instance_name("end@"), "end|");
        assert_eq!(sanitize_instance_name("multiple@at@symbols"), "multiple|at|symbols");
    }

    #[test]
    fn test_sanitize_no_change() {
        assert_eq!(sanitize_instance_name("normal_script"), "normal_script");
        assert_eq!(sanitize_instance_name("test|pipe"), "test|pipe");
        assert_eq!(sanitize_instance_name(""), "");
    }
}
