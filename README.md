# Wander Notes

Wander Notes は、散歩・思索・読書・場所について記録する日本語中心のサイトです。

## 構成

```
wander-notes/
├── site/            Zola で生成する静的ブログ（config / content / templates / static）
├── workers/
│   └── api/         将来 api.wandernotes.cc で公開する Rust サービスの土台（現状はスタブ）
├── docs/            ドキュメント
├── README.md
└── .gitignore
```

## 公開方針

- `https://wandernotes.cc` は Zola で生成した静的ブログを **Cloudflare Pages** で公開
- `https://api.wandernotes.cc` は Rust + Cloudflare Workers の別サービスとして後から追加

## 初期カテゴリ

- Walks
- Thoughts
- Books
- Places
- About

## ローカル確認

```bash
# サイト（site/ ディレクトリ内で実行）
cd site
zola serve   # ライブリロード付きの開発サーバ
zola build   # public/ に静的サイトを生成
```

```bash
# API
cargo test --manifest-path workers/api/Cargo.toml
```

## デプロイ

サイトは Cloudflare Pages で配信します。Pages のビルド設定は次のとおりです。

- Build command: `zola build`
- Build output directory: `public`
- Root directory: `site`
