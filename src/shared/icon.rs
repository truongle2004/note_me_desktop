use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum AppIcon {
    Search,
    Home,
    Inbox,
    Settings,
    Marketplace,
    Trash,
}

pub fn get_icon(icon: AppIcon) -> &'static str {
    match icon {
        AppIcon::Search => "󰍉",      // nf-md-magnify
        AppIcon::Home => "󰋜",        // nf-md-home
        AppIcon::Inbox => "󰮍",       // nf-md-inbox
        AppIcon::Marketplace => "󰏓", // nf-md-store
        AppIcon::Settings => "󰒓",    // nf-md-cog
        AppIcon::Trash => "󰩺",       // nf-md-trash_can
    }
}
