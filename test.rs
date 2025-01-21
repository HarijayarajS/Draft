#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_mobile_with_country() {
        let test_cases = vec![
            ("+14155552671", "US"), // USA
            ("+919876543210", "IN"), // India
            ("+442071838750", "GB"), // UK
            ("+8613912345678", "CN"), // China
            ("+81345678901", "JP"), // Japan
            ("+61412345678", "AU"), // Australia
            ("+4915112345678", "DE"), // Germany
            ("+33612345678", "FR"), // France
            ("+5511999998888", "BR"), // Brazil
            ("+27123456789", "ZA"), // South Africa
        ];

        for (number, country_code) in test_cases {
            let result = validate_mobile_with_country(number, country_code);
            assert!(result.is_ok(), "Failed for number: {}, country: {}", number, country_code);
        }
    }

    #[test]
    fn test_invalid_mobile_with_country() {
        let invalid_cases = vec![
            ("12345", "US"),            // Invalid USA number
            ("+91987", "IN"),           // Short Indian number
            ("+441", "GB"),             // Too short UK number
            ("+86139abc", "CN"),        // Invalid characters in China number
            ("+9991234567890", "ZZ"),   // Non-existent country code
        ];

        for (number, country_code) in invalid_cases {
            let result = validate_mobile_with_country(number, country_code);
            assert!(result.is_err(), "Should have failed for number: {}, country: {}", number, country_code);
        }
    }
}