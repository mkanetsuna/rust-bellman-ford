# 最短路問題に対するbellman-fordアルゴリズムの適用をRustで実装してみた

#Rust環境の構築

**Rustツールチェーンをインストール**

```install.sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Rustプロジェクトを作成**

```create Rust Project
cargo new rust_bellman_ford
cd rust_bellman_ford
```

**プログラムの実装**

`rust_bellman_ford`ディレクトリ配下の`src/main.rs`にプログラムを記述

**プログラムをビルドして実行**

```execute
cargo run
```