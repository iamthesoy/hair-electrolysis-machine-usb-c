use core::{fmt::Write, sync::atomic::Ordering};

use arrayvec::ArrayString;
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

use crate::{
    buzzer::{BUZZER_COMMAND_CHANNEL, BuzzerCommand},
    settings::{MAX_CURRENT_UA, RAMP_TIME_MS, SAVE_SETTINGS_CHANNEL, TARGET_LYE_LU},
};

pub struct Settings {
    max_current_ua: i16,
    target_lye_lu: i16,
    ramp_time_ms: i16,
    selected: usize,
    clicked: bool,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            max_current_ua: MAX_CURRENT_UA.load(Ordering::Relaxed).try_into().unwrap(),
            target_lye_lu: TARGET_LYE_LU.load(Ordering::Relaxed).try_into().unwrap(),
            ramp_time_ms: RAMP_TIME_MS.load(Ordering::Relaxed).try_into().unwrap(),
            selected: 0,
            clicked: false,
        }
    }

    pub fn scroll(&mut self, mut delta: i16) {
        if self.clicked {
            match self.selected {
                0 => self.max_current_ua = (self.max_current_ua + delta * 50).clamp(0, 2300),
                1 => self.target_lye_lu = (self.target_lye_lu + delta).clamp(1, 80),
                2 => self.ramp_time_ms = (self.ramp_time_ms + delta * 25).clamp(0, 5000),
                _ => {}
            }
        } else {
            let count = 5;
            let selected = i16::try_from(self.selected).unwrap();

            delta %= count;

            // the + count is so that we never drop below 0
            self.selected = usize::try_from((selected + count + delta) % count).unwrap();
        }
    }

    // returns true on exit
    pub async fn click(&mut self) -> bool {
        match self.selected {
            3 => {
                // save
                BUZZER_COMMAND_CHANNEL.send(BuzzerCommand::ACCEPT).await;

                MAX_CURRENT_UA.store(self.max_current_ua.try_into().unwrap(), Ordering::Relaxed);
                TARGET_LYE_LU.store(self.target_lye_lu.try_into().unwrap(), Ordering::Relaxed);
                RAMP_TIME_MS.store(self.ramp_time_ms.try_into().unwrap(), Ordering::Relaxed);

                SAVE_SETTINGS_CHANNEL.signal(());

                return true;
            }
            4 => {
                // cancel
                BUZZER_COMMAND_CHANNEL.send(BuzzerCommand::CANCEL).await;
                return true;
            }
            _ => {
                BUZZER_COMMAND_CHANNEL
                    .send(if self.clicked {
                        BuzzerCommand::ACCEPT
                    } else {
                        BuzzerCommand::OK
                    })
                    .await;
            }
        }

        self.clicked = !self.clicked;

        false
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

        let textbox_editing_style = TextBoxStyleBuilder::new()
            .height_mode(HeightMode::FitToText)
            .alignment(HorizontalAlignment::Center)
            .paragraph_spacing(6)
            .build();

        for i in if self.selected < 3 { 0..3 } else { 3..5 } {
            let top_left = Point::new(0, i32::try_from(i % 3).unwrap() * 14 + 16);

            let selected = i == self.selected;

            let (preamble, postamble) = if selected && self.clicked {
                ("> ", " <")
            } else {
                ("", "")
            };

            let mut text = ArrayString::<32>::new();

            match i {
                0 => write!(
                    &mut text,
                    "{}Cur: {:.2} mA{}",
                    preamble,
                    self.max_current_ua as f64 / 1000.0,
                    postamble
                ),
                1 => write!(
                    &mut text,
                    "{}Lye: {:} LU{}",
                    preamble, self.target_lye_lu, postamble
                ),
                2 => write!(
                    &mut text,
                    "{}Ramp: {:04} ms{}",
                    preamble, self.ramp_time_ms, postamble
                ),
                3 => write!(&mut text, "Save"),
                4 => write!(&mut text, "Cancel"),
                _ => unreachable!(),
            }
            .unwrap();

            if selected {
                display
                    .fill_solid(
                        &Rectangle::new(top_left, Size::new(128, 14)),
                        BinaryColor::On,
                    )
                    .unwrap();
            }

            TextBox::with_textbox_style(
                text.as_str(),
                Rectangle::new(top_left, Size::new(128, 0)),
                if selected {
                    character_style_off
                } else {
                    character_style_on
                },
                if selected && self.clicked {
                    textbox_editing_style
                } else {
                    textbox_style
                },
            )
            .draw(display)
            .unwrap();
        }
    }
}
