pub mod mime;

#[cfg(all(
    unix,
    not(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "android",
        target_os = "emscripten",
        target_os = "redox"
    ))
))]
#[path = "platform/linux.rs"]
mod platform;

#[cfg(target_os = "windows")]
#[path = "platform/windows.rs"]
mod platform;

#[cfg(target_os = "macos")]
#[path = "platform/macos.rs"]
mod platform;

#[cfg(target_os = "ios")]
#[path = "platform/ios.rs"]
mod platform;

#[cfg(target_os = "android")]
#[path = "platform/android.rs"]
mod platform;

#[cfg(not(any(
    all(
        unix,
        not(any(
            target_os = "macos",
            target_os = "ios",
            target_os = "android",
            target_os = "emscripten",
            target_os = "redox"
        ))
    ),
    target_os = "windows",
    target_os = "macos",
    target_os = "ios",
    target_os = "android"
)))]
#[path = "platform/dummy.rs"]
mod platform;

use mime::{ClipboardLoadData, ClipboardStoreData};
use raw_window_handle::HasDisplayHandle;
use std::error::Error;

pub struct Clipboard<C> {
    raw: C,
}

impl Clipboard<platform::Clipboard> {
    /// Safety: the display handle must be valid for the lifetime of `Clipboard`
    pub unsafe fn connect<W: HasDisplayHandle>(
        window: &W,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Clipboard {
            raw: platform::connect(window)?,
        })
    }

    pub fn read(&self) -> Result<String, Box<dyn Error>> {
        self.raw.read()
    }

    pub fn write(&mut self, contents: String) -> Result<(), Box<dyn Error>> {
        self.raw.write(contents)
    }
}

impl<C: ClipboardProvider> Clipboard<C> {
    pub fn read_primary(&self) -> Option<Result<String, Box<dyn Error>>> {
        self.raw.read_primary()
    }

    pub fn write_primary(
        &mut self,
        contents: String,
    ) -> Option<Result<(), Box<dyn Error>>> {
        self.raw.write_primary(contents)
    }
}

pub trait ClipboardProvider {
    fn read(&self) -> Result<String, Box<dyn Error>>;

    fn write(&mut self, contents: String) -> Result<(), Box<dyn Error>>;

    fn read_primary(&self) -> Option<Result<String, Box<dyn Error>>> {
        None
    }

    fn write_primary(
        &mut self,
        _contents: String,
    ) -> Option<Result<(), Box<dyn Error>>> {
        None
    }

    fn read_data<T: 'static>(&self) -> Option<Result<T, Box<dyn Error>>>
    where
        ClipboardLoadData<T>: platform::InnerAllowedMimeTypes,
    {
        None
    }

    fn write_data<T: Send + Sync + 'static>(
        &mut self,
        _contents: ClipboardStoreData<T>,
    ) -> Option<Result<(), Box<dyn Error>>>
    where
        ClipboardStoreData<T>: platform::InnerAsMimeTypes,
    {
        None
    }

    fn read_primary_data<T: 'static>(&self) -> Option<Result<T, Box<dyn Error>>>
    where
        ClipboardLoadData<T>: platform::InnerAllowedMimeTypes,
    {
        None
    }

    fn write_primary_data<T: Send + Sync + 'static>(
        &mut self,
        _contents: ClipboardStoreData<T>,
    ) -> Option<Result<(), Box<dyn Error>>>
    where
        ClipboardStoreData<T>: platform::InnerAsMimeTypes,
    {
        None
    }
}
