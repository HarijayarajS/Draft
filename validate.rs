use validator::{Validate, ValidationError, ValidationErrorsKind};
use std::borrow::Cow;

pub fn validate_input(input: &dyn Validate) -> Result<(), ValidationError> {
    if let Err(err) = input.validate() {
        let mut msgs: Vec<String> = vec![];

        for (field, error_kind) in err.errors() {
            let msg = match error_kind {
                ValidationErrorsKind::Field(field_errors) => field_errors
                    .iter()
                    .map(|field_error| {
                        field_error
                            .message
                            .clone()
                            .unwrap_or_else(|| Cow::Borrowed("Invalid input"))
                            .into_owned()
                    })
                    .collect::<Vec<_>>()
                    .join(", "),
                _ => format!("Invalid {}", field),
            };

            msgs.push(msg);
        }

        msgs.sort();
        let err_str = msgs.join(", ");

        // Create a new ValidationError with custom message
        let mut custom_error = ValidationError::new("validation_error");
        custom_error.message = Some(Cow::Owned(err_str));

        return Err(custom_error);
    }

    Ok(())
}