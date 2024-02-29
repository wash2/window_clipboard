use crate::{
    mime::{ClipboardLoadData, ClipboardStoreData},
    ClipboardProvider,
};

use raw_window_handle::{HasDisplayHandle, RawDisplayHandle};
use std::error::Error;
use wayland::MimeType;

pub use clipboard_wayland as wayland;
pub use clipboard_x11 as x11;
pub use wayland::{
    AllowedMimeTypes as InnerAllowedMimeTypes, AsMimeTypes as InnerAsMimeTypes,
};

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

    fn read<T: 'static>(&self) -> Option<Result<T, Box<dyn Error>>>
    where
        ClipboardLoadData<T>: InnerAllowedMimeTypes,
    {
        match self {
            Clipboard::Wayland(c) => {
                let ret = c.read::<ClipboardLoadData<T>>();
                Some(ret.map(|ret| ret.0))
            }
            Clipboard::X11(_) => None,
        }
    }

    fn write<T: Send + Sync + 'static>(
        &mut self,
        contents: ClipboardStoreData<T>,
    ) -> Option<Result<(), Box<dyn Error>>>
    where
        ClipboardStoreData<T>: InnerAsMimeTypes,
    {
        match self {
            Clipboard::Wayland(c) => {
                Some(c.write::<ClipboardStoreData<T>>(contents))
            }
            Clipboard::X11(_) => None,
        }
    }

    fn read_primary<T: 'static>(&self) -> Option<Result<T, Box<dyn Error>>>
    where
        ClipboardLoadData<T>: InnerAllowedMimeTypes,
    {
        match self {
            Clipboard::Wayland(c) => {
                let ret = c.read_primary::<ClipboardLoadData<T>>();
                Some(ret.map(|ret| ret.0))
            }
            Clipboard::X11(_) => None,
        }
    }

    fn write_primary<T: Send + Sync + 'static>(
        &mut self,
        contents: ClipboardStoreData<T>,
    ) -> Option<Result<(), Box<dyn Error>>>
    where
        ClipboardStoreData<T>: InnerAsMimeTypes,
    {
        match self {
            Clipboard::Wayland(c) => {
                Some(c.write_primary::<ClipboardStoreData<T>>(contents))
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

impl<T: crate::mime::AsMimeTypes> InnerAsMimeTypes for ClipboardLoadData<T> {
    fn available(&self) -> std::borrow::Cow<'static, [MimeType]> {
        self.0
            .available()
            .into_iter()
            .map(|m| MimeType::Other(m.clone().into()))
            .collect()
    }

    fn as_bytes(
        &self,
        mime_type: &MimeType,
    ) -> Option<std::borrow::Cow<'static, [u8]>> {
        self.0.as_bytes(mime_type.as_ref())
    }
}

impl<T: crate::mime::AllowedMimeTypes> InnerAllowedMimeTypes
    for ClipboardLoadData<T>
where
    ClipboardLoadData<T>: TryFrom<(Vec<u8>, MimeType)>,
{
    // TODO select text variants if string matches...
    fn allowed() -> std::borrow::Cow<'static, [wayland::MimeType]> {
        T::allowed()
            .into_iter()
            .map(|s| MimeType::Other(s.clone().into()))
            .collect()
    }
}

impl<T> TryFrom<(Vec<u8>, MimeType)> for ClipboardLoadData<T>
where
    T: for<'b> TryFrom<(Vec<u8>, String)>,
    T: 'static,
{
    type Error = crate::mime::Error;

    fn try_from(
        (value, mime): (Vec<u8>, MimeType),
    ) -> Result<Self, Self::Error> {
        let mime = mime.to_string();
        Ok(ClipboardLoadData(
            T::try_from((value, mime)).map_err(|_| crate::mime::Error)?,
        ))
    }
}
