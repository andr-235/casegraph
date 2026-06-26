pub const OBJECT_TYPE_PERSON: &str = "person";
pub const OBJECT_TYPE_ACCOUNT: &str = "account";
pub const OBJECT_TYPE_PHONE: &str = "phone";
pub const OBJECT_TYPE_ADDRESS: &str = "address";
pub const OBJECT_TYPE_VEHICLE: &str = "vehicle";
pub const OBJECT_TYPE_ORGANIZATION: &str = "organization";
pub const OBJECT_TYPE_DOCUMENT: &str = "document";
pub const OBJECT_TYPE_IMAGE: &str = "image";
pub const OBJECT_TYPE_PUBLICATION: &str = "publication";
pub const OBJECT_TYPE_EVENT: &str = "event";
pub const OBJECT_TYPE_SOURCE: &str = "source";
pub const OBJECT_TYPE_OTHER: &str = "other";

pub const OBJECT_TYPES: [&str; 12] = [
    OBJECT_TYPE_PERSON,
    OBJECT_TYPE_ACCOUNT,
    OBJECT_TYPE_PHONE,
    OBJECT_TYPE_ADDRESS,
    OBJECT_TYPE_VEHICLE,
    OBJECT_TYPE_ORGANIZATION,
    OBJECT_TYPE_DOCUMENT,
    OBJECT_TYPE_IMAGE,
    OBJECT_TYPE_PUBLICATION,
    OBJECT_TYPE_EVENT,
    OBJECT_TYPE_SOURCE,
    OBJECT_TYPE_OTHER,
];

pub fn is_valid_object_type(value: &str) -> bool {
    OBJECT_TYPES.contains(&value)
}

pub fn object_code_prefix(object_type: &str) -> Option<&'static str> {
    match object_type {
        OBJECT_TYPE_PERSON => Some("P"),
        OBJECT_TYPE_ACCOUNT => Some("A"),
        OBJECT_TYPE_PHONE => Some("TEL"),
        OBJECT_TYPE_ADDRESS => Some("ADDR"),
        OBJECT_TYPE_VEHICLE => Some("CAR"),
        OBJECT_TYPE_ORGANIZATION => Some("ORG"),
        OBJECT_TYPE_DOCUMENT => Some("DOC"),
        OBJECT_TYPE_IMAGE => Some("IMG"),
        OBJECT_TYPE_PUBLICATION => Some("PUB"),
        OBJECT_TYPE_EVENT => Some("EVT"),
        OBJECT_TYPE_SOURCE => Some("SRC"),
        OBJECT_TYPE_OTHER => Some("OBJ"),
        _ => None,
    }
}
