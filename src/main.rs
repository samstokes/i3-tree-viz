mod tree;

use std::io::{stdin, BufRead};

use eyre::Context;
use log::{debug, info, trace};

use tree::Node;

fn main() -> eyre::Result<()> {
    env_logger::init();

    let mut filtered_input = String::with_capacity(1024);
    for line in stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .filter(is_not_comment)
    {
        trace!("non-comment line: {}", line);
        filtered_input.push_str(&line);
        filtered_input.push('\n');
    }

    debug!("filtered input: {}", filtered_input);

    let forest = serde_json::Deserializer::from_str(&filtered_input)
        .into_iter()
        .collect::<Result<Vec<Node>, serde_json::Error>>()
        .wrap_err("invalid JSON")?;

    for tree in forest {
        info!("{:?}", tree);
    }

    Ok(())
}

fn is_not_comment(line: &String) -> bool {
    trace!("line: {}", line);
    !line.trim_start().starts_with("//")
}
