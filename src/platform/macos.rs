use crate::ClipboardProvider;

pub(crate) use clipboard_macos::Clipboard;
use raw_window_handle::HasDisplayHandle;
use std::error::Error;

pub fn connect<W: HasDisplayHandle>(
    _window: &W,
) -> Result<Clipboard, Box<dyn Error>> {
    Clipboard::new()
}

impl ClipboardProvider for Clipboard {
    fn read(&self) -> Result<String, Box<dyn Error>> {
        self.read()
    }

    fn write(&mut self, contents: String) -> Result<(), Box<dyn Error>> {
        self.write(contents)
    }
}
