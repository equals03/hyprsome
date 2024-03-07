use crate::{
    hyprland_ipc::{client, monitor, workspace},
    MAX_GROUP_WS,
};
use anyhow::Result;

pub fn init_workspaces(workspace_index: i32) -> Result<()> {
    for monitor in monitor::get_monitors()? {
        let monitor_id = monitor.id as i32;
        let workspace_id = (monitor_id + 1) * MAX_GROUP_WS + workspace_index;

        monitor::focus(monitor_id as u8)?;
        workspace::switch_to(workspace_id)?;
    }

    Ok(())
}

pub fn rearrange_workspaces() -> Result<()> {
    let monitors_len = monitor::get_count()?;

    for workspace in workspace::get_all()? {
        let group_index = workspace.id / MAX_GROUP_WS;
        if group_index < monitors_len {
            let monitor_id = group_index.max(1) - 1;
            workspace::move_to_monitor(workspace.id, monitor_id as u8)?;
        } else {
            workspace::move_to_monitor(workspace.id, (monitors_len - 1) as u8)?;
        }
    }
    Ok(())
}

pub fn workspace(workspace_index: i32) -> Result<()> {
    if let Some(active) = monitor::get_active()? {
        let monitor_id = active.id as i32;
        let workspace_number = (monitor_id + 1) * MAX_GROUP_WS + workspace_index;

        workspace::switch_to(workspace_number)?;
    }

    Ok(())
}

pub fn workspace_all(workspace_index: i32) -> Result<()> {
    let active_monitor = monitor::get_active()?;

    for monitor in monitor::get_monitors()? {
        let monitor_id = monitor.id as i32;
        let workspace_id = (monitor_id + 1) * MAX_GROUP_WS + workspace_index;

        monitor::focus(monitor_id as u8)?;
        workspace::switch_to(workspace_id)?;
    }

    if let Some(active_monitor) = active_monitor {
        let monitor_id = active_monitor.id as i32;
        let workspace_id = (monitor_id + 1) * MAX_GROUP_WS + workspace_index;

        monitor::focus(monitor_id as u8)?;
        workspace::switch_to(workspace_id)?;
    }

    Ok(())
}

pub fn move_to_workspace(workspace_index: i32, silent: bool) -> Result<()> {
    if let Some(active) = monitor::get_active()? {
        let monitor_id = active.id as i32;
        let workspace_id = (monitor_id + 1) * MAX_GROUP_WS + workspace_index;
        dbg!(workspace_id);

        client::move_to_workspace(workspace_id, silent, None)?;
    }

    Ok(())
}

pub fn move_current_workspace_to_monitor(monitor_index: i32, silent: bool) -> Result<()> {
    if let Some(target_monitor) = monitor::get_by_id(monitor_index)? {
        let target_monitor_id = target_monitor.id as i32;

        if let Some(source_workspace) = workspace::get_active()? {
            if source_workspace.monitor == target_monitor.name {
                // nothing to do. they are on the same monitor.
                return Ok(());
            }

            if let Some(source_monitor) = monitor::get_by_name(&source_workspace.monitor)? {
                let source_monitor_id = source_monitor.id as i32;
                let workspace_index =
                    source_workspace.id - ((source_monitor_id + 1) * MAX_GROUP_WS);
                let destination_id = (target_monitor_id + 1) * MAX_GROUP_WS + workspace_index;

                dbg!(
                    source_workspace.id,
                    source_monitor_id,
                    target_monitor_id,
                    workspace_index,
                    destination_id
                );
                // workspace::switch_to(destination_id)?;
                // workspace::move_to_monitor(destination_id, target_monitor_id as u8)?;

                // for client in client::get_all_on_workspace(source_workspace.id)? {
                //     client::move_to_workspace(destination_id, silent, Some(client.address))?
                // }
            }
        }
    }

    Ok(())
}
