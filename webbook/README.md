### なんでこれを作ったの？
Rustとwebアプリの勉強のため

### これは何？
RustでWebサーバを作る。参考にした本の最後に到達したので一旦完成。

[参考にした本](https://zenn.dev/bigen1925/books/introduction-to-web-application-with-python/viewer/preface)

### メモ
- docker内で動かす場合はbindするアドレスに注意。`docker inspect (コンテナ名)`の`Networks`内の`IPAddress`を使う。[参考](https://www.codit.work/notes/7sgrfyoex15pt6njnjs9/)。時間で変わる？
- root(main.rs)にmodを追加するとrootにある外部moduleを使えるようになる(`main.rs`に`mod settings`を追加すると、`server.rs`で`crate::settings`が使える)。
