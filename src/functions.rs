use enigo::{self, Enigo, KeyboardControllable, MouseButton, MouseControllable};
use gtk::gdk::keys::constants as key;
use gtk::gdk::keys::Key;
use gtk::glib::{Cast, Continue, IsA, Object, StaticType};
use gtk::prelude::{BinExt, ButtonExt, ContainerExt, EditableExt, ToolButtonExt, WidgetExt};
use gtk::{Bin, Button, Container, Entry, Inhibit, ToolButton, Widget, Window};

use crate::observer_new;

/// Simulate a click on a widget.
///
/// ## Warning!
///
/// Please note that the click will "fail" if the window isn't on top of all other windows (this
/// is a common issue on OSX). Don't forget to bring the button's window on top by using:
///
/// ```ignore
/// window.activate_focus();
/// ```
///
/// Example:
///
/// ```
/// extern crate gtk;
/// #[macro_use]
/// extern crate gtk_test;
///
/// use gtk::{Button, prelude::ButtonExt};
///
/// # fn main() {
/// gtk::init().expect("GTK init failed");
/// let but = Button::new();
/// but.connect_clicked(|_| {
///     println!("clicked");
/// });
/// gtk_test::click(&but);
/// # }
/// ```
pub fn click<W: Clone + IsA<Object> + IsA<Widget> + WidgetExt + IsA<W>>(widget: &W) {
    wait_for_draw(widget, || {
        let observer = if let Ok(tool_button) = widget.clone().dynamic_cast::<ToolButton>() {
            observer_new!(tool_button, connect_clicked, |_|)
        } else if let Ok(tool_button) = widget.clone().dynamic_cast::<Button>() {
            observer_new!(tool_button, connect_clicked, |_|)
        } else {
            observer_new!(widget, connect_button_release_event, |_, _| {
                Inhibit(false)
            })
        };
        let allocation = widget.allocation();
        mouse_move(widget, allocation.width / 2, allocation.height / 2);
        let mut enigo = Enigo::new();
        enigo.mouse_click(MouseButton::Left);
        observer.wait();
    });
}

/// Simulate a double-click on a widget.
///
/// ## Warning!
///
/// Please note that the double-click will "fail" if the window isn't on top of all other windows
/// (this is a common issue on OSX). Don't forget to bring the button's window on top by using:
///
/// ```ignore
/// window.activate_focus();
/// ```
///
/// Example:
///
/// ```
/// extern crate gtk;
/// #[macro_use]
/// extern crate gtk_test;
///
/// use gtk::{FileChooserAction, FileChooserWidget, prelude::FileChooserExt};
///
/// # fn main() {
/// gtk::init().expect("GTK init failed");
/// let fcw = FileChooserWidget::new(FileChooserAction::Open);
/// fcw.connect_file_activated(|_| {
///     println!("double clicked");
/// });
/// gtk_test::double_click(&fcw);
/// # }
/// ```
pub fn double_click<W: Clone + IsA<Object> + IsA<Widget> + WidgetExt>(widget: &W) {
    wait_for_draw(widget, || {
        let observer = observer_new!(widget, connect_button_release_event, |_, _| {
            Inhibit(false)
        });
        let allocation = widget.allocation();
        mouse_move(widget, allocation.width / 2, allocation.height / 2);
        let mut enigo = Enigo::new();
        enigo.mouse_click(MouseButton::Left);
        run_loop();
        enigo.mouse_click(MouseButton::Left);
        observer.wait();
    });
}

/// Move the mouse relative to the widget position.
///
/// Example:
///
/// ```
/// extern crate gtk;
/// #[macro_use]
/// extern crate gtk_test;
///
/// use gtk::Button;
///
/// # fn main() {
/// gtk::init().expect("GTK init failed");
/// let but = Button::new();
/// gtk_test::mouse_move(&but, 0, 0); // the mouse will be on the top-left corner of the button
/// # }
/// ```
pub fn mouse_move<W: IsA<Object> + IsA<Widget> + WidgetExt>(widget: &W, x: i32, y: i32) {
    wait_for_draw(widget, || {
        let toplevel_window = widget.toplevel().and_then(|toplevel| toplevel.window());
        if let (Some(toplevel), Some(toplevel_window)) = (widget.toplevel(), toplevel_window) {
            let (_, window_x, window_y) = toplevel_window.origin();
            if let Some((x, y)) = widget.translate_coordinates(&toplevel, x, y) {
                let x = window_x + x;
                let y = window_y + y;
                let mut enigo = Enigo::new();
                enigo.mouse_move_to(x, y);
                run_loop();
            }
        }
    });
}

