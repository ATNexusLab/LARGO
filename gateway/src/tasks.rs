use mongodb::{
    bson::{Bson, Document, doc},
    results::{CollectionSpecification, CollectionType},
};
use serde::Serialize;
use serde_json::Value;

pub const TASKS_COLLECTION_NAME: &str = "tasks";
pub const TASK_STATUS_PENDING: &str = "pending";

const TASK_STATUS_VALUES: [&str; 4] = ["pending", "in_progress", "done", "blocked"];
const TITLE_MIN_LENGTH: i32 = 1;
const TITLE_MAX_LENGTH: i32 = 120;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateTaskInput {
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ValidationDetail {
    pub field: &'static str,
    pub issue: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskValidationError {
    pub message: &'static str,
    pub details: Vec<ValidationDetail>,
}

pub fn tasks_collection_validator() -> Document {
    doc! {
        "$jsonSchema": {
            "bsonType": "object",
            "required": ["title", "status", "created_at"],
            "properties": {
                "title": {
                    "bsonType": "string",
                    "minLength": TITLE_MIN_LENGTH,
                    "maxLength": TITLE_MAX_LENGTH,
                },
                "status": {
                    "enum": TASK_STATUS_VALUES.to_vec(),
                },
                "created_at": {
                    "bsonType": "date",
                },
            },
        },
    }
}

pub fn tasks_collection_is_prepared(specification: &CollectionSpecification) -> bool {
    specification.name == TASKS_COLLECTION_NAME
        && specification.collection_type == CollectionType::Collection
        && specification
            .options
            .validator
            .as_ref()
            .is_some_and(validator_satisfies_foundation)
}

pub fn validate_create_task_payload(value: Value) -> Result<CreateTaskInput, TaskValidationError> {
    let object = value.as_object().ok_or_else(|| TaskValidationError {
        message: "title is required and must contain between 1 and 120 characters",
        details: vec![ValidationDetail {
            field: "title",
            issue: "required",
        }],
    })?;

    let mut details = Vec::new();

    for key in object.keys() {
        if key != "title" {
            details.push(ValidationDetail {
                field: "body",
                issue: "extra_fields_not_allowed",
            });
            break;
        }
    }

    let Some(title_value) = object.get("title") else {
        details.push(ValidationDetail {
            field: "title",
            issue: "required",
        });

        return Err(TaskValidationError {
            message: "title is required and must contain between 1 and 120 characters",
            details,
        });
    };

    let Some(title) = title_value.as_str() else {
        details.push(ValidationDetail {
            field: "title",
            issue: "must_be_string",
        });

        return Err(TaskValidationError {
            message: "title is required and must contain between 1 and 120 characters",
            details,
        });
    };

    let trimmed_title = title.trim();
    let trimmed_length = trimmed_title.chars().count();

    if !(TITLE_MIN_LENGTH as usize..=TITLE_MAX_LENGTH as usize).contains(&trimmed_length) {
        details.push(ValidationDetail {
            field: "title",
            issue: "length_out_of_range",
        });
    }

    if !details.is_empty() {
        return Err(TaskValidationError {
            message: "title is required and must contain between 1 and 120 characters",
            details,
        });
    }

    Ok(CreateTaskInput {
        title: trimmed_title.to_owned(),
    })
}

fn validator_satisfies_foundation(validator: &Document) -> bool {
    let Some(Bson::Document(schema)) = validator.get("$jsonSchema") else {
        return false;
    };

    has_string_field(schema, "bsonType", "object")
        && required_fields_match(schema)
        && properties_match(schema)
}

fn required_fields_match(schema: &Document) -> bool {
    let Some(Bson::Array(required)) = schema.get("required") else {
        return false;
    };

    required.len() == 3
        && contains_string(required, "title")
        && contains_string(required, "status")
        && contains_string(required, "created_at")
}

fn properties_match(schema: &Document) -> bool {
    let Some(Bson::Document(properties)) = schema.get("properties") else {
        return false;
    };

    let Some(Bson::Document(title)) = properties.get("title") else {
        return false;
    };
    let Some(Bson::Document(status)) = properties.get("status") else {
        return false;
    };
    let Some(Bson::Document(created_at)) = properties.get("created_at") else {
        return false;
    };

    has_string_field(title, "bsonType", "string")
        && has_i32_field(title, "minLength", TITLE_MIN_LENGTH)
        && has_i32_field(title, "maxLength", TITLE_MAX_LENGTH)
        && status_values_match(status)
        && has_string_field(created_at, "bsonType", "date")
}

fn status_values_match(status: &Document) -> bool {
    let Some(Bson::Array(values)) = status.get("enum") else {
        return false;
    };

    values.len() == TASK_STATUS_VALUES.len()
        && TASK_STATUS_VALUES
            .iter()
            .all(|expected| contains_string(values, expected))
}

fn has_string_field(document: &Document, key: &str, expected: &str) -> bool {
    matches!(document.get(key), Some(Bson::String(value)) if value == expected)
}

fn has_i32_field(document: &Document, key: &str, expected: i32) -> bool {
    matches!(document.get(key), Some(Bson::Int32(value)) if *value == expected)
}

fn contains_string(values: &[Bson], expected: &str) -> bool {
    values
        .iter()
        .any(|value| matches!(value, Bson::String(found) if found == expected))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn deve_validar_payload_valido() {
        let payload = json!({ "title": "Test Task" });
        let result = validate_create_task_payload(payload);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().title, "Test Task");
    }

    #[test]
    fn deve_rejeitar_titulo_curto() {
        let payload = json!({ "title": "" });
        let result = validate_create_task_payload(payload);
        assert!(result.is_err());
    }

    #[test]
    fn deve_rejeitar_campos_extras() {
        let payload = json!({ "title": "Valid", "extra": "field" });
        let result = validate_create_task_payload(payload);
        assert!(result.is_err());
    }
}
