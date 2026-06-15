# serpcheap Rust example

Minimal example using the official [`serpcheap`](https://github.com/SerpCheap/serpcheap-rust)
Rust SDK to run a real Google search.

## Quickstart

```bash
SERPCHEAP_API_KEY=your-key cargo run
```

It runs one search (`best running shoes`, `gl=us`), prints each organic result
as `position. title — link`, then a stats line (balance / cost / cached).

## Scraping page content

A second example attaches `ScrapeOptions` to fetch the content of the top
organic results inline:

```bash
SERPCHEAP_API_KEY=your-key cargo run --bin scrape
```

It searches `best running shoes` with scraping enabled
(`render_js`, `screenshot`, `top_n=3`) and, for each organic result, prints the
scraped `content` size (or `scrape_error`) and the `screenshot_url`. Each page
scraped successfully is billed on top of the search.

> Scrape support requires the scrape-capable `serpcheap` SDK, which is not yet
> published to crates.io — the dependency is pinned to git in `Cargo.toml`.

Get an API key at [serp.cheap](https://serp.cheap).
