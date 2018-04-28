extern crate gtk;
#[macro_use]
extern crate gtk_test;

use gtk::{
    Button,
    ButtonExt,
    ContainerExt,
    GtkWindowExt,
    Inhibit,
    Label,
    LabelExt,
    Orientation,
    WidgetExt,
    Window,
    WindowType,
};

pub fn init_ui() -> (Window, Label, Button) {
    gtk::init().unwrap();

    let window = Window::new(WindowType::Toplevel);
    let b = gtk::Box::new(Orientation::Vertical, 0);
    let label = Label::new("Test");
    let but = Button::new();

    let l = label.clone();
    but.connect_clicked(move |_| {
        l.set_text("Clicked");
    });

    b.add(&label);
    b.add(&but);
    window.add(&b);
    window.show_all();
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    (window, label, but)
}

fn main() {
    let (w, l, b) = init_ui();

    assert_text!(l, "Test");
    w.activate_focus();
    gtk_test::click(&b);
    gtk_test::wait(1000); // to be sure that GTK has updated the label's text
    assert_text!(l, "Clicked");
}
