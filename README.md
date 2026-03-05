# trusty-playground
A playground for working with Rust property-based and fuzz-testing.

## Exploring property-based tests
Some initial property-based tests have been added to this repo. Here's one way to run them explicitly.

`cargo test --test prop_tests`

Running `cargo test` runs these tests as well as the other tests it finds.

## Exploring UB (Undefined behaviour[s])
A couple of prerequsites need to be installed/configured in order to run miri which helps find undefined behaviours (UB) in Rust code. These include switching to Rust's `nightly` compiler and installing miri. A good place to find the instructions is <https://github.com/rust-lang/miri>. 

I also had to switch the Rust toolchain from the one homebrew installed to the more recent one I'd installed. I did this by setting the following environment variable: `export PATH="$HOME/.cargo/bin:$PATH"`.

Here's one way to run miri on the code we've explicitly added.

`cargo +nightly miri run --example ub_lab`

It can also be run for the various automated tests as follows:

`cargo +nightly miri test`

This helped uncover an implementation specific aspect of the property-based tests covered next.

### Splendid isolation

`MIRIFLAGS=-Zmiri-disable-isolation cargo +nightly miri test`

I've implemented an alternative that removes the need to specify the `MIRIFLAGS` environment parameter(s) by adding a custom configuration that is enabled when the tests are run with miri. Also the UB tests are now marked as `ignored` and only run as follows: `cargo test --test prop_tests -- --ignored` to prevent miri from rightly detecting the flaws these tests contain.
