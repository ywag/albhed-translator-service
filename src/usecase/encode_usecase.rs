use crate::domain::entities::OriginalText;

pub trait EncodeInputPort {
  fn encode(&self, input_data: EncodeInputData) -> Result<EncodeOutputData, String>;
}

pub struct EncodeInputData {
  text: String
}

pub struct EncodeOutputData {
  text: String
}

impl EncodeInputData {
  pub fn new(input: &str) -> Self {
    EncodeInputData { text: input.to_string() }
  }

  pub fn get_text(&self) -> &str {
    &self.text
  }
}

impl EncodeOutputData {
  pub fn new(output: &str) -> Self {
    EncodeOutputData { text: output.to_string() }
  }

  pub fn get_text(&self) -> &str {
    &self.text
  }

}

pub struct EncodeInteractor;

impl EncodeInteractor {
  pub fn new() -> EncodeInteractor {
    EncodeInteractor{}
  }
}

impl EncodeInputPort for EncodeInteractor {
    fn encode(&self, input_data: EncodeInputData) -> Result<EncodeOutputData, String> {
      match OriginalText::new(input_data.get_text()) {
        Ok(original_text) => {
          Ok(EncodeOutputData::new(original_text.encode().text()))
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
  fn test_encode() {
    let encode_port = EncodeInteractor::new();
    let encode_input_data = EncodeInputData::new("じかんがねえ！ さっさと はっしんしろ！");
    let result = encode_port.encode(encode_input_data);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().get_text(), "ギアンダメネ！ ラッラソ マッキンキノ！");
  }
}