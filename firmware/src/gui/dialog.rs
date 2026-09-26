use embedded_graphics::{
    Drawable,
    mono_font::{MonoTextStyle, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point, Size},
    primitives::Rectangle,
};
use embedded_text::{
    TextBox,
    alignment::HorizontalAlignment,
    style::{HeightMode, TextBoxStyleBuilder},
};

pub struct Dialog {
    text: &'static str,
}

impl Dialog {
    pub fn new(text: &'static str) -> Self {
        Self { text }
    }

    pub fn render<E: core::fmt::Debug, D: DrawTarget<Color = BinaryColor, Error = E>>(
        &self,
        display: &mut D,
    ) {
        let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
        let textbox_style = TextBoxStyleBuilder::new()
            .height_mode(HeightMode::FitToText)
            .alignment(HorizontalAlignment::Center)
            .build();

        TextBox::with_textbox_style(
            self.text,
            Rectangle::new(Point::new(0, 16), Size::new(128, 0)),
            character_style,
            textbox_style,
        )
        .draw(display)
        .unwrap();
    }
}
