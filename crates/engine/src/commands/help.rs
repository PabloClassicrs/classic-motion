//! Help menu: community and project links (the frontend opens `Event::OpenUrl`).

use serde_json::{Value, json};

use super::{CommandSpec, always};
use crate::{Result, Session, cmd};

/// Help ▸ Classic Motion Help / Scripting Help / Expression Reference / Effect Reference.
fn docs(_: &mut Session, _: &Value) -> Result<Value> {
    Ok(json!({}))
}

pub fn specs() -> Vec<CommandSpec> {
    vec![
        cmd!("help.docs", "Classic Motion Help...", [], Some("F1"), "{page?: help|scripting|expressions|effects|agents}", always, docs),
        cmd!("help.discord", "Join the Community Discord...", ["Help"], None, "{}", always, |_, _| Ok(json!({}))),
        cmd!("help.website", "Website", ["Help"], None, "{}", always, |_, _| Ok(json!({}))),
        cmd!("help.appPage", "Classic Motion Home Page", ["Help"], None, "{}", always, |_, _| Ok(json!({}))),
        cmd!("help.github", "Classic Motion on GitHub", ["Help"], None, "{}", always, |_, _| Ok(json!({}))),
        cmd!("help.onlineTutorials", "Online Tutorials...", ["Help"], None, "{}", always, |_, _| Ok(json!({}))),
        cmd!("help.inAppTutorials", "In-App Tutorials...", ["Help"], None, "{} → the Home screen's Learn tab", always, |s, p| super::frontend(
            s,
            "help.inAppTutorials",
            p
        )),
        cmd!("help.reportIssue", "Provide Feedback...", ["Help"], None, "{}", always, |_, _| Ok(json!({}))),
    ]
}
