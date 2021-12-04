use std::cell::RefCell;
use std::rc::Rc;

/// Used to wait for a widget's signal.
///
/// It's recommended to use it with the [`observer_new`] macro.
///
/// Example:
///
/// ```
/// extern crate gtk;
/// #[macro_use]
/// extern crate gtk_test;
///
/// use gtk::prelude::GtkWindowExt;
///
/// # fn main() {
/// gtk::init().expect("initialization failed");
/// let window = gtk::Window::new(gtk::WindowType::Toplevel);
///
/// let observer = observer_new!(window, connect_activate_focus, |_|);
/// window.emit_activate_focus();
/// observer.wait();
/// # }
/// ```
pub struct Observer {
    result: Rc<RefCell<bool>>,
}

impl Observer {
    /// Returns a new observer.
    ///
    /// It's recommended to not use it directly as is but instead to use the [`observer_new`] macro.
    ///
    /// But anyway, here's an example using it as is:
    ///
    /// ```
    /// extern crate gtk;
    /// #[macro_use]
    /// extern crate gtk_test;
    ///
    /// use gtk::prelude::GtkWindowExt;
    ///
    /// # fn main() {
    /// gtk::init().expect("GTK init failed");
    ///
    /// let window = gtk::Window::new(gtk::WindowType::Toplevel);
    ///
    /// let observer = gtk_test::Observer::new();
    /// let inner = observer.get_inner().clone();
    /// window.connect_activate_focus(move |_| {
    ///     *inner.borrow_mut() = true;
    /// });
    ///
    /// window.emit_activate_focus();
    /// observer.wait();
    /// # }
    /// ```
    pub fn new() -> Observer {
        Observer {
            result: Rc::new(RefCell::new(false)),
        }
    }

    /// Returns the inner field. Just don't use it.
    pub fn get_inner(&self) -> &Rc<RefCell<bool>> {
        &self.result
    }

    /// Wait for the signal to be triggered.
    ///
    /// ```
    /// extern crate gtk;
    /// #[macro_use]
    /// extern crate gtk_test;
    ///
    /// use gtk::prelude::GtkWindowExt;
    ///
    /// # fn main() {
    /// gtk::init().expect("initialization failed");
    /// let window = gtk::Window::new(gtk::WindowType::Toplevel);
    ///
    /// let observer = observer_new!(window, connect_activate_focus, |_|);
    /// window.emit_activate_focus();
    /// observer.wait();
    /// # }
    /// ```
    pub fn wait(&self) {
        loop {
            if let Ok(ref result) = self.result.try_borrow() {
                if **result {
                    break;
                }
            }
            crate::run_loop();
        }
    }
}

impl Default for Observer {
    fn default() -> Self {
        Self::new()
    }
}
