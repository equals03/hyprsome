use anyhow::{Context, Result};
use hyprland::{
    data::{Client, Workspace, Workspaces},
    dispatch::{
        Dispatch, DispatchType, MonitorIdentifier, WorkspaceIdentifier,
        WorkspaceIdentifierWithSpecial,
    },
    prelude::*,
};

use crate::MAX_GROUP_WS;

pub fn get_all() -> Result<Workspaces> {
    Ok(Workspaces::get()?)
}

pub fn get_by_id(id: i32) -> Result<Option<Workspace>> {
    Ok(Workspaces::get()?.into_iter().find(|w| w.id == id))
}

pub fn get_by_group(group: i32) -> Result<Option<Workspace>> {
    Ok(Workspaces::get()?
        .into_iter()
        .find(|w| w.id / MAX_GROUP_WS == group))
}

pub fn get_active() -> Result<Option<Workspace>> {
    if let Some(active) = Client::get_active()? {
        Ok(get_by_id(active.workspace.id)?)
    } else {
        Ok(None)
    }
}

pub fn switch_to(workspace_id: i32) -> Result<()> {
    Dispatch::call(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Id(
        workspace_id,
    )))
    .with_context(|| {
        format!(
            "failure when trying to dispatch workspace [workspace_id: {}]",
            workspace_id
        )
    })?;

    Ok(())
}
pub fn move_to_monitor(workspace_id: i32, monitor_id: u8) -> Result<()> {
    Dispatch::call(DispatchType::MoveWorkspaceToMonitor(
        WorkspaceIdentifier::Id(workspace_id),
        MonitorIdentifier::Id(monitor_id as i128),
    )).with_context(|| format!("failure when trying to dispatch move workspace to monitor [workspace_id: {}, monitor_id: {}]", workspace_id, monitor_id))?;

    Ok(())
}

// commands
