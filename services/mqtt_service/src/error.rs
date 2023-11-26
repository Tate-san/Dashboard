
#[derive(Debug)]
pub enum MqttError {
    SerdeJsonError(serde_json::Error),
    SqlxError(sqlx::Error)
}

impl MqttError {
    pub fn get_error(&self) -> String {
        match self {
            MqttError::SerdeJsonError(e) => e.to_string(),
            MqttError::SqlxError(e) => e.to_string(),
        }
    }
}

impl From<serde_json::Error> for MqttError {
    fn from(error: serde_json::Error) -> Self {
        MqttError::SerdeJsonError(error)
    }
}

impl From<sqlx::Error> for MqttError {
    fn from(error: sqlx::Error) -> Self {
        MqttError::SqlxError(error)
    }
}