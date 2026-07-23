# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

Wander Notes is a Japanese-language notes site (walks, thoughts, books, places). Content and copy are written in Japanese — match that when editing content or user-facing UI strings.

The repo holds **two independent services** that ship separately:

1. **The static blog** (`https://wandernotes.cc`) — the `site/` directory, built with [Zola](https://www.getzola.org/) and deployed via **Cloudflare Pages** (build command `zola build`, output `public/`, root directory `site`). There is no server-side logic and no serving Worker.
2. **The API** (`https://api.wandernotes.cc`) — the Rust crate under `workers/api/`. It is currently a **stub/scaffold** (`workers/api/src/lib.rs` only declares an API host constant and a list of planned features). It is not yet a Cloudflare Worker: there is no `worker` crate dependency, wasm target, or `wrangler.toml`. Becoming a real Worker is future work. For now treat it as a placeholder for the planned features (contact form, view/like counts, walk-log API, search/AI).

## Commands

```bash
# Site — run inside site/
cd site
zola serve   # local dev server with live reload
zola build   # generate the site into site/public/ (gitignored)

# API
cargo test --manifest-path workers/api/Cargo.toml
```

The Rust crate uses **edition 2024**, so a recent stable toolchain is required.

## Architecture

**Site generation flow:** `site/content/*.md` + `site/templates/*.html` → `zola build` → `site/public/` → Cloudflare Pages serves it. There is no runtime component for the site.

**Content structure** (`site/content/`): each top-level directory is a Zola *section* backed by an `_index.md` (TOML frontmatter delimited by `+++`). The four sections — `walks`, `thoughts`, `books`, `places` — plus the standalone page `about.md` correspond to the nav links hardcoded in `site/templates/base.html` (`/walks/`, `/thoughts/`, `/books/`, `/places/`, `/about/`). To add an entry, create a Markdown file with `+++` frontmatter (`title`, `date`, `description`) in the relevant section directory. Sections use `sort_by = "date"` to order entries.

**Templates** (`site/templates/`) use Tera and all extend `base.html`:
- `index.html` — home page; renders a card grid by iterating `section.subsections`.
- `section.html` — a category landing page; lists `section.pages`.
- `page.html` — an individual entry (also renders standalone pages like `about.md`).

Adding a nav link or a new section requires editing both `site/templates/base.html` (the nav is static markup) and creating the matching `site/content/<name>/_index.md`.

**Config:** `site/config.toml` (Zola: base_url, language `ja`, Atom feed enabled). Styling is a single hand-written `site/static/css/styles.css` — `compile_sass = false`, so there is no CSS build step; templates reference it via `get_url(path='css/styles.css')`.
