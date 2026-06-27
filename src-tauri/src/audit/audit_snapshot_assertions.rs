use serde_json::Value;

const FORBIDDEN_EXACT_KEYS: &[&str] = &[
    // passwords / auth secrets
    "password",
    "password_hash",
    "passwordHash",
    "currentPassword",
    "newPassword",
    "temporaryPassword",
    "confirmPassword",
    "secret",
    "token",
    "credential",
    "credentials",
    "apiKey",
    "api_key",
    "licenseKey",
    "license_key",
    // local paths / filesystem
    "path",
    "filePath",
    "absolutePath",
    "originalPath",
    "storedPath",
    "storagePath",
    "storageRoot",
    "backupPath",
    "archivePath",
    "outputPath",
    "exportPath",
    "templatePath",
    "thumbnailPath",
    "sourcePath",
    "targetPath",
    "tempDir",
    "workingDir",
    // large / raw content
    "content",
    "body",
    "fullText",
    "html",
    "markdown",
    "editorState",
    "sections",
    "sectionContent",
    "fileContent",
    "archiveContent",
    "zipBytes",
    "docxBytes",
    "sqliteDump",
    "databaseDump",
    "base64",
];

// Fragment checks catch key names that aren't an exact match but still look
// like secrets.  Plain "password" / "secret" / "token" are NOT listed here
// because they are already caught by FORBIDDEN_EXACT_KEYS *after* key
// normalisation, and listing them as fragments would cause false-positives
// on legitimate fields like `mustChangePassword` or `secretaryName`.
const FORBIDDEN_KEY_FRAGMENTS: &[&str] = &["apikey", "licensekey"];

const FORBIDDEN_PATH_SUFFIXES: &[&str] = &["path", "dir", "directory"];

pub fn assert_audit_value_is_safe(label: &str, value: &Value) {
    walk_value(label, "$", value);
}

pub fn assert_optional_audit_value_is_safe(label: &str, value: &Option<Value>) {
    if let Some(value) = value {
        assert_audit_value_is_safe(label, value);
    }
}

fn walk_value(label: &str, json_path: &str, value: &Value) {
    match value {
        Value::Object(map) => {
            for (key, nested) in map {
                assert_key_is_allowed(label, json_path, key);

                let next_path = format!("{json_path}.{key}");
                walk_value(label, &next_path, nested);
            }
        }
        Value::Array(items) => {
            for (index, nested) in items.iter().enumerate() {
                let next_path = format!("{json_path}[{index}]");
                walk_value(label, &next_path, nested);
            }
        }
        Value::String(text) => {
            assert_string_value_is_allowed(label, json_path, text);
        }
        _ => {}
    }
}

fn assert_key_is_allowed(label: &str, json_path: &str, key: &str) {
    let normalized = normalize_key(key);

    for forbidden in FORBIDDEN_EXACT_KEYS {
        if normalized == normalize_key(forbidden) {
            panic!(
                "unsafe audit snapshot key in {label}: {json_path}.{key} matches forbidden key {forbidden}"
            );
        }
    }

    for fragment in FORBIDDEN_KEY_FRAGMENTS {
        if normalized.contains(fragment) {
            panic!(
                "unsafe audit snapshot key in {label}: {json_path}.{key} contains forbidden fragment {fragment}"
            );
        }
    }

    for suffix in FORBIDDEN_PATH_SUFFIXES {
        if normalized.ends_with(suffix) {
            panic!(
                "unsafe audit snapshot key in {label}: {json_path}.{key} looks like a filesystem path key"
            );
        }
    }
}

fn assert_string_value_is_allowed(label: &str, json_path: &str, text: &str) {
    if looks_like_windows_absolute_path(text) || looks_like_unix_absolute_path(text) {
        panic!(
            "unsafe audit snapshot string in {label}: {json_path} looks like an absolute local path"
        );
    }

    if looks_like_large_base64_blob(text) {
        panic!(
            "unsafe audit snapshot string in {label}: {json_path} looks like large base64/blob content"
        );
    }
}

fn normalize_key(key: &str) -> String {
    key.chars()
        .filter(|ch| *ch != '_' && *ch != '-' && *ch != ' ')
        .flat_map(|ch| ch.to_lowercase())
        .collect()
}

fn looks_like_windows_absolute_path(text: &str) -> bool {
    let bytes = text.as_bytes();

    bytes.len() >= 3
        && bytes[1] == b':'
        && (bytes[2] == b'\\' || bytes[2] == b'/')
        && bytes[0].is_ascii_alphabetic()
}

fn looks_like_unix_absolute_path(text: &str) -> bool {
    text.starts_with("/Users/")
        || text.starts_with("/home/")
        || text.starts_with("/mnt/")
        || text.starts_with("/var/")
        || text.starts_with("/tmp/")
        || text.starts_with("/private/")
}

fn looks_like_large_base64_blob(text: &str) -> bool {
    if text.len() < 256 {
        return false;
    }

    let base64_chars = text
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric() || *ch == '+' || *ch == '/' || *ch == '=')
        .count();

    base64_chars * 100 / text.len() > 95
}
