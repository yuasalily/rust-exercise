### 何のために作ったの？
rustのデザインパターン勉強用

### これは何？
[参考](https://rust-unofficial.github.io/patterns/intro.html)（[日本語訳](https://qiita.com/Yappii_111/items/4ccc3a8461cdd4035651)）を自分で試すやつ。

### メモ
- mutabilityについて([参考](https://doc.rust-jp.rs/the-rust-programming-language-ja/1.6/book/mutability.html))

```
let array = &mut [4,5,6];
array[0] = 1; // arrayの中身はmutableなのでこれは可能([1,5,6]になる)。
// array = &mut [4,5,6]  arrayはimmutableなのでこれは出来ない;
```
