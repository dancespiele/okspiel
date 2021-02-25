use iced::{
    button::{Style, StyleSheet},
    Background, Color, Vector,
};

pub enum ButtonStyles {
    Confirm,
    Delete,
    Success,
    Warnning,
}

impl StyleSheet for ButtonStyles {
    fn active(&self) -> Style {
        match self {
            ButtonStyles::Confirm => Style {
                border_color: Color::from_rgb(1.0, 1.0, 1.0),
                background: Some(Background::Color(Color::from_rgb(0.0, 0.3, 1.0))),
                border_radius: 2.0,
                text_color: Color::from_rgb(1.0, 1.0, 1.0),
                shadow_offset: Vector::new(1.0, 2.0),
                border_width: 0.5,
            },
            ButtonStyles::Delete => Style {
                border_color: Color::from_rgb(1.0, 1.0, 1.0),
                background: Some(Background::Color(Color::from_rgb(0.97, 0.1, 0.1))),
                border_radius: 2.0,
                text_color: Color::from_rgb(1.0, 1.0, 1.0),
                shadow_offset: Vector::new(1.0, 2.0),
                border_width: 0.5,
            },
            ButtonStyles::Success => Style {
                border_color: Color::from_rgb(1.0, 1.0, 1.0),
                background: Some(Background::Color(Color::from_rgb(0.44, 0.62, 0.35))),
                border_radius: 2.0,
                text_color: Color::from_rgb(1.0, 1.0, 1.0),
                shadow_offset: Vector::new(1.0, 2.0),
                border_width: 0.5,
            },
            ButtonStyles::Warnning => Style {
                border_color: Color::from_rgb(0.0, 0.0, 0.0),
                background: Some(Background::Color(Color::from_rgb(0.96, 0.91, 0.33))),
                border_radius: 2.0,
                text_color: Color::from_rgb(0.0, 0.0, 0.0),
                shadow_offset: Vector::new(1.0, 2.0),
                border_width: 0.5,
            },
        }
    }
}
