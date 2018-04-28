/*
 * Copyright (c) 2017-2018 Boucher, Antoni <bouanto@zoho.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

extern crate gtk;
#[macro_use]
extern crate gtk_test;

use gtk::{Button, ButtonExt, ContainerExt, Inhibit, Label, LabelExt, Orientation, WidgetExt, Window, WindowType};

pub fn init_ui() -> (Window, Label, Button) {
    gtk::init().unwrap();

    let window = Window::new(WindowType::Toplevel);
    let b = gtk::Box::new(Orientation::Vertical, 0);
    let label = Label::new("Test");
    let but = Button::new();

    let l = label.clone();
    but.connect_clicked(move |_| {
        println!("clicked");
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

#[cfg(test)]
mod tests {
    use ::init_ui;

    use gtk_test;
    use gtk::{GtkWindowExt, LabelExt};

    #[test]
    fn simple_test() {
        let (w, l, b) = init_ui();

        assert_text!(l, "Test");
        w.activate_focus();
        gtk_test::click(&b);
        gtk_test::wait(1000);
        assert_text!(l, "Clicked");
    }
}
