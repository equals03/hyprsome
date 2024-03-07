use super::option;
use anyhow::{Context, Result};
use hyprland::data::{Client, Clients, Monitor, Monitors, Transforms};
use hyprland::dispatch::*;
use hyprland::prelude::*;

pub trait MonitorPositions {
    fn is_top_monitor(&self) -> bool;
    fn is_bottom_monitor(&self) -> bool;
    fn is_rightmost_monitor(&self) -> bool;
    fn is_leftmost_monitor(&self) -> bool;
}

impl MonitorPositions for Monitor {
    fn is_top_monitor(&self) -> bool {
        let monitors = match Monitors::get() {
            Ok(m) => m,
            _ => return false,
        };

        let min = match monitors.into_iter().min_by_key(|m| m.y) {
            Some(m) => m,
            _ => return false,
        };

        min.y == self.y
    }

    fn is_bottom_monitor(&self) -> bool {
        let monitors = match Monitors::get() {
            Ok(m) => m,
            _ => return false,
        };

        let max = match monitors.into_iter().max_by_key(|m| m.y) {
            Some(m) => m,
            _ => return false,
        };

        max.y == self.y
    }

    fn is_rightmost_monitor(&self) -> bool {
        let monitors = match Monitors::get() {
            Ok(m) => m,
            _ => return false,
        };

        let max = match monitors.into_iter().max_by_key(|m| m.x) {
            Some(m) => m,
            _ => return false,
        };

        max.x == self.x
    }

    fn is_leftmost_monitor(&self) -> bool {
        let monitors = match Monitors::get() {
            Ok(m) => m,
            _ => return false,
        };

        let min = match monitors.into_iter().min_by_key(|m| m.x) {
            Some(m) => m,
            _ => return false,
        };

        min.x == self.x
    }
}

pub trait MonitorClientPositions {
    fn is_top_client(&self, aw: &Client) -> bool;
    fn is_bottom_client(&self, aw: &Client) -> bool;
    fn is_rightmost_client(&self, aw: &Client) -> bool;
    fn is_leftmost_client(&self, aw: &Client) -> bool;

    fn get_leftmost_client_for_monitor(&self) -> Option<Client>;
}

impl MonitorClientPositions for Monitor {
    fn is_top_client(&self, aw: &Client) -> bool {
        if aw.monitor != self.id {
            return false;
        }

        let gaps = option::get_gaps();

        if self.y + (gaps as i32) + (self.reserved.1 as i32) == (aw.at.1 as i32) {
            return true;
        }

        false
    }

    fn is_bottom_client(&self, aw: &Client) -> bool {
        if aw.monitor != self.id {
            return false;
        }

        let gaps = option::get_gaps();

        if self.real_height() + self.y as f32 - gaps as f32 - self.reserved.1 as f32
            == aw.size.1 as f32 + gaps as f32
        {
            return true;
        }

        false
    }

    fn is_rightmost_client(&self, aw: &Client) -> bool {
        if aw.monitor != self.id {
            return false;
        }

        let gaps = option::get_gaps();

        if self.real_width() + self.x as f32 - gaps as f32 == aw.size.0 as f32 + aw.at.0 as f32 {
            return true;
        }

        false
    }

    fn is_leftmost_client(&self, aw: &Client) -> bool {
        if aw.monitor != self.id {
            return false;
        }

        let gaps = option::get_gaps();

        if (aw.at.0 - gaps) as i32 == self.x {
            return true;
        }

        false
    }

    fn get_leftmost_client_for_monitor(&self) -> Option<Client> {
        let clients = match Clients::get() {
            Ok(clients) => clients,
            _ => return None,
        };

        clients
            .into_iter()
            .filter(|c| c.monitor == self.id)
            .min_by_key(|c| c.at.0)
    }
}

pub trait MonitorDimensions {
    fn real_width(&self) -> f32;
    fn real_height(&self) -> f32;
}

impl MonitorDimensions for Monitor {
    fn real_width(&self) -> f32 {
        match self.transform {
            Transforms::Normal
            | Transforms::Normal180
            | Transforms::Flipped
            | Transforms::Flipped180 => self.width as f32 / self.scale,
            Transforms::Normal90 | Transforms::Normal270 | Transforms::Flipped90 => {
                self.height as f32 / self.scale
            }
            _ => self.width as f32,
        }
    }

    fn real_height(&self) -> f32 {
        match self.transform {
            Transforms::Normal
            | Transforms::Flipped
            | Transforms::Normal180
            | Transforms::Flipped180 => self.height as f32 / self.scale,
            Transforms::Normal90 | Transforms::Normal270 | Transforms::Flipped90 => {
                self.width as f32 / self.scale
            }
            _ => self.height as f32,
        }
    }
}

pub fn get_monitors() -> Result<Monitors> {
    Ok(Monitors::get()?)
}

pub fn get_active() -> Result<Option<Monitor>> {
    Ok(Monitors::get()?.into_iter().find(|m| m.focused))
}

pub fn get_by_id(id: i32) -> Result<Option<Monitor>> {
    Ok(Monitors::get()?.into_iter().find(|m| m.id == id as i128))
}

pub fn get_by_name(name: &str) -> Result<Option<Monitor>> {
    Ok(Monitors::get()?.into_iter().find(|m| m.name == name))
}
pub fn get_count() -> Result<i32> {
    Ok(Monitors::get()?.count() as i32)
}

pub fn focus(id: u8) -> Result<()> {
    Dispatch::call(DispatchType::FocusMonitor(MonitorIdentifier::Id(id)))?;

    Ok(())
}
pub fn focus_left() -> Result<()> {
    Dispatch::call(DispatchType::FocusMonitor(MonitorIdentifier::Direction(
        Direction::Left,
    )))?;

    Ok(())
}
pub fn focus_right() -> Result<()> {
    Dispatch::call(DispatchType::FocusMonitor(MonitorIdentifier::Direction(
        Direction::Right,
    )))?;

    Ok(())
}

pub fn focus_up() -> Result<()> {
    Dispatch::call(DispatchType::FocusMonitor(MonitorIdentifier::Direction(
        Direction::Up,
    )))?;

    Ok(())
}

pub fn focus_down() -> Result<()> {
    Dispatch::call(DispatchType::FocusMonitor(MonitorIdentifier::Direction(
        Direction::Down,
    )))?;

    Ok(())
}
