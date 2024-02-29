use crate::ClipboardProvider;

use raw_window_handle::HasDisplayHandle;

struct Clipboard;

pub fn connect<W: HasDisplayHandle>(
    _window: &W,
) -> Result<Clipboard, Box<dyn std::error::Error>> {
    Ok(Clipboard)
}

impl ClipboardProvider for Clipboard {
    fn read(&self) -> Result<String, Box<dyn std::error::Error>> {
        Err(Box::new(Error::Unimplemented))
    }

    fn write(
        &mut self,
        _contents: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Err(Box::new(Error::Unimplemented))
    }
}

#[derive(Debug, Clone, Copy, thiserror::Error)]
enum Error {
    #[error("unimplemented")]
    Unimplemented,
}
