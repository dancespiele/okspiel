use iced::{
    button::{Style, StyleSheet},
    Background, Color, Vector,
};

pub enum ButtonStyles {
    Confirm,
    Delete,
}

impl StyleSheet for ButtonStyles {
    fn active(&self) -> Style {
        match self {
            ButtonStyles::Confirm => Style {
                border_color: Color::from_rgb(1.0, 1.0, 1.0),
                background: Some(Background::Color(Color::from_rgb(0.0, 0.3, 1.0))),
                border_radius: 2.0,
                text_color: Color::from_rgb(255.0, 255.0, 255.0),
                shadow_offset: Vector::new(1.0, 2.0),
                border_width: 0.5,
            },
            ButtonStyles::Delete => Style {
                border_color: Color::from_rgb(1.0, 1.0, 1.0),
                background: Some(Background::Color(Color::from_rgb(0.97, 0.1, 0.1))),
                border_radius: 2.0,
                text_color: Color::from_rgb(255.0, 255.0, 255.0),
                shadow_offset: Vector::new(1.0, 2.0),
                border_width: 0.5,
            },
        }
    }
}
