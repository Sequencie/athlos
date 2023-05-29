use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Challenge {
    pub _id: Option<ObjectId>,
    pub name: String,
    pub sensor_id: String,
    pub points: u32,
    pub image: String
    // Add location later.
}

impl Validate for Challenge {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors: ValidationErrors = ValidationErrors::new();
        if self.name.is_empty() {
            errors.add("Name", ValidationError::new("A name for the challenge was not provided"));
        }
        if self.sensor_id.is_empty() {
            errors.add("Sensor id", ValidationError::new("No sensor was provided"));
        }
        if self.points == 0 {
            errors.add("Points", ValidationError::new("No points were provided"));
        }
        if self.image.is_empty() {
            errors.add("Image", ValidationError::new("No image was provided"));
        }
        if errors.is_empty() { 
            return Ok(()); 
        }
        return Err(errors);
    
    }
}