use core::{fmt::Write, sync::atomic::Ordering};

use arrayvec::ArrayString;
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

use crate::{
    output_handler::{OutputHandler, RunStatus},
    settings::{MAX_CURRENT_UA, TARGET_LYE_LU},
};

pub struct RunView {
    status: RunStatus,
    max_current_ua: u16,
    target_lye_lu: u16,
}

impl RunView {
    pub fn new() -> Self {
        Self {
            status: RunStatus::default(),
            max_current_ua: MAX_CURRENT_UA.load(Ordering::Relaxed),
            target_lye_lu: TARGET_LYE_LU.load(Ordering::Relaxed),
        }
    }

    pub async fn render<E: core::fmt::Debug, D: DrawTarget<Color = BinaryColor, Error = E>>(
        &mut self,
        display: &mut D,
    ) -> ArrayString<12> {
        if let Some(status) = OutputHandler::try_get_run_status() {
            self.status = status;
        }

        let mut s = ArrayString::<128>::new();
        write!(
            &mut s,
            "{:.2} LU @ {:.2}mA\n\
                     Cur   {:.2} mA ({:.2})\n\
                     Lye   {:.2} LU \n\
                     Body  {:.2} ohm",
            self.target_lye_lu,
            self.max_current_ua as f64 / 1000.0,
            self.status.measured_current_ma,
            self.status.measured_current_ma - self.status.current_ma,
            self.status.lye_units,
            self.status.body_resistance
        )
        .unwrap();

        let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
        let textbox_style = TextBoxStyleBuilder::new()
            .height_mode(HeightMode::FitToText)
            .alignment(HorizontalAlignment::Left)
            .build();

        TextBox::with_textbox_style(
            s.as_str(),
            Rectangle::new(Point::new(0, 16), Size::new(128, 0)),
            character_style,
            textbox_style,
        )
        .draw(display)
        .unwrap();

        let mut title = ArrayString::new();
        write!(
            &mut title,
            "{:05.2} s",
            self.status.duration_ms as f64 / 1000.0,
        )
        .unwrap();
        title
    }
}
