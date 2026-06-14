# serpcheap Rust example

Minimal example using the official [`serpcheap`](https://github.com/SerpCheap/serpcheap-rust)
Rust SDK to run a real Google search.

## Quickstart

```bash
SERPCHEAP_API_KEY=your-key cargo run
```

It runs one search (`best running shoes`, `gl=us`), prints each organic result
as `position. title — link`, then a stats line (balance / cost / cached).

Get an API key at [serp.cheap](https://serp.cheap).
