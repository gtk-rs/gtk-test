/// To check if the widget's label matches the given string.
///
/// Example:
///
/// ```
/// extern crate gtk;
/// #[macro_use]
/// extern crate gtk_test;
///
/// use gtk::{Button, ButtonExt, LabelExt};
///
/// # fn main() {
/// gtk::init().expect("GTK init failed");
/// let but = Button::new();
/// but.set_label("text");
/// assert_label!(but, "text");
/// # }
/// ```
#[macro_export]
macro_rules! assert_label {
    ($widget:expr, $string:expr) => {
        assert_eq!($widget.get_label().expect("get label"), $string.to_string());
    };
}

/// To check if the widget's text matches the given string.
///
/// Example:
///
/// ```
/// extern crate gtk;
/// #[macro_use]
/// extern crate gtk_test;
///
/// use gtk::{Label, LabelExt};
///
/// # fn main() {
/// gtk::init().expect("GTK init failed");
/// let label = Label::new("I'm a label!");
/// assert_text!(label, "I'm a label!");
/// # }
/// ```
#[macro_export]
macro_rules! assert_text {
    ($widget:expr, $string:expr) => {
        assert_eq!($widget.get_text().expect("get text"), $string.to_string());
    };
}