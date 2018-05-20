//! Platform-independent terminal interface
//!
//! Two distinct interfaces to operating system terminal devices are provided,
//! each abstracting over the differences between Unix terminals and Windows console.
//!
//! The [`Terminal`] interface treats the terminal as a line-by-line
//! output device. Methods exist to add color and style attributes to text,
//! and to make relative movements of the cursor.
//!
//! The [`Screen`] interface treats the entire terminal window as a drawable
//! buffer. Methods exist to set the cursor position and to write text with
//! color and style attributes.
//!
//! ## Concurrency
//!
//! Each interface uses internal locking mechanisms to allow sharing of the
//! terminal interface between threads while maintaining coherence of read/write
//! operations.
//!
//! See the documentation for [`Terminal`] and [`Screen`] for further details.
//!
//! [`Screen`]: struct.Screen.html
//! [`Terminal`]: struct.Terminal.html
//! [`refresh`]: struct.Screen.html#method.refresh

#![deny(missing_docs)]

#[macro_use] extern crate bitflags;
extern crate libc;
extern crate smallstr;
extern crate unicode_normalization;
extern crate unicode_width;

#[cfg(unix)] extern crate nix;
#[cfg(unix)] extern crate terminfo;

#[cfg(windows)] extern crate winapi;

pub use screen::{Screen, ScreenReadGuard, ScreenWriteGuard};
pub use sequence::{FindResult, SequenceMap};
pub use signal::{Signal, SignalSet};
pub use terminal::{
    Color, Cursor, CursorMode, Size, Style,
    Event, Key, MouseEvent, MouseInput, MouseButton, ModifierState,
    PrepareConfig, PrepareState,
    Terminal, TerminalReadGuard, TerminalWriteGuard,
};

#[macro_use] mod buffer;
mod priv_util;
pub mod screen;
pub mod sequence;
pub mod signal;
pub mod terminal;
pub mod util;

#[cfg(unix)]
#[path = "unix/mod.rs"]
mod sys;

#[cfg(windows)]
#[path = "windows/mod.rs"]
mod sys;

#[cfg(unix)]
pub use sys::ext as unix;

#[cfg(windows)]
pub use sys::ext as windows;

#[cfg(test)]
mod test {
    use screen::Screen;
    use terminal::Terminal;

    fn assert_has_traits<T: 'static + Send + Sync>() {}

    #[test]
    fn test_traits() {
        assert_has_traits::<Terminal>();
        assert_has_traits::<Screen>();
    }
}
