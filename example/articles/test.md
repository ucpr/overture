+++
title = "Test Article"
description = "Test Description"
date = "2024/04/18"
tags = ["test", "article"]
+++

## blockquote

body

> こんにちは。これはぶろっくくおーとです。

日本語表示。

> Hello. This is a block quote.

英語表示。

> こんにちは。This is a block quote.

日本語と英語表示。

> Hello. This is a block quote.
>> こんにちは。これはぶろっくくおーとです。

二重引用

## blockquote with highlight (未対応)

> [!NOTE]
> `NOTE` の例

未対応

> [!TIP]
> `TIP` の例

> [!IMPORTANT]
> `IMPORTANT` の例

並ぶと padding 入ってないのでラインがくっつく

> [!WARNING]
> `WARNING` の例

> [!CAUTION]
> `CAUTION` の例

## pre & codeblock

```rust
const body = "---
title: Test Title
tags:
  - rust
  - test
---"

let tree = markdown::to_mdast(body, config).ok().unwrap();
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

## code

`Hello, World!` はこんにちは世界という意味です。

## ul & li

- item1
    - item2
- item3
    - item4
    - item5
        - item6

## ol & li

1. item1
    1. item2
1. item3
    1. item4
    1. item5
        1. item6

## em

*italic で文字を書いてみるよ。*

## strong

**bold で文字を書いてみるよ。**

## em & strong

***italic と bold を組み合わせて文字を書いてみるよ。***

## strike

~~strike で文字を書いてみるよ。~~

## hr

`---` で水平線を引く。

---

`***` で水平線を引く。

***

## link

[Google](https://www.google.com)

### 定義参照リンク

[Google][1]

[1]: https://www.google.com

## コメント

<!-- コメント -->

これは使えないぽい

## GFM: Table

|header1|header2|header3|
|---|---|---|
|align left|align right|align center|
|a|b|c|
