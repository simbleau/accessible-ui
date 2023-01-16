use cssugar::prelude::*;
use stylist::macros::vendor::once_cell::sync::Lazy;

#[derive(Clone, Debug, PartialEq)]
pub struct AuiSpec {
    /// Foreground color
    pub color: Color,
    /// Background color
    pub background_color: Color,
    /// Link color
    pub link: Color,
    /// Link hover color
    pub link_hover: Color,
    /// Header color
    pub header_color: Color,
}

static DEFAULT_AUI: Lazy<AuiSpec> = Lazy::new(|| AuiSpec {
    color: Color::from_rgb(0x21, 0x25, 0x29),
    background_color: Color::from_rgb(0xf8, 0xf9, 0xfa),
    link: Color::from_rgb(0x00, 0x33, 0xdd),
    link_hover: Color::from_rgb(0x00, 0x33, 0xdd).lighten(0.3),
    header_color: Color::from_rgb(0x00, 0x33, 0xdd),
});

impl Default for AuiSpec {
    fn default() -> Self {
        DEFAULT_AUI.to_owned()
    }
}
