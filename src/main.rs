use serpcheap::{Client, SearchParams};

fn main() {
    let api_key = match std::env::var("SERPCHEAP_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Set SERPCHEAP_API_KEY to run this example, e.g.:");
            eprintln!("  SERPCHEAP_API_KEY=your-key cargo run");
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

    let params = SearchParams::new("best running shoes").gl("us");

    match client.search(&params) {
        Ok(res) => {
            for r in &res.organic {
                println!("{}. {} — {}", r.position, r.title, r.link);
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