/// Send a mouse press event to the given widget.
///
/// ## Warning!
///
/// Please note that the mouse-press event will "fail" if the window isn't on top of all other
/// windows (this is a common issue on OSX). Don't forget to bring the button's window on top
/// by using:
///
/// ```ignore
/// window.activate_focus();
/// ```
///
/// Example:
///
/// ```
/// extern crate gtk;
/// #[macro_use]
/// extern crate gtk_test;
///
/// use gtk::{Entry, prelude::EntryExt};
///
/// # fn main() {
/// gtk::init().expect("GTK init failed");
/// let entry = Entry::new();
/// entry.connect_icon_press(|_, _, _| {
///     println!("pressed");
/// });
/// gtk_test::mouse_press(&entry);
/// # }
/// ```
pub fn mouse_press<W: IsA<Object> + IsA<Widget> + WidgetExt>(widget: &W) {
    wait_for_draw(widget, || {
        let allocation = widget.allocation();
        mouse_move(widget, allocation.width / 2, allocation.height / 2);
        let mut enigo = Enigo::new();
        enigo.mouse_down(MouseButton::Left);
        run_loop();
    });
}

/// Send a mouse release event to the given widget.
///
/// ## Warning!
///
/// Please note that the mouse-release event will "fail" if the window isn't on top of all other
/// windows (this is a common issue on OSX). Don't forget to bring the button's window on top
/// by using:
///
/// ```ignore
/// window.activate_focus();
/// ```
///
/// Example:
///
/// ```
/// extern crate gtk;
/// #[macro_use]
/// extern crate gtk_test;
///
/// use gtk::{Entry, prelude::EntryExt};
///
/// # fn main() {
/// gtk::init().expect("GTK init failed");
/// let entry = Entry::new();
/// entry.connect_icon_release(|_, _, _| {
///     println!("released");
/// });
/// gtk_test::mouse_release(&entry);
/// # }
/// ```
pub fn mouse_release<W: IsA<Object> + IsA<Widget> + WidgetExt>(widget: &W) {
    wait_for_draw(widget, || {
        let allocation = widget.allocation();
        mouse_move(widget, allocation.width / 2, allocation.height / 2);
        let mut enigo = Enigo::new();
        enigo.mouse_up(MouseButton::Left);
        run_loop();
    });
}

/// Send a key event to the given widget.
///
/// ## Warning!
///
/// Please note that the enter-key event will "fail" if the window isn't on top of all other
/// windows (this is a common issue on OSX). Don't forget to bring the button's window on top
/// by using:
///
/// ```ignore
/// window.activate_focus();
/// ```
///
/// Example:
///
/// ```
/// #[macro_use]
/// extern crate gtk_test;
///
/// use gtk_test::gtk::{Entry, prelude::EntryExt};
///
/// # fn main() {
/// gtk_test::gtk::init().expect("GTK init failed");
/// let entry = Entry::new();
/// entry.connect_preedit_changed(|_, _| {
///     println!("key entered");
/// });
/// gtk_test::enter_key(&entry, gtk_test::gdk::keys::constants::Agrave);
/// # }
/// ```
pub fn enter_key<W: Clone + IsA<Object> + IsA<Widget> + WidgetExt>(widget: &W, key: Key) {
    wait_for_draw(widget, || {
        let observer = observer_new!(widget, connect_key_release_event, |_, _| { Inhibit(false) });
        focus(widget);
        let mut enigo = Enigo::new();
        enigo.key_click(gdk_key_to_enigo_key(key));
        observer.wait();
    });
}

/// Send keys event to the given widget.
///
/// ## Warning!
///
/// Please note that the enter-key event will "fail" if the window isn't on top of all other
/// windows (this is a common issue on OSX). Don't forget to bring the button's window on top
/// by using:
///
/// ```ignore
/// window.activate_focus();
/// ```
///
/// Example:
///
/// ```
/// extern crate gtk;
/// #[macro_use]
/// extern crate gtk_test;
///
/// use gtk::{Entry, prelude::EntryExt};
///
/// # fn main() {
/// gtk::init().expect("GTK init failed");
/// let entry = Entry::new();
/// entry.connect_preedit_changed(|_, _| {
///     println!("key entered");
/// });
/// gtk_test::enter_keys(&entry, "A lot of keys!");
/// # }
/// ```
pub fn enter_keys<W: Clone + IsA<Object> + IsA<Widget> + WidgetExt>(widget: &W, text: &str) {
    wait_for_draw(widget, || {
        focus(widget);
        let mut enigo = Enigo::new();
        for char in text.chars() {
            let observer =
                observer_new!(widget, connect_key_release_event, |_, _| { Inhibit(false) });
            enigo.key_sequence(&char.to_string());
            observer.wait();
        }
    });
}

