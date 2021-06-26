# @mochiya98/rs-read-image

Node 学園 36 時限目 オンライン  
Rust でネイティブモジュールを書く サンプルレポジトリ

結構リッチな構成です

```
lib/ # ライブラリ本体
napi/ # Node API(N-API)
packages/ # 各プラットフォームバイナリ用パッケージ
  linux-x64/
  ...
wasm/ # wasm
wasm-browser-test # wasm × webpackのテスト
```

## 環境構築

```bash
npm i -g @napi-rs/cli
cargo install wasm-pack
```
