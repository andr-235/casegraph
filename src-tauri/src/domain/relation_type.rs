pub const RELATION_TYPE_RELATED_TO: &str = "related_to";
pub const RELATION_TYPE_USES: &str = "uses";
pub const RELATION_TYPE_BELONGS_TO: &str = "belongs_to";
pub const RELATION_TYPE_MENTIONED_IN: &str = "mentioned_in";
pub const RELATION_TYPE_APPEARS_WITH: &str = "appears_with";
pub const RELATION_TYPE_CONFIRMED_BY_MATERIAL: &str = "confirmed_by_material";
pub const RELATION_TYPE_LINKED_TO_PHONE: &str = "linked_to_phone";
pub const RELATION_TYPE_LINKED_TO_ACCOUNT: &str = "linked_to_account";
pub const RELATION_TYPE_LINKED_TO_DOCUMENT: &str = "linked_to_document";
pub const RELATION_TYPE_LINKED_TO_VEHICLE: &str = "linked_to_vehicle";
pub const RELATION_TYPE_LINKED_TO_ADDRESS: &str = "linked_to_address";
pub const RELATION_TYPE_LINKED_TO_ORGANIZATION: &str = "linked_to_organization";
pub const RELATION_TYPE_OTHER: &str = "other";

pub const RELATION_TYPES: [&str; 13] = [
    RELATION_TYPE_RELATED_TO,
    RELATION_TYPE_USES,
    RELATION_TYPE_BELONGS_TO,
    RELATION_TYPE_MENTIONED_IN,
    RELATION_TYPE_APPEARS_WITH,
    RELATION_TYPE_CONFIRMED_BY_MATERIAL,
    RELATION_TYPE_LINKED_TO_PHONE,
    RELATION_TYPE_LINKED_TO_ACCOUNT,
    RELATION_TYPE_LINKED_TO_DOCUMENT,
    RELATION_TYPE_LINKED_TO_VEHICLE,
    RELATION_TYPE_LINKED_TO_ADDRESS,
    RELATION_TYPE_LINKED_TO_ORGANIZATION,
    RELATION_TYPE_OTHER,
];

pub fn is_valid_relation_type(value: &str) -> bool {
    RELATION_TYPES.contains(&value)
}
