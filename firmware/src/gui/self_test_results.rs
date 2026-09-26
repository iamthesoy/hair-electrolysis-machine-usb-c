use core::fmt::Write;

use arrayvec::ArrayString;
use embedded_graphics::{
    Drawable,
    mono_font::{MonoTextStyle, ascii::FONT_5X7},
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point, Size},
    primitives::Rectangle,
};
use embedded_text::{
    TextBox,
    alignment::HorizontalAlignment,
    style::{HeightMode, TextBoxStyleBuilder},
};

const THRESHOLD_CHARGE_PUMP_VOLTS: f64 = 12.0;
const THRESHOLD_MAX_DELTA_CURRENT_MA: f64 = 0.1;
const THRESHOLD_MAX_LEAKAGE_MA: f64 = 0.03;

#[derive(Copy, Clone)]
pub struct SweepPoint {
    pub requested_current_ma: f64,
    pub measured_current_ma: f64,
    // calculated_resistance: f64,
    // charge_pump_voltage: f64,
}

impl SweepPoint {
    pub fn empty() -> Self {
        Self {
            requested_current_ma: 0.0,
            measured_current_ma: 0.0,
        }
    }

    pub fn new(requested_current_ma: f64, measured_current_ma: f64) -> Self {
        Self {
            requested_current_ma,
            measured_current_ma,
        }
    }
}

// todo it would be nice if this graphed things
pub struct SelfTestResults {
    measured_charge_pump_volts: f64,
    max_delta_current_ma: f64,
    leakage_current_ma: f64,
}

impl SelfTestResults {
    pub fn new(measured_charge_pump_volts: f64, sweep: [SweepPoint; 32]) -> Self {
        let max_delta_current_ma = sweep
            .iter()
            .map(|point| {
                let delta = point.requested_current_ma - point.measured_current_ma;

                delta.abs()
            })
            .fold(f64::NEG_INFINITY, f64::max);

        Self {
            measured_charge_pump_volts,
            max_delta_current_ma,
            leakage_current_ma: sweep[0].measured_current_ma,
        }
    }

    pub fn render<E: core::fmt::Debug, D: DrawTarget<Color = BinaryColor, Error = E>>(
        &self,
        display: &mut D,
    ) {
        let mut s = ArrayString::<128>::new();
        write!(
            &mut s,
            "[{}] Max V   {:02.2} > {:02.2}\n\
             [{}] Max delta {:.2} < {:.2}\n\
             [{}] Leakage   {:.2} < {:.2}",
            // unloaded V
            if self.measured_charge_pump_volts > THRESHOLD_CHARGE_PUMP_VOLTS {
                "P"
            } else {
                "F"
            },
            self.measured_charge_pump_volts,
            THRESHOLD_CHARGE_PUMP_VOLTS,
            // worst delta
            if self.max_delta_current_ma < THRESHOLD_MAX_DELTA_CURRENT_MA {
                "P"
            } else {
                "F"
            },
            self.max_delta_current_ma,
            THRESHOLD_MAX_DELTA_CURRENT_MA,
            // leakage current
            if self.leakage_current_ma < THRESHOLD_MAX_LEAKAGE_MA {
                "P"
            } else {
                "F"
            },
            self.leakage_current_ma,
            THRESHOLD_MAX_LEAKAGE_MA,
        )
        .unwrap();

        let character_style = MonoTextStyle::new(&FONT_5X7, BinaryColor::On);
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
    }
}
