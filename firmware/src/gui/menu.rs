use embedded_graphics::{
    Drawable,
    mono_font::{MonoTextStyle, ascii::FONT_8X13},
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point, Size},
    primitives::Rectangle,
};
use embedded_text::{
    TextBox,
    alignment::HorizontalAlignment,
    style::{HeightMode, TextBoxStyleBuilder},
};

pub struct Menu<const N: usize> {
    options: [&'static str; N],
    selected: usize,
}

impl<const N: usize> Menu<N> {
    pub fn new(options: [&'static str; N]) -> Self {
        Self {
            options,
            selected: 0,
        }
    }

    pub fn selection(&self) -> usize {
        self.selected
    }

    pub fn scroll(&mut self, mut delta: i32) {
        let count = i32::try_from(self.options.len()).unwrap();
        let selected = i32::try_from(self.selected).unwrap();

        delta %= count;

        // the + count is so that we never drop below 0
        self.selected = usize::try_from((selected + count + delta) % count).unwrap();
    }

    pub fn render<E: core::fmt::Debug, D: DrawTarget<Color = BinaryColor, Error = E>>(
        &self,
        display: &mut D,
    ) {
        let character_style_off = MonoTextStyle::new(&FONT_8X13, BinaryColor::Off);
        let character_style_on = MonoTextStyle::new(&FONT_8X13, BinaryColor::On);
        let textbox_style = TextBoxStyleBuilder::new()
            .height_mode(HeightMode::FitToText)
            .alignment(HorizontalAlignment::Justified)
            .paragraph_spacing(6)
            .build();

        for (i, option) in self.options.iter().enumerate() {
            let top_left = Point::new(0, i32::try_from(i).unwrap() * 14 + 16);

            let selected = i == self.selected;

            if selected {
                display
                    .fill_solid(
                        &Rectangle::new(top_left, Size::new(128, 14)),
                        BinaryColor::On,
                    )
                    .unwrap();
            }

            TextBox::with_textbox_style(
                option,
                Rectangle::new(top_left, Size::new(128, 0)),
                if selected {
                    character_style_off
                } else {
                    character_style_on
                },
                textbox_style,
            )
            .draw(display)
            .unwrap();
        }
    }
}
