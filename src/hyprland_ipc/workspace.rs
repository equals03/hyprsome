use hyprland::dispatch::{Dispatch, DispatchType, WorkspaceIdentifierWithSpecial};

pub fn focus(workspace_number: &i32) {
    let _ = Dispatch::call(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Id(
        *workspace_number,
    )));
}

pub fn move_to(workspace_number: &i32) {
    let _ = Dispatch::call(DispatchType::MoveToWorkspaceSilent(
        WorkspaceIdentifierWithSpecial::Id(*workspace_number),
        Option::None,
    ));
}

pub fn move_focus(workspace_number: &i32) {
    let _ = Dispatch::call(DispatchType::MoveToWorkspace(
        WorkspaceIdentifierWithSpecial::Id(*workspace_number),
        Option::None,
    ));
}
