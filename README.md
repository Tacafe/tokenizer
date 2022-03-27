# tokenizer
Sample mecab wrapper for rust tutorial

# Rust tutorial memo
## TDD
- rustのお作法
  - unit test: モジュールやソースコードと同じ場所に書く(src配下)
  - integuration test: tests/ ディレクトリ配下に置く
  - internalなテスト(非公開の関数)もテスト書くべき
  - tests/直下に置かれたrsファイルはテスト対象として暗黙的にコンパイルされる
  - tests/fooサブモジュールに置かれたrsファイルは明示的にクレートとして使用しない以上コンパイルされない
  - src/main.rsはextern crateで関数をインポートできないので、テストしない
- test runner: cargo test
  - testは並列実行するのでfixture(file)などのデータにテスト間の依存がないようにする
  ```.bash
  # テストを全実行
  cargo test

  # テスト関数名を指定して実行
  cargo test test_name

  # 特定のintegurationテストファイルを指定して実行
  cargo test --test integuration_test_file_name
  ```