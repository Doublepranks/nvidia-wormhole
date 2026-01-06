use iced::widget::container;
use iced::{Background, Border, Color, Theme};

// Brand Colors
pub const NVIDIA_GREEN: Color = Color::from_rgb(0.46, 0.72, 0.0); // #76b900
pub const PANEL_BG: Color = Color::from_rgb(0.12, 0.12, 0.12); // #1f1f1f
pub const ACCENT_BG: Color = Color::from_rgb(0.15, 0.15, 0.15); // #262626
pub const TEXT_DIM: Color = Color::from_rgb(0.60, 0.60, 0.60);

// --- Container Styles (simple closures) ---

pub fn card(_theme: &Theme) -> container::Appearance {
    container::Appearance {
        text_color: Some(Color::WHITE),
        background: Some(Background::Color(PANEL_BG)),
        border: Border {
            color: Color::from_rgb(0.25, 0.25, 0.25),
            width: 1.0,
            radius: 12.0.into(),
        },
        shadow: Default::default(),
    }
}

pub fn metric_card(_theme: &Theme) -> container::Appearance {
    container::Appearance {
        text_color: Some(Color::WHITE),
        background: Some(Background::Color(ACCENT_BG)),
        border: Border {
            color: NVIDIA_GREEN,
            width: 1.0,
            radius: 8.0.into(),
        },
        shadow: Default::default(),
    }
}

pub fn warning_card(_theme: &Theme) -> container::Appearance {
    container::Appearance {
        text_color: Some(Color::WHITE),
        background: Some(Background::Color(Color::from_rgb(0.3, 0.2, 0.0))),
        border: Border {
            color: Color::from_rgb(1.0, 0.6, 0.0),
            width: 1.0,
            radius: 8.0.into(),
        },
        shadow: Default::default(),
    }
}
