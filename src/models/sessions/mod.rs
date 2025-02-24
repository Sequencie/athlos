use validator::{Validate, ValidationError, ValidationErrors};
pub mod schema;
use crate::models::sessions::schema::Session;

impl Validate for Session {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors: ValidationErrors = ValidationErrors::new();
        if self.players.is_empty() {
            errors.add("Players", ValidationError::new("No players were added to this session"));
        }
        if self.location.to_string().is_empty() {
            errors.add("Location", ValidationError::new("No location was provided for this session"));
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}