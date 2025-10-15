use std::collections::HashSet;

type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug)]
struct ApiError(String);

impl ApiError {
    fn error(msg: impl Into<String>) -> Self {
        ApiError(msg.into())
    }
}

fn validate_schedule_fields(
    enabled_days: &Option<Vec<i16>>,
    enabled_time_from: &Option<i16>,
    enabled_time_to: &Option<i16>,
) -> ApiResult<()> {
    // Validate enabled days
    if let Some(days) = enabled_days {
        // Check duplicates
        let unique_days: HashSet<_> = days.iter().collect();
        if unique_days.len() != days.len() {
            return Err(ApiError::error("enabled_days contains duplicate values"));
        }

        // Check range 1..=7
        if days.iter().any(|&d| d < 1 || d > 7) {
            return Err(ApiError::error("enabled_days contains invalid values (must be 1..7)"));
        }
    }

    // Validate time: either both present or both absent
    match (enabled_time_from, enabled_time_to) {
        (Some(from), Some(to)) => {
            let time_max = 60 * 24; // minutes in a day
            if *from < 0 || *from > time_max {
                return Err(ApiError::error("enabled_time_from out of range"));
            }
            if *to < 0 || *to > time_max {
                return Err(ApiError::error("enabled_time_to out of range"));
            }
        }
        (None, None) => {} // ok
        _ => return Err(ApiError::error("enabled_time_from and enabled_time_to must both be set or both be None")),
    }

    Ok(())
}