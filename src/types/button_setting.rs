use serde::{Deserialize, Serialize};

use crate::shared::icon::AppIcon;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ButtonSetting {
    pub text: String,
    pub icon: AppIcon,
}
