use std::str::FromStr;

use catppuccin::{self, Flavor};
use clap::ValueEnum;

/// Convert a catppuccin color into an ANSI color (type casting)
pub const fn to_ansi(color: &catppuccin::Color) -> ansi_term::Colour {
    ansi_term::Colour::RGB(color.rgb.r, color.rgb.g, color.rgb.b)
}

/// Color variant specified by the user and parsed at runtime.
#[derive(ValueEnum, Clone, Debug, Default)]
pub enum ColorVariant {
    #[default]
    Mocha,
    Frappe,
    Latte,
    Macchiato,
}

impl FromStr for ColorVariant {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let valid_flavors: Vec<&str> = vec!["mocha", "latte", "frappe", "macchiato"];
        let mut variant: Self = Self::Mocha;
        if valid_flavors.contains(&s) {
            variant = match s {
                "mocha" => Self::Mocha,
                "latte" => Self::Latte,
                "frappe" => Self::Frappe,
                "macchiato" => Self::Macchiato,
                _ => Self::Mocha,
            };
        }
        Ok(variant)
    }
}

impl From<ColorVariant> for Flavor {
    fn from(value: ColorVariant) -> Self {
        match value {
            ColorVariant::Mocha => catppuccin::PALETTE.mocha,
            ColorVariant::Frappe => catppuccin::PALETTE.frappe,
            ColorVariant::Macchiato => catppuccin::PALETTE.macchiato,
            ColorVariant::Latte => catppuccin::PALETTE.latte,
        }
    }
}
