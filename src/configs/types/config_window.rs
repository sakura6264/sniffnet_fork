use iced::window::Position;
use iced::{Point, Size};
use serde::{Deserialize, Serialize};
/*
#[cfg(not(test))]
use crate::SNIFFNET_LOWERCASE;
*/
#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
pub struct ConfigWindow {
    pub position: (i32, i32),
    pub size: (u32, u32),
}

impl ConfigWindow {
    const FILE_NAME: &'static str = "window";
    #[cfg(not(test))]
    pub fn load() -> Self {
        if let Ok(window) = crate::utils::confy_proxy_load::<ConfigWindow>(Self::FILE_NAME) {
            window
        } else {
            crate::utils::confy_proxy_store(Self::FILE_NAME, ConfigWindow::default()).unwrap_or(());
            ConfigWindow::default()
        }
    }

    #[cfg(not(test))]
    pub fn store(self) {
        crate::utils::confy_proxy_store(Self::FILE_NAME, self).unwrap_or(());
    }
}

impl Default for ConfigWindow {
    fn default() -> Self {
        Self {
            position: (0, 0),
            size: (1190, 670),
        }
    }
}

pub trait ToPosition {
    fn to_position(self) -> Position;
}

impl ToPosition for (i32, i32) {
    fn to_position(self) -> Position {
        #[allow(clippy::cast_precision_loss)]
        Position::Specific(Point {
            x: self.0 as f32,
            y: self.1 as f32,
        })
    }
}

pub trait ToSize {
    fn to_size(self) -> Size;
}

impl ToSize for (u32, u32) {
    fn to_size(self) -> Size {
        #[allow(clippy::cast_precision_loss)]
        Size {
            width: self.0 as f32,
            height: self.1 as f32,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ConfigWindow;

    impl ConfigWindow {
        pub fn test_path() -> String {
            format!("{}/{}.toml", env!("CARGO_MANIFEST_DIR"), Self::FILE_NAME)
        }

        pub fn load() -> Self {
            confy::load_path::<ConfigWindow>(ConfigWindow::test_path())
                .unwrap_or_else(|_| ConfigWindow::default())
        }

        pub fn store(self) {
            confy::store_path(ConfigWindow::test_path(), self).unwrap_or(());
        }
    }
}
