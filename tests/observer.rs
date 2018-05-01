extern crate gtk;
#[macro_use]
extern crate gtk_test;

use gtk::{
    GtkWindowExt,
    Window,
    WindowType,
};

fn main() {
    gtk::init().expect("initialization failed");

    let window = Window::new(WindowType::Toplevel);

    let observer = observer_new!(window, connect_activate_focus, |_|);
    window.emit_activate_focus();
    observer.wait();
    let observer = observer_new!(window, connect_activate_focus, |w| {
        w.set_title("Title!");
    });
    window.emit_activate_focus();
    observer.wait();
    assert_title!(window, "Title!");
}
