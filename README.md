# `nstdapi`
A helper crate for `nstd` providing manipulation of an item.

# Features
## `capi`
- Mark function definitions as `#[no_mangle]`.
- Set function ABIs to `extern "C"`.
- Mark static item definitions as `#[no_mangle]`.
- Set static item declaration ABIs to `extern "C"`.
- Mark structures, enumerations, and unions as `#[repr(C)]`.
## `link`
- Implicitly enables `capi`.
- Turn function definitions into function declarations.
- Turn static item definitions into static item declarations.
