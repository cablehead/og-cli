
cli frontend for [opengraph-rs](https://crates.io/crates/opengraph-rs) crate

usage:

```nushell
http get https://www.rust-lang.org | og-cli | from json
──────────────────┬─────────────────────────────────────────────────────────────────────────────────────────────────────────
 type             │ website
 title            │
 url              │
                  │ ───┬──────┬──────────────────────────────────────────────────────────────┬────────────┬───────┬────────
 images           │  # │ type │                             url                              │ secure_url │ width │ height
                  │ ───┼──────┼──────────────────────────────────────────────────────────────┼────────────┼───────┼────────
                  │  0 │      │ https://www.rust-lang.org/static/images/rust-social-wide.jpg │            │       │
                  │ ───┴──────┴──────────────────────────────────────────────────────────────┴────────────┴───────┴────────
 audios           │ [list 0 items]
 videos           │ [list 0 items]
 description      │ A language empowering everyone to build reliable and efficient software.
 determiner       │
 locale           │ en_US
 locale_alternate │ [list 0 items]
 site_name        │
──────────────────┴─────────────────────────────────────────────────────────────────────────────────────────────────────────
```
