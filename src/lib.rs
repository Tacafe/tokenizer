pub mod tokenizer {
  extern crate mecab;
  use mecab::Tagger;

  pub struct Tokenizer {
    pub tagger: Tagger
  }

  impl Default for Tokenizer {
    fn default() -> Self {
      Self { tagger: Tagger::new("") }
    }
  }

  impl Tokenizer {
    pub fn tokenize(self: &mut Tokenizer, s: &str) -> Vec<String> {
      let mut tokens = Vec::new();
      for node in self.tagger.parse_to_node(s).iter_next() {
        match node.stat as i32 {
            mecab::MECAB_BOS_NODE => {},
            mecab::MECAB_EOS_NODE => {},
            _ => {
                let surface: &str = &(node.surface)[..(node.length as usize)];
                tokens.push(surface.to_string());
            }
        }
      }
      tokens
    }
  }

}

#[cfg(test)]
mod tests {
  #[test]
  fn tokenize_returns_vectr () {
    let mut t = crate::tokenizer::Tokenizer::default();

    let tokenized = t.tokenize("すもももももももものうち");
    let expected = ["すもも", "も", "もも", "も", "もも", "の", "うち"];
    assert_eq!(tokenized, expected);
  }
}