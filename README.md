# gtk-test [![Build Status](https://github.com/gtk-rs/gtk-test/actions/workflows/linux_ci.yml/badge.svg)](https://github.com/gtk-rs/gtk-test/actions/workflows/linux_ci.yml) [![Build Status](https://github.com/gtk-rs/gtk-test/actions/workflows/mac_ci.yml/badge.svg)](https://github.com/gtk-rs/gtk-test/actions/workflows/mac_ci.yml) [![Build status](https://ci.appveyor.com/api/projects/status/h72xnw2ghjpy2m9y/branch/master?svg=true)](https://ci.appveyor.com/project/GuillaumeGomez/gtk-test/branch/master)

This projects allows you to test your GTK's applications UI. It has to be used with [gtk-rs](https://gtk-rs.org) crates.

## How does it work?

It's quite simple actually (even though you have to perform a few more things on OSX to make it work as expected...) :

```rust
gtk::init().unwrap(); // You need to init GTK otherwise it'll just crash...
```

Then you build your UI as you would in normal time (using `Glade` or by hand). Only one thing actually changes: you must not call `gtk::main`!

Once you have built your UI, just call the `gtk_test` macros/functions to test it. Just one note about this though: sometimes, you need to let time for GTK to process some events. For example, if you clicked on a button and you have an associated action to it, it's more careful to use `gtk_test::wait`.

Another recommended thing is to give focus to the window in case you have to interact with it (to click on a button or to input some text...):

```rust
let w = gtk::Window::new();
// ...
w.activate_focus();
```

### General setup

When running test, you need to specify that you only want **ONE** thread. To do so:

```bash
cargo test -- --test-threads=1
```

Otherwise, GTK contexts might conflict into each others.

### Specific setup for OSX

A few more things have to be done on OSX to make this work. First, you won't be able to add the `#[test]` attribute to your functions, it doesn't work. Instead, you have to write your test just like you would write a normal binary (so with a `main` function as entry point).

A short example (you can find the full version in the `tests` folder of this repository):

```rust
fn main() {
    let (w, l, b) = init_ui();

    assert_text!(l, "Test");
    w.activate_focus();
    gtk_test::click(&b);
    gtk_test::wait(1000); // to be sure that GTK has updated the label's text
    assert_text!(l, "Clicked");
}
```

Then you need to add into your `Cargo.toml` file:

```toml
[[test]]
harness = false # This is the important line!
name = "basic"
```

It allows your test to be run as a "normal" binary.

### Example?

You can find a few in the tests folder. Just copy/paste it and you're good to go (don't forget to add the missing pieces in your `Cargo.toml` file!).

### Using it on CI?

It's actually possible (only tested for travis though). You "just" need a window manager. Here's what you have to ad in your Ci configuration file to make it work:

Install the following packages:

 * libcurl4-openssl-dev
 * libelf-dev
 * libdw-dev
 * binutils-dev
 * libsoup2.4-dev
 * libxtst-dev
 * at-spi2-core
 * openbox

Then you to execute the following commands:

```
> "export DISPLAY=:99.0"
> "sh -e /etc/init.d/xvfb start"
> sleep 3 # give xvfb some time to start
> openbox &
```

Take a look at our `.travis.yml` file to see how we set things up.
