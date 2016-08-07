varsun: Variable Substituion Functions(s) for Rust
==================================================

varsunとは - About of varsun -
------------------------------

Go言語における `os.ExpandEnv` 相当のものが欲しかったけど、標準ライブラリには見当たらなかった。
そんなわけで必要な機能を持ったものを作ろうとした結果がこの `varsun` である。

### できること - Features -

- `$foo` や `${bar}` を見つけて、それを対応する文字列へと置き換える
- `%foo%` も対応。
- `varsun` 直下の関数を使うと、`Windows` なら `%foo%` を、それ以外は `%foo` を認識するように自動的に切り替わる。
- ついでに環境変数展開を標準装備。
    - 使い方の参考でもある。

### やりたいこと - TODO -

- 効率のいい文字列探索
    - さすがに一文字ずつ見てくのは非効率といわざるを得ない
- マクロ使ってみたい
- ベンチマーク
    - `cargo bench` が動かん(´・ω・｀)

使い方 - Usage -
----------------

```rust
extern crate varsun;

// on Linux system.
let homedir = varsun::substitute("$HOME", |name: &str| -> Option<String> {
    match ::std::env::var(name) {
        Ok(val) => Some(val),
        Err(_)  => NOne,
    }
});

// on Windows.
let homedir = varsun::substitute("%USERPROFILE%", |name: &str| -> Option<String> {
    match ::std::env::var(name) {
        Ok(val) => Some(val),
        Err(_)  => None,
    }
});
```

### 常にPOSIX (`$HOGE`) を使う - Use POSIX (`$HOGE`) style always -

```rust
extern crate varsun;

// on Linux.
let homedir = varsun::posix::substitute("${HOME}", |name: &str| -> Option<String> {
    match ::std::env::var(name) {
        Ok(val) => Some(val),
        Err(_)  => None,
    }
});

// on Windows.
let homedir = varsun::posix::substitute("${USERPROFILE}", |name: &str| -> Option<String> {
    match ::std::env::var(name) {
        Ok(val) => Some(val),
        Err(_)  => None,
    }
});
```

### 常にWindows (`%HOGE%`) を使う - Use Windows (`%HOGE%`) style always -

```rust
extern crate varsun;

// on Linux.
let homedir = varsun::windows::substitute("%HOME%", |name: &str| -> Option<String> {
    match ::std::env::var(name) {
        Ok(val) => Some(val),
        Err(_)  => None,
    }
});

// on Windows.
let homedir = varsun::windows::substitute("%USERPROFILE%", |name: &str| -> Option<String> {
    match ::std::env::var(name) {
        Ok(val) => Some(val),
        Err(_)  => None,
    }
});
```

作者 - Author -
---------------

**TANAKA Kenichi aka chikuchikugonzalez (ちくちく('ω')ごんざれす)**

- [chiku2gonzalez on Twitter](https://twitter.com/chiku2gonzalez)
- [chikuchikugonzalez on Hatena Blog](http://chiku2gonzalez.hatenablog.com/)

ライセンス - LICENSE -
----------------------
[MIT License](http://chiku2gonzalez.bitbucket.org/license/MITv2016.txt)
