use std::env;
use std::io::{self, BufRead};

use rs_prom2json::{docs2writer, samples2writer, scrape2writer};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let (show_docs, show_samples) = match args.len() {
        1 => (true, true),
        2 if args[1] == "--all" => (true, true),
        2 if args[1] == "--docs" => (true, false),
        2 if args[1] == "--samples" => (false, true),
        _ => {
            eprintln!("Usage: prom2json [--all|--docs|--samples]");
            return Ok(());
        }
    };

    let stdin = io::stdin();
    let reader = stdin.lock();
    let lines = reader.lines();

    let scrape = rs_prom2json::parse(lines)?;

    let stdout = io::stdout();
    let writer = stdout.lock();

    if show_docs && show_samples {
        scrape2writer(&scrape, writer)
    } else if show_docs {
        docs2writer(&scrape, writer)
    } else {
        samples2writer(&scrape, writer)
    }
}
