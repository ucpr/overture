+++
title = "markdown-rs で front-matter に設定された値を参照する"
description = "markdown-rs で front-matter に設定された値を参照する"
date = "2024/04/18"
tags = ["rust", "markdown"]
+++

## はじめに
body

## 環境

- rustc
  - 1.77.0 (aedd173a2 2024-03-17)
- markdown-rs
  - 1.0.0-alpha.16

## front-matter とは

## markdown-rs での front-matter の取得

hogehoge

```rust
const body = "+++
title = \"Test Title\"
tags = [\"rust\", \"test\"]
+++
"

let tree = markdown::to_mdast(body, config).ok().unwrap();
```

HugaHuga

```rust
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

## 参考

- [github.com/wooorm/markdown-rs](https://github.com/wooorm/markdown-rs)
