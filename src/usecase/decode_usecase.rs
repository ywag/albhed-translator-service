use crate::domain::entities::AlBhedText;

pub trait DecodeInputPort {
  fn decode(&self, input_data: DecodeInputData) -> Result<DecodeOutputData, String>;
}

pub struct DecodeInputData {
  text: String
}

pub struct DecodeOutputData {
  text: String
}

impl DecodeInputData {
  pub fn new(input: &str) -> Self {
    DecodeInputData { text: input.to_string() }
  }

  pub fn get_text(&self) -> &str {
    &self.text
  }
}

impl DecodeOutputData {
  pub fn new(output: &str) -> Self {
    DecodeOutputData { text: output.to_string() }
  }

  pub fn get_text(&self) -> &str {
    &self.text
  }

}

pub struct DecodeInteractor;

impl DecodeInteractor {
  pub fn new() -> DecodeInteractor {
    DecodeInteractor{}
  }
}

impl DecodeInputPort for DecodeInteractor {
    fn decode(&self, input_data: DecodeInputData) -> Result<DecodeOutputData, String> {
      match AlBhedText::new(input_data.get_text()) {
        Ok(albhed_text) => {
          Ok(DecodeOutputData::new(albhed_text.decode().text()))
        }
        Err(error) => {
          Err(error)
        }
      }
    }
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_decode() {
    let decode_port = DecodeInteractor::new();
    let decode_input_data = DecodeInputData::new("マギレヤキセ！");
    let result = decode_port.decode(decode_input_data);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().get_text(), "はじめまして！");
  }
}