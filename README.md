# 最短路問題に対するbellman-ford法をRustで実装してみた(Rust環境構築から)

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

## プログラム解説

### 初期化
グラフ $G = (V, E)$ のノード集合を $V$ 、エッジ集合を $E$ とする。始点を $s$ とし、各ノード $v$ の初期距離 $d(v)$ を以下のように設定する。

$$
d(v) =
\begin{cases}
0 & \text{if } v = s \\
\infty & \text{otherwise}
\end{cases}
$$

### エッジの緩和
グラフの全エッジ $(u, v) \in E$ に対して、以下の操作を |V| - 1 回繰り返す。

$$
\text{if } d(u) + w(u, v) < d(v) \text{ then } d(v) = d(u) + w(u, v)
$$

*ここで、$w(u, v)$ はエッジ $(u, v)$ の重み

### 負の重みサイクルのチェック
グラフの全エッジ $(u, v) \in E$ に対して、以下の条件をチェックする。

$$
\text{if } d(u) + w(u, v) < d(v) \text{ then グラフには負の重みサイクルが存在する}
$$

### 最短経路の構築
終点 $t$ から始点 $s$ への最短経路を辿るために、各ノード $v$ の前任者 $pred(v)$ を以下のように更新する。

$$
\text{if } d(u) + w(u, v) < d(v) \text{ then } pred(v) = u
$$

### 全体のアルゴリズムの流れ
1. 初期化ステップを実行。
2. グラフの全エッジに対して、|V| - 1 回のエッジ緩和を実行。
3. 負の重みサイクルが存在するかをチェック。
4. 最短経路を構築。

### 参考コードの数式化

1. **初期化**:
   $$
   d(v) = 
   \begin{cases}
   0 & \text{if } v = s \\
   \infty & \text{otherwise}
   \end{cases}
   $$

2. **エッジの緩和**:
   $$
   \text{for each } (u, v) \in E \text{ do}
   $$
   $$
   \quad \text{if } d(u) + w(u, v) < d(v) \text{ then } d(v) = d(u) + w(u, v)
   $$
   
3. **負の重みサイクルのチェック**:
   $$
   \text{for each } (u, v) \in E \text{ do}
   $$
   $$
   \quad \text{if } d(u) + w(u, v) < d(v) \text{ then グラフには負の重みサイクルが存在する}
   $$

4. **最短経路の構築**:
   $$
   \text{while } current\_node \neq s \text{ do}
   $$
   $$
   \quad path.push(current\_node)
   $$
   $$
   \quad current\_node = pred(current\_node)
   $$
   $$
   path.push(s)
   $$
   $$
   path.reverse()
   $$