/// Returns the child element which has the given name.
///
/// Example:
///
/// ```
/// extern crate gtk;
/// #[macro_use]
/// extern crate gtk_test;
///
/// use gtk::{prelude::BuildableExtManual, Button, prelude::ContainerExt, prelude::WidgetExt, Window, WindowType};
///
/// # fn main() {
/// gtk::init().expect("GTK init failed");
/// let but = Button::new();
/// let w = Window::new(WindowType::Toplevel);
///
/// but.set_widget_name("Button");
/// w.add(&but);
///
/// gtk_test::find_child_by_name::<Button, Window>(&w, "Button").expect("failed to find child");
/// // Or even better:
/// let but: Button = gtk_test::find_child_by_name(&w, "Button").expect("failed to find child");
/// # }
/// ```
pub fn find_child_by_name<C: IsA<Widget>, W: Clone + IsA<Object> + IsA<Widget>>(
    parent: &W,
    name: &str,
) -> Option<C> {
    find_widget_by_name(parent, name).and_then(|widget| widget.downcast().ok())
}

/// Returns the child widget which has the given name.
///
/// Example:
///
/// ```
/// extern crate gtk;
/// #[macro_use]
/// extern crate gtk_test;
///
/// use gtk::{Button, prelude::ContainerExt, prelude::WidgetExt, Window, WindowType};
///
/// # fn main() {
/// gtk::init().expect("GTK init failed");
/// let but = Button::new();
/// let w = Window::new(WindowType::Toplevel);
///
/// but.set_widget_name("Button");
/// w.add(&but);
///
/// gtk_test::find_widget_by_name(&w, "Button").unwrap();
/// # }
/// ```
pub fn find_widget_by_name<W: Clone + IsA<Object> + IsA<Widget>>(
    parent: &W,
    name: &str,
) -> Option<Widget> {
    if let Ok(container) = parent.clone().dynamic_cast::<Container>() {
        for child in container.children() {
            if child.widget_name() == name {
                return Some(child);
            }
            if let Some(widget) = find_widget_by_name(&child, name) {
                return Some(widget);
            }
        }
    } else if let Ok(bin) = parent.clone().dynamic_cast::<Bin>() {
        if let Some(child) = bin.child() {
            if child.widget_name() == name {
                return Some(child);
            }
            if let Some(widget) = find_widget_by_name(&child, name) {
                return Some(widget);
            }
        }
    }
    None
}

/// Focus on the given widget.
///
/// Example:
///
/// ```
/// extern crate gtk;
/// #[macro_use]
/// extern crate gtk_test;
///
/// use gtk::{Button, Inhibit, prelude::WidgetExt};
///
/// # fn main() {
/// gtk::init().expect("GTK init failed");
/// let but = Button::new();
///
/// but.connect_focus(|_, _| {
///     println!("focused!");
///     Inhibit(false)
/// });
/// gtk_test::focus(&but);
/// # }
/// ```
pub fn focus<W: Clone + IsA<Object> + IsA<Widget> + WidgetExt>(widget: &W) {
    wait_for_draw(widget, || {
        if !widget.has_focus() {
            widget.grab_focus();
            if let Ok(entry) = widget.clone().dynamic_cast::<Entry>() {
                // Hack to make it work on Travis.
                // Should use grab_focus_without_selecting() instead.
                entry.set_position(-1);
            }
        }
    });
}

/// Send a key press event to the given widget.
///
/// ## Warning!
///
/// Please note that the key-press event will "fail" if the window isn't on top of all other
/// windows (this is a common issue on OSX). Don't forget to bring the button's window on top
/// by using:
///
/// ```ignore
/// window.activate_focus();
/// ```
///
/// Example:
///
/// ```
/// #[macro_use]
/// extern crate gtk_test;
///
/// use gtk_test::gtk::{Entry, Inhibit, prelude::WidgetExt};
///
/// # fn main() {
/// gtk_test::gtk::init().expect("GTK init failed");
/// let entry = Entry::new();
/// entry.connect_key_press_event(|_, _| {
///     println!("key pressed");
///     Inhibit(false)
/// });
/// gtk_test::key_press(&entry, gtk_test::gdk::keys::constants::Agrave);
/// # }
/// ```
// FIXME: don't wait the observer for modifier keys like shift?
pub fn key_press<W: Clone + IsA<Object> + IsA<Widget> + WidgetExt>(widget: &W, key: Key) {
    wait_for_draw(widget, || {
        let observer = observer_new!(widget, connect_key_press_event, |_, _| { Inhibit(false) });
        focus(widget);
        let mut enigo = Enigo::new();
        enigo.key_down(gdk_key_to_enigo_key(key));
        observer.wait();
    });
}

