pub mod tokenizer {
  extern crate mecab;
  use mecab::Tagger;

  pub struct Tokenizer {
    pub tagger: Tagger
  }

  #[derive(Debug, PartialEq)]
  pub struct Token {
    pub surface: String,
    pub feature: String
  }

  impl Default for Tokenizer {
    fn default() -> Self {
      Self { tagger: Tagger::new("") }
    }
  }

  impl Tokenizer {
    pub fn tokenize(self: &mut Tokenizer, s: &str) -> Vec<Token> {
      let mut tokens = Vec::new();
      for node in self.tagger.parse_to_node(s).iter_next() {
        match node.stat as i32 {
            mecab::MECAB_BOS_NODE => {},
            mecab::MECAB_EOS_NODE => {},
            _ => {
              let surface: String = (&(node.surface)[..(node.length as usize)]).to_string();
              let feature: String = node.feature;
              let t = Token {
                surface: surface,
                feature: feature
              };
              tokens.push(t);
            }
          }
        }
      tokens
    }
  }
}

#[cfg(test)]
mod tests {
  use super::tokenizer::*;

  #[test]
  fn tokenize_returns_tokens() {
    let mut expected_tokens: Vec<Token> = Vec::new();
    expected_tokens.push(Token { surface: String::from("隣"), feature: String::from("名詞,一般,*,*,*,*,隣,トナリ,トナリ") });
    expected_tokens.push(Token { surface: String::from("の"), feature: String::from("助詞,連体化,*,*,*,*,の,ノ,ノ") });
    expected_tokens.push(Token { surface: String::from("客"), feature: String::from("名詞,一般,*,*,*,*,客,キャク,キャク") });
    expected_tokens.push(Token { surface: String::from("は"), feature: String::from("助詞,係助詞,*,*,*,*,は,ハ,ワ") });
    expected_tokens.push(Token { surface: String::from("よく"), feature: String::from("副詞,一般,*,*,*,*,よく,ヨク,ヨク") });
    expected_tokens.push(Token { surface: String::from("牡蠣"), feature: String::from("名詞,一般,*,*,*,*,牡蠣,カキ,カキ") });
    expected_tokens.push(Token { surface: String::from("を"), feature: String::from("助詞,格助詞,一般,*,*,*,を,ヲ,ヲ") });
    expected_tokens.push(Token { surface: String::from("食う"), feature: String::from("動詞,自立,*,*,五段・ワ行促音便,基本形,食う,クウ,クウ") });
    expected_tokens.push(Token { surface: String::from("客"), feature: String::from("名詞,一般,*,*,*,*,客,キャク,キャク") });
    expected_tokens.push(Token { surface: String::from("だ"), feature: String::from("助動詞,*,*,*,特殊・ダ,基本形,だ,ダ,ダ") });

    let mut t = Tokenizer::default();
    let tokenized = t.tokenize("隣の客はよく牡蠣を食う客だ");

    assert_eq!(tokenized, expected_tokens);
  }

}