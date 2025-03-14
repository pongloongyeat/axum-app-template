#[derive(Clone, Debug)]
pub struct ValidationError {
    pub property: String,
    pub errors: Vec<String>,
}

pub trait Validatable {
    fn validated_properties() -> Vec<String>;

    fn validate_property(&self, property: &str) -> Option<Vec<String>>;

    fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let errors = Self::validated_properties()
            .iter()
            .filter_map(|property| {
                self.validate_property(property)
                    .map(|errors| (property, errors))
            })
            .map(|(property, errors)| ValidationError {
                property: property.to_owned(),
                errors,
            })
            .collect::<Vec<ValidationError>>();

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
