use std::{
    borrow::Cow,
    ffi::c_void,
    sync::{mpsc::SendError, Arc},
};

use bitflags::bitflags;

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
pub mod platform;

bitflags! {
    // Attributes can be applied to flags types
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct DndAction: u32 {
        const Copy = 0b00000001;
        const Move = 0b00000010;
        const Ask = 0b00000100;
    }
}

#[derive(Debug)]
pub enum DndEvent<T> {
    /// Dnd Offer event with the corresponding destination rectangle ID.
    Offer(Option<u128>, OfferEvent<T>),
    /// Dnd Source event.
    Source(SourceEvent),
}

#[derive(Debug)]
pub enum SourceEvent {
    /// DnD operation ended.
    Finished,
    /// DnD Cancelled.
    Cancelled,
    /// DnD action chosen by the compositor.
    Action(DndAction),
    /// Mime accepted by destination.
    /// If [`None`], no mime types are accepted.
    Mime(Option<String>),
    /// DnD Dropped. The operation is still ongoing until receiving a
    /// [`SourceEvent::Finished`] event.
    Dropped,
}

#[derive(Debug)]
pub enum OfferEvent<T> {
    Enter {
        x: f64,
        y: f64,
        mime_types: Vec<String>,
        surface: T,
    },
    Motion {
        x: f64,
        y: f64,
    },
    /// The offer is no longer on a DnD destination.
    LeaveDestination,
    /// The offer has left the surface.
    Leave,
    /// An offer was dropped
    Drop,
    /// If the selected action is ASK, the user must be presented with a
    /// choice. [`Clipboard::set_action`] should then be called before data
    /// can be requested and th DnD operation can be finished.
    SelectedAction(DndAction),
    Data {
        data: Vec<u8>,
        mime_type: String,
    },
}

/// A rectangle with a logical location and size relative to a [`DndSurface`]
#[derive(Debug, Default, Clone)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

pub trait Sender<T> {
    /// Send an event in the channel
    fn send(&self, t: DndEvent<T>) -> Result<(), SendError<DndEvent<T>>>;
}

pub trait RawSurface {
    /// # Safety
    ///
    /// returned pointer must be a valid pointer to the underlying surface, and it must
    /// remain valid for as long as `RawSurface` object is alive.
    unsafe fn get_ptr(&mut self) -> *mut c_void;
}

/// A rectangle with a logical location and size relative to a [`DndSurface`]
#[derive(Debug, Clone)]
pub struct DndDestinationRectangle {
    /// A unique ID
    pub id: u128,
    /// The rectangle representing this destination.
    pub rectangle: Rectangle,
    /// Accepted mime types in this rectangle
    pub mime_types: Vec<Cow<'static, str>>,
    /// Accepted actions in this rectangle
    pub actions: DndAction,
    /// Prefered action in this rectangle
    pub preferred: DndAction,
}

#[derive(Clone)]
pub struct DndSurface(pub Arc<Box<dyn RawSurface + 'static + Send + Sync>>);

#[derive(Clone)]
pub struct DataWrapper<T>(pub T);
