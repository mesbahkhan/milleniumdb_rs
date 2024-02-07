use crate::query::exceptions::LogicException;

#[derive(Debug, Copy, Clone)]
pub enum ResponseType {
    JSON,
    XML,
    TSV,
    CSV,
    TURTLE,
}

impl ResponseType {
    pub fn response_type_to_string(response_type: ResponseType) -> &'static str {
        match response_type {
            ResponseType::JSON => "JSON",
            ResponseType::XML => "XML",
            ResponseType::TSV => "TSV",
            ResponseType::CSV => "CSV",
            ResponseType::TURTLE => "TURTLE",
        }
    }
}

impl std::fmt::Display for ResponseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ResponseType::response_type_to_string(*self))
    }
}

impl From<&str> for ResponseType {
    fn from(s: &str) -> Self {
        match s {
            "JSON" => ResponseType::JSON,
            "XML" => ResponseType::XML,
            "TSV" => ResponseType::TSV,
            "CSV" => ResponseType::CSV,
            "TURTLE" => ResponseType::TURTLE,
            _ => panic!("Invalid response type string"),
        }
    }
}

impl ResponseType {
    pub fn from_str(s: &str) -> Result<ResponseType, LogicException> {
        match s {
            "JSON" => Ok(ResponseType::JSON),
            "XML" => Ok(ResponseType::XML),
            "TSV" => Ok(ResponseType::TSV),
            "CSV" => Ok(ResponseType::CSV),
            "TURTLE" => Ok(ResponseType::TURTLE),
            _ => Err(LogicException::new("Unmanaged ResposeType in response_type_to_string")),
        }
    }
}
