use iced::{
    Background, Color, Theme, border, theme::palette, widget::button
};


pub fn button_transparent(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();
    let base = button_styled(palette.background.base);
    
    match status {
        button::Status::Active => button::Style {
            background: Some(Background::Color(
                Color::TRANSPARENT,
            )),
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(
                palette.background.strong.color,
            )),
            ..base
        },
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(palette.background.weak.color)),
            ..base
        },
        button::Status::Disabled => button_disabled(base),
    }
}

fn button_styled(pair: palette::Pair) -> button::Style {
    button::Style {
        background: Some(Background::Color(pair.color)),
        text_color: pair.text,
        border: border::rounded(2),
        ..button::Style::default()
    }
}

fn button_disabled(style: button::Style) -> button::Style {
    button::Style {
        background: style
            .background
            .map(|background| background.scale_alpha(0.35)),
        text_color: style.text_color.scale_alpha(0.35),
        ..style
    }
}