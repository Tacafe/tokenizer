use tokenizer::tokenizer::Tokenizer;

fn main() {
    let mut t: Tokenizer = Tokenizer::default();
    let sentence = "隣の客はよく牡蠣を食う客だ";
    for token in t.tokenize(sentence).iter() {
        println!("{}", token);
    }
}
