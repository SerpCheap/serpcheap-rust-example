use serpcheap::{Client, ScrapeOptions, SearchParams};

fn main() {
    let api_key = match std::env::var("SERPCHEAP_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Set SERPCHEAP_API_KEY to run this example, e.g.:");
            eprintln!("  SERPCHEAP_API_KEY=your-key cargo run --bin scrape");
            std::process::exit(1);
        }
    };

    let client = match Client::builder(api_key).build() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[{}] {}", e.code.as_str(), e.message);
            std::process::exit(1);
        }
    };

    let params = SearchParams::new("best running shoes").gl("us").scrape(
        ScrapeOptions::new()
            .render_js(true)
            .screenshot(true)
            .top_n(3),
    );

    match client.search(&params) {
        Ok(res) => {
            for r in &res.organic {
                println!("{}. {} — {}", r.position, r.title, r.link);
                match (&r.content, &r.scrape_error) {
                    (Some(content), _) => println!("   content: {} bytes", content.len()),
                    (_, Some(err)) => println!("   scrape failed: {}", err),
                    _ => {}
                }
                if let Some(url) = &r.screenshot_url {
                    println!("   screenshot: {}", url);
                }
            }
            if let Some(s) = res.stats {
                println!(
                    "\nstats: balance={} cost={} cached={}",
                    s.balance, s.cost, s.cached
                );
            }
        }
        Err(e) => {
            eprintln!("[{}] {}", e.code.as_str(), e.message);
            std::process::exit(1);
        }
    }
}
