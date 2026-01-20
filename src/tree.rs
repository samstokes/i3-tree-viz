use serde::Deserialize;
use std::collections::HashMap;
use petgraph::graph::DiGraph;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AerospaceWindow {
    #[serde(rename = "window-id")]
    pub window_id: i64,
    #[serde(rename = "app-name")]
    pub app_name: Option<String>,
    #[serde(rename = "window-title")]
    pub window_title: Option<String>,
    #[serde(rename = "workspace")]
    pub workspace: Option<String>,
    #[serde(rename = "app-bundle-id")]
    pub app_bundle_id: Option<String>,
}

pub type Dag = DiGraph<String, &'static str>;

impl AerospaceWindow {
    pub fn description(&self) -> String {
        let mut parts = Vec::new();

        if let Some(title) = &self.window_title {
            if !title.is_empty() {
                parts.push(title.clone());
            }
        }

        if let Some(app_name) = &self.app_name {
            parts.push(format!("({})", app_name));
        }

        if parts.is_empty() {
            parts.push(format!("Window {}", self.window_id));
        }

        parts.join(" ")
    }

    pub fn add_to_dag(
        &self,
        dag: &mut Dag,
        parent_ix: petgraph::graph::NodeIndex,
    ) -> petgraph::graph::NodeIndex {
        let window_ix = dag.add_node(self.description());
        dag.add_edge(parent_ix, window_ix, "");
        window_ix
    }
}

pub fn windows_as_dag(windows: &[AerospaceWindow]) -> Dag {
    let mut dag = Dag::new();

    // Group windows by workspace
    let mut by_workspace: HashMap<String, Vec<&AerospaceWindow>> = HashMap::new();

    for window in windows {
        let workspace_name = window.workspace.as_deref().unwrap_or("unknown");
        by_workspace.entry(workspace_name.to_string())
            .or_insert_with(Vec::new)
            .push(window);
    }

    // Sort workspaces by name for consistent output
    let mut workspace_names: Vec<_> = by_workspace.keys().cloned().collect();
    workspace_names.sort();

    for workspace_name in workspace_names {
        let workspace_windows = &by_workspace[&workspace_name];

        // Create workspace node
        let workspace_node = dag.add_node(format!("Workspace: {}", workspace_name));

        // Add all windows in this workspace
        for window in workspace_windows {
            window.add_to_dag(&mut dag, workspace_node);
        }
    }

    dag
}
