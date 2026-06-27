pub const EVENT_TYPE_FACT: &str = "fact";
pub const EVENT_TYPE_ACTION: &str = "action";
pub const EVENT_TYPE_OBSERVATION: &str = "observation";
pub const EVENT_TYPE_DOCUMENT_FIXATION: &str = "document_fixation";
pub const EVENT_TYPE_RELATION_ESTABLISHED: &str = "relation_established";
pub const EVENT_TYPE_MATERIAL_RECEIVED: &str = "material_received";
pub const EVENT_TYPE_OTHER: &str = "other";

pub const EVENT_TYPES: [&str; 7] = [
    EVENT_TYPE_FACT,
    EVENT_TYPE_ACTION,
    EVENT_TYPE_OBSERVATION,
    EVENT_TYPE_DOCUMENT_FIXATION,
    EVENT_TYPE_RELATION_ESTABLISHED,
    EVENT_TYPE_MATERIAL_RECEIVED,
    EVENT_TYPE_OTHER,
];

pub fn is_valid_event_type(value: &str) -> bool {
    EVENT_TYPES.contains(&value)
}
