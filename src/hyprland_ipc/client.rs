use super::monitor::{self, MonitorClientPositions, MonitorPositions};
use anyhow::Result;
use hyprland::{
    data::{Client, Clients},
    dispatch::{
        Direction, Dispatch, DispatchType, WindowIdentifier, WorkspaceIdentifierWithSpecial,
    },
    prelude::*,
    shared::Address,
};

pub trait ClientFocus {
    fn focus_up(&self) -> Result<()>;
    fn focus_down(&self) -> Result<()>;
    fn focus_right(&self) -> Result<()>;
    fn focus_left(&self) -> Result<()>;
}

impl ClientFocus for Client {
    fn focus_up(&self) -> Result<()> {
        let monitor = match monitor::get_by_id(self.monitor as i32)? {
            Some(m) => m,
            None => return Ok(()),
        };

        let is_top_client = monitor.is_top_client(self);

        if is_top_client && monitor.is_top_monitor() {
            return Ok(());
        }

        if is_top_client {
            monitor::focus_up()?;
            return Ok(());
        }

        focus_by_direction(Direction::Up)?;

        Ok(())
    }
    fn focus_down(&self) -> Result<()> {
        let monitor = match monitor::get_by_id(self.monitor as i32)? {
            Some(m) => m,
            None => return Ok(()),
        };

        let is_bottom_client = monitor.is_bottom_client(self);

        if is_bottom_client && monitor.is_bottom_monitor() {
            return Ok(());
        }

        if is_bottom_client {
            monitor::focus_down()?;
            return Ok(());
        }

        focus_by_direction(Direction::Down)?;

        Ok(())
    }

    fn focus_right(&self) -> Result<()> {
        let monitor = match monitor::get_by_id(self.monitor as i32)? {
            Some(m) => m,
            None => return Ok(()),
        };

        let is_rightmost_client = monitor.is_rightmost_client(self);

        if is_rightmost_client && monitor.is_rightmost_monitor() {
            return Ok(());
        }

        if is_rightmost_client {
            monitor::focus_right()?;
            return Ok(());
        }

        focus_by_direction(Direction::Right)?;

        Ok(())
    }
    fn focus_left(&self) -> Result<()> {
        let monitor = match monitor::get_by_id(self.monitor as i32)? {
            Some(m) => m,
            None => return Ok(()),
        };

        let is_leftmost_client = monitor.is_leftmost_client(self);

        if is_leftmost_client && monitor.is_leftmost_monitor() {
            return Ok(());
        }

        if is_leftmost_client {
            monitor::focus_left()?;
            return Ok(());
        }

        focus_by_direction(Direction::Left)?;

        Ok(())
    }
}

pub fn get_all_on_workspace(workspace_id: i32) -> Result<Vec<Client>> {
    let clients: Vec<Client> = Clients::get()?
        .into_iter()
        .filter(|c| c.workspace.id == workspace_id)
        .collect();

    Ok(clients)
}

pub fn move_to_workspace(
    workspace_id: i32,
    silently: bool,
    address: Option<Address>,
) -> Result<()> {
    let window_identifier = match address {
        Some(address) => Some(WindowIdentifier::Address(address)),
        _ => None,
    };

    let workspace_identifier = WorkspaceIdentifierWithSpecial::Id(workspace_id);

    let dispatch_type = if silently {
        DispatchType::MoveToWorkspaceSilent(workspace_identifier, window_identifier)
    } else {
        DispatchType::MoveToWorkspace(workspace_identifier, window_identifier)
    };

    Ok(Dispatch::call(dispatch_type)?)
}

pub fn focus_by_direction(direction: Direction) -> Result<()> {
    Dispatch::call(DispatchType::MoveFocus(direction))?;

    Ok(())
}

// pub fn move_active_to_workspace(workspace_id: i32) -> Result<()> {
//     Dispatch::call(DispatchType::MoveToWorkspace(
//         WorkspaceIdentifierWithSpecial::Id(workspace_id),
//         None,
//     ))?;

//     Ok(())
// }
