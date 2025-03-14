use std::collections::HashMap;
use std::sync::LazyLock;

pub struct AlBhedText {
  text: String
}

impl AlBhedText {
  pub fn new(input: &str) -> Result<Self, String> {
    if input.is_empty() {
      return Err("Empty String".to_string());
    }

    Ok(AlBhedText {
      text: input.to_string()
    })
  }

  pub fn text(&self) -> &str {
    &self.text
  }

  pub fn decode(&self) -> OriginalText {
    let s: String = self.text
                .chars()
                .map(|c| *FROM_ALBHED_RULE.get(&c).unwrap_or(&c))
                .collect();

    OriginalText::new(&s).unwrap()
    
  }
}

pub struct OriginalText {
  text: String
}

impl OriginalText {
  pub fn new(input: &str) -> Result<Self, String> {
    if input.is_empty() {
      return Err("Empty String".to_string());
    }

    Ok(OriginalText {
      text: input.to_string()
    })
  }

  pub fn text(&self) -> &str {
    &self.text
  }

  pub fn encode(&self) -> AlBhedText {
    let s: String = self.text
                .chars()
                .map(|c| *TO_ALBHED_RULE.get(&c).unwrap_or(&c))
                .collect();

    AlBhedText::new(&s).unwrap()
  }
}


static TO_ALBHED_RULE: LazyLock<HashMap<char, char>> = LazyLock::new(|| {
  let mut mapping = HashMap::new();

  //英語変換ルール
  mapping.insert('E','A');
  mapping.insert('P','B');
  mapping.insert('S','C');
  mapping.insert('T','D');
  mapping.insert('I','E');
  mapping.insert('W','F');
  mapping.insert('K','G');
  mapping.insert('N','H');
  mapping.insert('U','I');
  mapping.insert('V','J');
  mapping.insert('G','K');
  mapping.insert('C','L');
  mapping.insert('L','M');
  mapping.insert('R','N');
  mapping.insert('Y','O');
  mapping.insert('B','P');
  mapping.insert('X','Q');
  mapping.insert('H','R');
  mapping.insert('M','S');
  mapping.insert('D','T');
  mapping.insert('O','U');
  mapping.insert('F','V');
  mapping.insert('Z','W');
  mapping.insert('Q','X');
  mapping.insert('A','Y');
  mapping.insert('J','Z');

  //日本語変換ルール
  mapping.insert('あ','ワ');
  mapping.insert('い','ミ');
  mapping.insert('う','フ');
  mapping.insert('え','ネ');
  mapping.insert('お','ト');
  mapping.insert('か','ア');
  mapping.insert('き','チ');
  mapping.insert('く','ル');
  mapping.insert('け','テ');
  mapping.insert('こ','ヨ');
  mapping.insert('さ','ラ');
  mapping.insert('し','キ');
  mapping.insert('す','ヌ');
  mapping.insert('せ','へ');
  mapping.insert('そ','ホ');
  mapping.insert('た','サ');
  mapping.insert('ち','ヒ');
  mapping.insert('つ','ユ');
  mapping.insert('て','セ');
  mapping.insert('と','ソ');
  mapping.insert('な','ハ');
  mapping.insert('に','シ');
  mapping.insert('ぬ','ス');
  mapping.insert('ね','メ');
  mapping.insert('の','オ');
  mapping.insert('は','マ');
  mapping.insert('ひ','リ');
  mapping.insert('ふ','ク');
  mapping.insert('へ','ケ');
  mapping.insert('ほ','ロ');
  mapping.insert('ま','ヤ');
  mapping.insert('み','イ');
  mapping.insert('む','ツ');
  mapping.insert('め','レ');
  mapping.insert('も','コ');
  mapping.insert('や','タ');
  mapping.insert('ゆ','ヲ');
  mapping.insert('よ','モ');
  mapping.insert('ら','ナ');
  mapping.insert('り','ニ');
  mapping.insert('る','ウ');
  mapping.insert('れ','エ');
  mapping.insert('ろ','ノ');
  mapping.insert('わ','カ');
  mapping.insert('を','ム');
  mapping.insert('ん','ン');
  mapping.insert('が','ダ');
  mapping.insert('ぎ','ジ');
  mapping.insert('ぐ','ヅ');
  mapping.insert('げ','デ');
  mapping.insert('ご','ゾ');
  mapping.insert('ざ','バ');
  mapping.insert('じ','ギ');
  mapping.insert('ず','ブ');
  mapping.insert('ぜ','ゲ');
  mapping.insert('ぞ','ボ');
  mapping.insert('だ','ガ');
  mapping.insert('ぢ','ビ');
  mapping.insert('づ','グ');
  mapping.insert('で','べ');
  mapping.insert('ど','ゴ');
  mapping.insert('ば','ザ');
  mapping.insert('び','ヂ');
  mapping.insert('ぶ','ズ');
  mapping.insert('べ','ゼ');
  mapping.insert('ぼ','ド');
  mapping.insert('ぱ','プ');
  mapping.insert('ぴ','ぺ');
  mapping.insert('ぷ','パ');
  mapping.insert('ぺ','ポ');
  mapping.insert('ぽ','ピ');
  mapping.insert('ぁ','ァ');
  mapping.insert('ぃ','ィ');
  mapping.insert('ぅ','ゥ');
  mapping.insert('ぇ','ェ');
  mapping.insert('ぉ','ォ');
  mapping.insert('ゃ','ャ');
  mapping.insert('ゅ','ュ');
  mapping.insert('ょ','ョ');
  mapping.insert('っ','ッ');

  mapping
});

static FROM_ALBHED_RULE: LazyLock<HashMap<char, char>> = LazyLock::new(|| {
  let mut reverse = HashMap::new();
  for (&k, &v) in TO_ALBHED_RULE.iter() {
    reverse.insert(v, k);
  }
  reverse
});


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new_with_empty_text() {
    let result = AlBhedText::new("");
    assert!(result.is_err());
  }

  #[test]
  fn test_decode_jp() {
    let result = AlBhedText::new("ギアンダメネ！ ラッラソ マッキンキノ！");
    assert!(result.is_ok());

    let decoded = result.unwrap().decode();
    assert_eq!(decoded.text(), "じかんがねえ！ さっさと はっしんしろ！");
  }

  #[test]
  fn test_encode_jp() {
    let result = OriginalText::new("やっちまうか！？");
    assert!(result.is_ok());

    let encoded = result.unwrap().encode();
    assert_eq!(encoded.text(), "タッヒヤフア！？");
  }
}