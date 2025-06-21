pub trait RowJsonExtSafe {
    fn get_jsonb_or_default_safe<T: DeserializeOwned + Default>(&self, column: &str) -> T;
}

impl RowJsonExtSafe for Row {
    fn get_jsonb_or_default_safe<T: DeserializeOwned + Default>(&self, column: &str) -> T {
        let value: Option<Value> = self.try_get(column).unwrap_or(None);
        if let Some(json_value) = value {
            serde_json::from_value(json_value).unwrap_or_default()
        } else {
            T::default()
        }
    }
}