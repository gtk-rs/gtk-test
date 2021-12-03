# gtk-test

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

It's actually possible (only tested for GitHub Actions though). You "just" need a display server. Here's what you have to add in your CI configuration file to make it work:

Install the following packages:

 * libgtk-3-dev
 * libxdo-dev

Then run the application in a virtual display environment using [xvfb-action.](https://github.com/GabrielBB/xvfb-action) For example:

```
- name: Run headless test
  uses: GabrielBB/xvfb-action@v1
  with:
    run: cargo test -- --test-threads=1
```

Take a look at our `.github/workflows/linux_ci.yml` file to see how we set things up.
