use serde_json::Value;

pub trait GetValue {
    fn get_as_string(&self, key: &str) -> Option<String>;
    fn get_as_bool(&self, key: &str) -> Option<bool>;
    fn get_as_double(&self, key: &str) -> Option<f64>;
    fn get_as_i64(&self, key: &str) -> Option<i64>;
    fn get_as_vec(&self, key: &str) -> Option<Vec<Value>>;
}

impl GetValue for Value {
    fn get_as_string(&self, key: &str) -> Option<String> {
        self.get(key)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    fn get_as_bool(&self, key: &str) -> Option<bool> {
        self.get(key).and_then(|v| v.as_bool())
    }

    fn get_as_double(&self, key: &str) -> Option<f64> {
        self.get(key).and_then(|v| v.as_f64())
    }

    fn get_as_i64(&self, key: &str) -> Option<i64> {
        self.get(key).and_then(|v| v.as_i64())
    }

    fn get_as_vec(&self, key: &str) -> Option<Vec<Value>> {
        self.get(key).and_then(|v| v.as_array()).cloned()
    }
}
