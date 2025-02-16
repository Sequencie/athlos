use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Theme {
    pub _id: Option<ObjectId>,
    pub name: String,
    pub image: String
}

impl Validate for Theme {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors: ValidationErrors = ValidationErrors::new();
        if self.name.is_empty() {
            errors.add("Name", ValidationError::new("A name for the theme was not provided"))
        }
        if self.name.is_empty() {
            errors.add("Image", ValidationError::new("An image for the theme was not provided"))
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}