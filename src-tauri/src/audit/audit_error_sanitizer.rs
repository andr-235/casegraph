use serde_json::{Map, Value};

pub const REDACTED_PATH: &str = "[redacted:path]";
pub const REDACTED_SECRET: &str = "[redacted:secret]";
pub const REDACTED_CONTENT: &str = "[redacted:content]";

/// Recursively sanitize a JSON value before writing it to `technical_details`.
///
/// - Keys that look like filesystem paths → replaced with `[redacted:path]`
/// - Keys that look like secrets/credentials → replaced with `[redacted:secret]`
/// - Keys that look like large content blobs → replaced with `[redacted:content]`
/// - String values → path-like tokens are redacted via `sanitize_error_text`
pub fn sanitize_audit_details(value: Value) -> Value {
    sanitize_value(value, None)
}

/// Redact absolute filesystem path tokens from a free-form error text string.
///
/// Splits on whitespace, trims common punctuation from each token, checks
/// whether the cleaned token looks like an absolute path, and replaces it.
pub fn sanitize_error_text(input: &str) -> String {
    let mut result = String::new();

    for token in input.split_whitespace() {
        let cleaned = token.trim_matches(|ch: char| {
            ch == '"'
                || ch == '\''
                || ch == '`'
                || ch == ','
                || ch == ';'
                || ch == ':'
                || ch == '('
                || ch == ')'
                || ch == '['
                || ch == ']'
        });

        let replacement = if looks_like_absolute_path(cleaned) {
            token.replace(cleaned, REDACTED_PATH)
        } else {
            token.to_string()
        };

        if !result.is_empty() {
            result.push(' ');
        }

        result.push_str(&replacement);
    }

    result
}

fn sanitize_value(value: Value, key: Option<&str>) -> Value {
    if let Some(key) = key {
        if is_secret_key(key) {
            return Value::String(REDACTED_SECRET.to_string());
        }

        if is_path_key(key) {
            return Value::String(REDACTED_PATH.to_string());
        }

        if is_content_key(key) {
            return Value::String(REDACTED_CONTENT.to_string());
        }
    }

    match value {
        Value::Object(map) => {
            let mut sanitized = Map::new();

            for (nested_key, nested_value) in map {
                sanitized.insert(
                    nested_key.clone(),
                    sanitize_value(nested_value, Some(&nested_key)),
                );
            }

            Value::Object(sanitized)
        }
        Value::Array(items) => Value::Array(
            items
                .into_iter()
                .map(|item| sanitize_value(item, key))
                .collect(),
        ),
        Value::String(text) => Value::String(sanitize_error_text(&text)),
        other => other,
    }
}

fn normalize_key(key: &str) -> String {
    key.chars()
        .filter(|ch| *ch != '_' && *ch != '-' && *ch != ' ')
        .flat_map(|ch| ch.to_lowercase())
        .collect()
}

fn is_secret_key(key: &str) -> bool {
    let key = normalize_key(key);

    key.contains("password")
        || key.contains("secret")
        || key.contains("token")
        || key.contains("credential")
        || key.contains("apikey")
        || key.contains("licensekey")
}

fn is_path_key(key: &str) -> bool {
    let key = normalize_key(key);

    key == "path"
        || key.ends_with("path")
        || key.ends_with("dir")
        || key.ends_with("directory")
        || key == "storageroot"
        || key == "tempdir"
        || key == "workingdir"
}

fn is_content_key(key: &str) -> bool {
    let key = normalize_key(key);

    key == "content"
        || key == "body"
        || key == "fulltext"
        || key == "html"
        || key == "markdown"
        || key == "editorstate"
        || key == "sections"
        || key == "filecontent"
        || key == "archivecontent"
        || key == "zipbytes"
        || key == "docxbytes"
        || key == "sqlitedump"
        || key == "databasedump"
        || key == "base64"
}

fn looks_like_absolute_path(value: &str) -> bool {
    looks_like_windows_absolute_path(value)
        || looks_like_unc_path(value)
        || looks_like_unix_absolute_path(value)
}

fn looks_like_windows_absolute_path(value: &str) -> bool {
    let bytes = value.as_bytes();

    bytes.len() >= 3
        && bytes[1] == b':'
        && (bytes[2] == b'\\' || bytes[2] == b'/')
        && bytes[0].is_ascii_alphabetic()
}

fn looks_like_unc_path(value: &str) -> bool {
    value.starts_with("\\\\")
}

fn looks_like_unix_absolute_path(value: &str) -> bool {
    value.starts_with("/Users/")
        || value.starts_with("/home/")
        || value.starts_with("/mnt/")
        || value.starts_with("/var/")
        || value.starts_with("/tmp/")
        || value.starts_with("/private/")
}
