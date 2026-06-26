pub const MATERIAL_TYPE_IMAGE: &str = "image";
pub const MATERIAL_TYPE_PDF: &str = "pdf";
pub const MATERIAL_TYPE_DOCUMENT: &str = "document";
pub const MATERIAL_TYPE_SPREADSHEET: &str = "spreadsheet";
pub const MATERIAL_TYPE_TEXT: &str = "text";
pub const MATERIAL_TYPE_HTML: &str = "html";
pub const MATERIAL_TYPE_OTHER: &str = "other";

pub const MATERIAL_TYPES: [&str; 7] = [
    MATERIAL_TYPE_IMAGE,
    MATERIAL_TYPE_PDF,
    MATERIAL_TYPE_DOCUMENT,
    MATERIAL_TYPE_SPREADSHEET,
    MATERIAL_TYPE_TEXT,
    MATERIAL_TYPE_HTML,
    MATERIAL_TYPE_OTHER,
];

pub fn is_valid_material_type(value: &str) -> bool {
    MATERIAL_TYPES.contains(&value)
}
