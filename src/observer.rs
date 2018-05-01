use std::cell::RefCell;
use std::rc::Rc;

pub struct Observer<SIGNAL> {
    result: Rc<RefCell<Option<SIGNAL>>>,
}

impl<SIGNAL: Clone + 'static> Observer<SIGNAL> {
    pub fn new<F>(stream: &EventStream<SIGNAL>, predicate: F) -> Self
    where F: Fn(&SIGNAL) -> bool + 'static {
        let result = Rc::new(RefCell::new(None));
        let res = result.clone();
        stream.observe(move |signal| {
            if predicate(signal) {
                *res.borrow_mut() = Some(signal.clone());
            }
        });
        Self {
            result,
        }
    }

    pub fn wait(&self) -> SIGNAL {
        loop {
            if let Ok(ref result) = self.result.try_borrow() {
                if result.is_some() {
                    break;
                }
            }
            ::run_loop();
        }
        self.result.borrow_mut().take()
            .expect("Message to take")
    }
}

#[macro_export]
macro_rules! observer_new {
    ($component:expr, $pat:pat) => {
        $crate::Observer::new($component.stream(), |signal|
            if let $pat = signal {
                true
            }
            else {
                false
            }
        );
    };
}

#[macro_export]
macro_rules! observer_wait {
    (let $($variant:ident)::*($name1:ident, $name2:ident $(,$rest:ident)*) = $observer:expr) => {
        let ($name1, $name2 $(, $rest)*) = {
            let signal = $observer.wait();
            if let $($variant)::*($name1, $name2 $(, $rest)*) = signal {
                ($name1, $name2 $(, $rest)*)
            }
            else {
                panic!("Wrong message type.");
            }
        };
    };
    (let $($variant:ident)::*($name:ident) = $observer:expr) => {
        let $name = {
            let signal = $observer.wait();
            if let $($variant)::*($name) = signal {
                $name
            }
            else {
                panic!("Wrong message type.");
            }
        };
    };
    (let $($variant:ident)::* = $observer:expr) => {
        let () = {
            let signal = $observer.wait();
            if let $($variant)::* = signal {
                ()
            }
            else {
                panic!("Wrong message type.");
            }
        };
    };
}
