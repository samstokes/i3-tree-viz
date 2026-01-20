mod tree;

use std::io::{stdin, Read};

use eyre::Context;
use log::debug;

use petgraph::dot::Dot;
use tree::AerospaceWindow;

use crate::tree::windows_as_dag;

fn main() -> eyre::Result<()> {
    env_logger::init();

    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .wrap_err("failed to read stdin")?;

    debug!("input: {}", input);

    let windows: Vec<AerospaceWindow> = serde_json::from_str(&input)
        .wrap_err("invalid JSON - expected array of window objects from 'aerospace list-windows --all --json'")?;

    debug!("parsed {} windows", windows.len());
    for window in &windows {
        debug!("{:?}", window);
    }

    let dag = windows_as_dag(&windows);

    let dot = Dot::new(&dag);

    println!("{}", dot);

    Ok(())
}
