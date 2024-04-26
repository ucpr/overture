+++
title = "markdown-rs で front matter の値を参照する"
description = "markdown-rs で front matter の値を参照する"
date = "2024-04-18T00:00:00+09:00"
tags = ["rust", "markdown"]
+++

## はじめに
markdown ファイルにメタデータを記述できる YAML front matter という仕様があります。この記事では、markdown-rs で front matter の値を取得する方法について説明します。

## 前提

この記事内での使用されているツール、クレートのバージョンはは下記の通りです。

- rustc
  - 1.77.0 (aedd173a2 2024-03-17)
- markdown-rs
  - 1.0.0-alpha.16

## markdown-rs での YAML front matter の取得

markdown-rs で front matter の値を取得するには AST を取得し、AST から front matter の値を取得します。
AST の取得には `markdown::to_mdast` 関数を使用します。

```rust
const body = "---
title: Test Title
tags:
  - rust
  - test
---"

let tree = markdown::to_mdast(body, config).ok().unwrap();
```

そして、AST から front matter の値を取得するために、AST のノードを探索して `Node::Yaml` の型に一致するものを取得します。
取得できる値は `String` 型になるので、必要であれば YAML パーサーを使って値をパースしてあげることで front matter に記述された値を取得することができます。

```rust
let mut front_matter = String::new();
tree.children().into_iter().for_each(|node| {
    for child in node.iter() {
        match child {
            markdown::mdast::Node::Yaml(yaml) => {
                front_matter = yaml.value.clone();
            }
            _ => {}
        }
    }
});
```

## TOML 形式で記述された front matter の取得

markdown-rs では YAML 以外にも TOML  形式の front matter の記述もサポートしており、YAML のときと同じ形で値を取得することができます。
TOML 形式での front matter の記述では、デリミタが `+++` となります。

```rust
const body = "+++
title = \"Test Title\"
tags = [\"rust\", \"test\"]
+++"

let tree = markdown::to_mdast(body, config).ok().unwrap();
```

データの取得は YAML とほぼ同様で、`match` で `Node::Toml` にマッチさせて値を取得します。

```rust
let mut front_matter = String::new();
tree.children().into_iter().for_each(|node| {
    for child in node.iter() {
        match child {
            markdown::mdast::Node::Toml(toml) => {
                front_matter = toml.value.clone();
            }
            _ => {}
        }
    }
});
```

## おわりに

この記事では、markdown-rs で front matter の値を取得する方法について説明しました。
front matter の取得方法についてドキュメントや記事などが殆どなかったのでブログにまとめましたが、 markdown-rs ではテストが充実しており他にも使い方で迷ったりした場合はこちらを参照すると良いかもしれません。

本記事において、異なっている説明や表現がありましたらご連絡ください。

## 参考

- [wooorm/markdown-rs | github.com](https://github.com/wooorm/markdown-rs)
- [YAML front matter の使用 | docs.github.com](https://docs.github.com/ja/contributing/writing-for-github-docs/using-yaml-frontmatter)
- [Front Matter | jekyllrb.com](https://jekyllrb.com/docs/front-matter/)