/// Send a key release event to the given widget.
///
/// ## Warning!
///
/// Please note that the key-release event will "fail" if the window isn't on top of all other
/// windows (this is a common issue on OSX). Don't forget to bring the button's window on top
/// by using:
///
/// ```ignore
/// window.activate_focus();
/// ```
///
/// Example:
///
/// ```
/// #[macro_use]
/// extern crate gtk_test;
///
/// use gtk_test::gtk::{Entry, Inhibit, prelude::WidgetExt};
///
/// # fn main() {
/// gtk_test::gtk::init().expect("GTK init failed");
/// let entry = Entry::new();
/// entry.connect_key_release_event(|_, _| {
///     println!("key released");
///     Inhibit(false)
/// });
/// gtk_test::key_release(&entry, gtk_test::gdk::keys::constants::Agrave);
/// # }
/// ```
pub fn key_release<W: Clone + IsA<Object> + IsA<Widget> + WidgetExt>(widget: &W, key: Key) {
    wait_for_draw(widget, || {
        let observer = observer_new!(widget, connect_key_release_event, |_, _| { Inhibit(false) });
        focus(widget);
        let mut enigo = Enigo::new();
        enigo.key_up(gdk_key_to_enigo_key(key));
        observer.wait();
    });
}

/// Wait for events the specified amount the milliseconds.
///
/// Very convenient when you need GTK to update the UI to let it process some events.
///
/// Example:
///
/// ```
/// extern crate gtk;
/// extern crate gtk_test;
///
/// # fn main() {
/// gtk::init().expect("GTK init failed");
/// gtk_test::wait(1000); // wait for a second
/// # }
/// ```
pub fn wait(ms: u32) {
    gtk::glib::timeout_add(std::time::Duration::from_millis(ms as u64), || {
        gtk::main_quit();
        Continue(false)
    });
    gtk::main();
}

/// Process all pending events and then return.
///
/// This function is called in all functions related to events handling (like `key_release` for
/// example).
///
/// Example:
///
/// ```
/// extern crate gtk;
/// extern crate gtk_test;
///
/// # fn main() {
/// gtk::init().expect("GTK init failed");
/// gtk_test::run_loop();
/// # }
/// ```
pub fn run_loop() {
    while gtk::events_pending() {
        gtk::main_iteration();
    }
}

/// Wait until the given condition returns `true`.
///
/// Example:
///
/// ```
/// extern crate gtk;
/// extern crate gtk_test;
///
/// # fn main() {
/// gtk::init().expect("GTK init failed");
/// let mut x = 0;
///
/// gtk_test::wait_until_done(move || {
///     x += 1;
///     x > 10
/// });
/// # }
/// ```
pub fn wait_until_done<F: FnMut() -> bool>(mut f: F) {
    while !f() {
        run_loop();
    }
}

/// Wait for a widget to be drawn.
///
/// Example:
///
/// ```
/// extern crate gtk;
/// extern crate gtk_test;
///
/// use gtk::{prelude::WidgetExt, Window, WindowType};
///
/// # fn main() {
/// gtk::init().expect("GTK init failed");
/// let mut w = Window::new(WindowType::Toplevel);
///
/// w.show_all();
/// gtk_test::wait_for_draw(&w, || {
///     println!("drawn!");
/// });
/// # }
/// ```
pub fn wait_for_draw<W, F: FnOnce()>(widget: &W, callback: F)
where
    W: IsA<Object> + IsA<Widget> + WidgetExt,
{
    if widget.ancestor(Window::static_type()).is_none() {
        return;
    }
    gtk::test_widget_wait_for_draw(widget);
    callback();
}

fn gdk_key_to_enigo_key(key: Key) -> enigo::Key {
    use enigo::Key::*;
    match key {
        key::Return => Return,
        key::Tab => Tab,
        key::space => Space,
        key::BackSpace => Backspace,
        key::Escape => Escape,
        key::Super_L | key::Super_R => Meta,
        key::Control_L | key::Control_R => Control,
        key::Shift_L | key::Shift_R => Shift,
        key::Shift_Lock => CapsLock,
        key::Alt_L | key::Alt_R => Alt,
        key::End => End,
        key::Option => Option,
        key::Home => Home,
        key::Page_Down => PageDown,
        key::Page_Up => PageUp,
        key::leftarrow => LeftArrow,
        key::rightarrow => RightArrow,
        key::downarrow => DownArrow,
        key::uparrow => UpArrow,
        key::F1 => F1,
        key::F2 => F2,
        key::F3 => F3,
        key::F4 => F4,
        key::F5 => F5,
        key::F6 => F6,
        key::F7 => F7,
        key::F8 => F8,
        key::F9 => F9,
        key::F10 => F10,
        key::F11 => F11,
        key::F12 => F12,
        _ => {
            if let Some(char) = key.to_unicode() {
                Layout(char)
            } else {
                Raw(*key as u16)
            }
        }
    }
}
