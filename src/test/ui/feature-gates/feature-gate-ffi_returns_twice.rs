// ignore-tidy-linelength
#![crate_type = "lib"]

extern {
    #[ffi_returns_twice] //~ ERROR the `#[ffi_returns_twice]` attribute is an experimental feature (see issue #58314)
    pub fn foo();
}
