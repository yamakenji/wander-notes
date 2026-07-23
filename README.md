# Wander Notes

Wander Notes は、散歩・思索・読書・場所について記録する日本語中心のサイトです。

## 構成

- `config.toml` — Zola のサイト設定
- `content/` — Markdown コンテンツ
- `templates/` — Zola テンプレート
- `static/` — 配信用の静的アセット
- `workers/site/` — `public/` を配信する Cloudflare Worker
- `api/` — 将来 `https://api.wandernotes.cc` で公開する Rust API サービスの土台

## 公開方針

- `https://wandernotes.cc` は Zola で生成した静的ブログを公開
- `https://api.wandernotes.cc` は Rust + Cloudflare Workers の別サービスとして後から追加

## 初期カテゴリ

- Walks
- Thoughts
- Books
- Places
- About

## ローカル確認

```bash
zola serve
```

```bash
zola build
```

```bash
cargo test --manifest-path api/Cargo.toml
```