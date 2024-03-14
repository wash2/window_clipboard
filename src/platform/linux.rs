use crate::{
    mime::{ClipboardLoadData, ClipboardStoreData},
    ClipboardProvider,
};

use raw_window_handle::{HasDisplayHandle, RawDisplayHandle};
use std::error::Error;

pub use clipboard_wayland as wayland;
pub use clipboard_x11 as x11;

pub enum Clipboard {
    Wayland(wayland::Clipboard),
    X11(x11::Clipboard),
}

impl ClipboardProvider for Clipboard {
    fn read(&self) -> Result<String, Box<dyn Error>> {
        match self {
            Clipboard::Wayland(c) => c.read(),
            Clipboard::X11(c) => c.read().map_err(Box::from),
        }
    }

    fn write(&mut self, contents: String) -> Result<(), Box<dyn Error>> {
        match self {
            Clipboard::Wayland(c) => c.write(contents),
            Clipboard::X11(c) => c.write(contents).map_err(Box::from),
        }
    }

    fn read_primary(&self) -> Option<Result<String, Box<dyn Error>>> {
        match self {
            Clipboard::Wayland(c) => Some(c.read_primary()),
            Clipboard::X11(c) => Some(c.read_primary().map_err(Box::from)),
        }
    }

    fn write_primary(
        &mut self,
        contents: String,
    ) -> Option<Result<(), Box<dyn Error>>> {
        match self {
            Clipboard::Wayland(c) => Some(c.write_primary(contents)),
            Clipboard::X11(c) => {
                Some(c.write_primary(contents).map_err(Box::from))
            }
        }
    }

    fn read_data<T: 'static>(&self) -> Option<Result<T, Box<dyn Error>>>
    where
        T: mime::AllowedMimeTypes,
    {
        match self {
            Clipboard::Wayland(c) => {
                let ret = c.read_data::<ClipboardLoadData<T>>();
                Some(ret.map(|ret| ret.0))
            }
            Clipboard::X11(_) => None,
        }
    }

    fn write_data<T: Send + Sync + 'static>(
        &mut self,
        contents: ClipboardStoreData<T>,
    ) -> Option<Result<(), Box<dyn Error>>>
    where
        T: mime::AsMimeTypes,
    {
        match self {
            Clipboard::Wayland(c) => {
                Some(c.write_data::<ClipboardStoreData<T>>(contents))
            }
            Clipboard::X11(_) => None,
        }
    }

    fn read_primary_data<T: 'static>(&self) -> Option<Result<T, Box<dyn Error>>>
    where
        T: mime::AllowedMimeTypes,
    {
        match self {
            Clipboard::Wayland(c) => {
                let ret = c.read_primary_data::<ClipboardLoadData<T>>();
                Some(ret.map(|ret| ret.0))
            }
            Clipboard::X11(_) => None,
        }
    }

    fn read_primary_raw(
        &self,
        allowed: Vec<String>,
    ) -> Option<Result<(Vec<u8>, String), Box<dyn Error>>> {
        match self {
            Clipboard::Wayland(c) => Some(c.read_primary_raw(allowed)),
            Clipboard::X11(_) => None,
        }
    }

    fn read_raw(
        &self,
        allowed: Vec<String>,
    ) -> Option<Result<(Vec<u8>, String), Box<dyn Error>>> {
        match self {
            Clipboard::Wayland(c) => Some(c.read_raw(allowed)),
            Clipboard::X11(_) => None,
        }
    }

    fn write_primary_data<T: Send + Sync + 'static>(
        &mut self,
        contents: ClipboardStoreData<T>,
    ) -> Option<Result<(), Box<dyn Error>>>
    where
        T: mime::AsMimeTypes,
    {
        match self {
            Clipboard::Wayland(c) => {
                Some(c.write_primary_data::<ClipboardStoreData<T>>(contents))
            }
            Clipboard::X11(_) => None,
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
