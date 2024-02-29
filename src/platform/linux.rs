use crate::ClipboardProvider;

use raw_window_handle::{HasDisplayHandle, RawDisplayHandle};
use std::error::Error;

pub use clipboard_wayland as wayland;
pub use clipboard_x11 as x11;

pub enum Clipboard {
    Wayland(wayland::Clipboard),
    X11(x11::Clipboard),
}

impl ClipboardProvider for Clipboard {
    fn read_text(&self) -> Result<String, Box<dyn Error>> {
        match self {
            Clipboard::Wayland(c) => c.read_text(),
            Clipboard::X11(c) => c.read().map_err(Box::from),
        }
    }

    fn write_text(&mut self, contents: String) -> Result<(), Box<dyn Error>> {
        match self {
            Clipboard::Wayland(c) => c.write_text(contents),
            Clipboard::X11(c) => c.write(contents).map_err(Box::from),
        }
    }

    fn read_primary_text(&self) -> Option<Result<String, Box<dyn Error>>> {
        match self {
            Clipboard::Wayland(c) => Some(c.read_primary_text()),
            Clipboard::X11(c) => Some(c.read_primary().map_err(Box::from)),
        }
    }

    fn write_primary_text(
        &mut self,
        contents: String,
    ) -> Option<Result<(), Box<dyn Error>>> {
        match self {
            Clipboard::Wayland(c) => Some(c.write_primary_text(contents)),
            Clipboard::X11(c) => {
                Some(c.write_primary(contents).map_err(Box::from))
            }
        }
    }
}

pub unsafe fn connect<W: HasDisplayHandle>(
    window: &W,
) -> Result<Clipboard, Box<dyn Error>> {
    let clipboard = match window.display_handle()?.as_raw() {
        RawDisplayHandle::Wayland(handle) => Clipboard::Wayland(
            wayland::Clipboard::connect(handle.display.as_ptr()),
        ) as _,
        _ => Clipboard::X11(x11::Clipboard::connect()?) as _,
    };

    Ok(clipboard)
}
