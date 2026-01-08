use serde::{Deserialize, Serialize};

use crate::state::button_setting::ButtonSetting;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Deserialize, Serialize, Clone)]
// #[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct SidePanelState {
    // #[serde(skip)]
    pub item3: Vec<ButtonSetting>,

    // #[serde(skip)]
    pub item4: Vec<ButtonSetting>,
}
