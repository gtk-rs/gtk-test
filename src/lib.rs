#![warn(missing_docs)]

//! Crate to test UI interactions with [gtk-rs] crates.
//!
//! [gtk-rs]: https://gtk-rs.org
//!
//! Small example:
//!
//! ```
//! extern crate gtk;
//! #[macro_use]
//! extern crate gtk_test;
//!
//! use gtk::{prelude::ButtonExt, prelude::ContainerExt, prelude::GtkWindowExt, prelude::LabelExt, prelude::WidgetExt};
//!
//! # fn main() {
//! gtk::init().expect("GTK init failed");
//!
//! let win = gtk::Window::new(gtk::WindowType::Toplevel);
//! let but = gtk::Button::new();
//!
//! but.set_label(""); // Otherwise, assert_label! call will fail.
//! but.connect_clicked(|b| {
//!     b.set_label("clicked!");
//! });
//!
//! win.add(&but);
//! win.show_all();
//! win.activate_focus(); // Very important, otherwise tests will fail on OSX!
//!
//! assert_label!(but, "");
//! gtk_test::click(&but);
//! gtk_test::wait(1000); // To be sure that GTK has updated the label's text.
//! assert_label!(but, "clicked!");
//! # }
//! ```

extern crate enigo;
extern crate gdk;
extern crate glib;
extern crate gtk;

mod macros;

mod functions;
mod observer;

pub use functions::*;
pub use observer::Observer;
