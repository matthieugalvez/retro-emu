use sdl2::rwops::RWops;
use sdl2::ttf::{Font, Sdl2TtfContext};

pub const FONT: &[u8] = include_bytes!("../../assets/AdwaitaMono-Bold.ttf");

pub fn load_font<'ttf>(
    ttf_context: &'ttf Sdl2TtfContext,
    size: u16,
) -> Result<Font<'ttf, 'static>, String> {
    let rwops = RWops::from_bytes(FONT)?;
    ttf_context.load_font_from_rwops(rwops, size)
}
