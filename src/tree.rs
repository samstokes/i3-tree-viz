use eyre::{eyre, Context};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum ContainerKind {
    /// container
    #[serde(rename = "con")]
    Container,
    #[serde(rename = "floating_con")]
    Floating,
}

impl ContainerKind {
    pub fn as_suffix(&self) -> Option<&'static str> {
        match self {
            ContainerKind::Floating => Some("(float)"),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct Layout<'a>(&'a str);

#[derive(Debug, Deserialize)]
pub struct Node<'a> {
    name: Option<&'a str>,
    #[serde(rename = "type")]
    kind: ContainerKind,
    layout: Option<Layout<'a>>,
    #[serde(default, rename = "nodes")]
    children: Vec<Node<'a>>,
}

pub type Dag = daggy::Dag<String, &'static str>;

impl<'a> Node<'a> {
    pub fn description(&self) -> String {
        let mut parts = Vec::with_capacity(3);

        if let Some(name) = self.name {
            parts.push(name.to_string());
        }

        if let Some(Layout(layout)) = self.layout {
            parts.push(format!("<{}>", layout));
        }

        if let Some(suffix) = self.kind.as_suffix() {
            parts.push(suffix.to_string());
        }

        if parts.is_empty() {
            parts.push("[anon]".to_string());
        }

        parts.join(" ")
    }

    pub fn validate(&self) -> eyre::Result<()> {
        match (&self.layout, self.children.as_slice()) {
            (Some(layout), []) => Err(eyre!("container with layout {:?} but no children!", layout)),
            (None, children) if !children.is_empty() => Err(eyre!(
                "container with children {:?} but no layout!",
                children
            )),
            _ => Ok(()),
        }
        .wrap_err(
            self.name
                .map(|s| s.to_string())
                .unwrap_or_else(|| "anonymous".to_string()),
        )?;

        for child in &self.children {
            child.validate()?;
        }

        Ok(())
    }

    pub fn add_to_dag(
        &self,
        dag: &mut Dag,
        parent_ix: Option<daggy::NodeIndex>,
    ) -> daggy::NodeIndex {
        let ix = if let Some(parent_ix) = parent_ix {
            dag.add_child(parent_ix, "", self.description()).1
        } else {
            dag.add_node(self.description())
        };

        for child in &self.children {
            child.add_to_dag(dag, Some(ix));
        }

        ix
    }
}

pub fn forest_as_dag<'a>(roots: impl Iterator<Item = &'a Node<'a>>) -> Dag {
    let mut dag = Dag::new();
    for root in roots {
        root.add_to_dag(&mut dag, None);
    }
    dag
}
