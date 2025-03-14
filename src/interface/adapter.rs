use serde::{Deserialize, Serialize};
use serde_json;

use crate::usecase::{decode_usecase::{DecodeInputData, DecodeInputPort}, encode_usecase::{EncodeInputData, EncodeInputPort}};

#[derive(Deserialize)]
pub struct AlBhedTransferRequest {
  text: String
}

#[derive(Serialize)]
pub struct AlBhedTransferResponse {
  result: String
}

pub struct JsonAlBhedTransferAdapter {
  encode_input_port: Box<dyn EncodeInputPort + Sync + Send>,
  decode_input_port: Box<dyn DecodeInputPort + Sync + Send>
}

impl JsonAlBhedTransferAdapter {

  pub fn new(encode_port: Box<dyn EncodeInputPort + Sync + Send>, decode_port: Box<dyn DecodeInputPort + Sync + Send>) -> Self {
    JsonAlBhedTransferAdapter {encode_input_port: encode_port, decode_input_port:   decode_port}
  }

  pub fn encode(&self, json: &str) -> Result<String, String> {
    let request: AlBhedTransferRequest = serde_json::from_str(json)
      .map_err(|_| "Invalid JSON format".to_string())?;
    let encode_input_data = EncodeInputData::new(&request.text);
    match self.encode_input_port.encode(encode_input_data) {
      Ok(encoded) => {
        let response = AlBhedTransferResponse {result: encoded.get_text().to_string()};
        serde_json::to_string(&response)
          .map_err(|_| "Failed to serialize response".to_string())
      },
      Err(error) => {
        Err(error)
      }
    }
  }

  pub fn decode(&self, json: &str) -> Result<String, String> {
    let request: AlBhedTransferRequest = serde_json::from_str(json)
      .map_err(|_| "Invalid JSON format".to_string())?;
    let decode_input_data = DecodeInputData::new(&request.text);
    match self.decode_input_port.decode(decode_input_data) {
      Ok(decoded) => {
        let response = AlBhedTransferResponse {result: decoded.get_text().to_string()};
        serde_json::to_string(&response)
          .map_err(|_| "Failed to serialize response".to_string())
       },
       Err(error) => {
        Err(error)
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::usecase::{decode_usecase::DecodeInteractor, encode_usecase::EncodeInteractor};

  #[test]
  fn test_encode_valid_json() {
    let encode_port = EncodeInteractor::new();
    let decode_port = DecodeInteractor::new();
    let adapter = JsonAlBhedTransferAdapter::new(Box::new(encode_port), Box::new(decode_port));
    let json = r#"{"text": "がんばろう！"}"#;
    let result = adapter.encode(json).unwrap();
    assert_eq!(result, r#"{"result":"ダンザノフ！"}"#);
  }

  #[test]
  fn test_encode_invalid_json() {
    let encode_port = EncodeInteractor::new();
    let decode_port = DecodeInteractor::new();
    let adapter = JsonAlBhedTransferAdapter::new(Box::new(encode_port), Box::new(decode_port));
    let json = r#"invalid json"#;
    let result = adapter.encode(json);
    assert!(result.is_err());
  }

  #[test]
  fn test_decode_valid_json() {
    let encode_port = EncodeInteractor::new();
    let decode_port = DecodeInteractor::new();
    let adapter = JsonAlBhedTransferAdapter::new(Box::new(encode_port), Box::new(decode_port));
    let json = r#"{"text": "ヤヌサー"}"#;
    let result = adapter.decode(json).unwrap();
    assert_eq!(result, r#"{"result":"ますたー"}"#);
  }

  #[test]
  fn test_decode_invalid_json() {
    let encode_port = EncodeInteractor::new();
    let decode_port = DecodeInteractor::new();
    let adapter = JsonAlBhedTransferAdapter::new(Box::new(encode_port), Box::new(decode_port));
    let json = r#"invalid json"#;
    let result = adapter.decode(json);
    assert!(result.is_err());
  }
}