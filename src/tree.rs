use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Node<'a> {
    name: Option<&'a str>,
    #[serde(default)]
    nodes: Vec<Node<'a>>,
}
