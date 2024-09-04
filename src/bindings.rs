// Disable several lints. The auto-generated bindings do not conform to them.
#![allow(clippy::as_conversions)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::default_trait_access)]
#![allow(clippy::indexing_slicing)]
#![allow(clippy::missing_assert_message)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::pub_underscore_fields)]
#![allow(clippy::semicolon_if_nothing_returned)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::transmute_int_to_bool)]
#![allow(clippy::transmute_ptr_to_ptr)]
#![allow(clippy::type_complexity)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::used_underscore_binding)]
#![allow(clippy::useless_transmute)]
#![allow(missing_debug_implementations)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unsafe_op_in_unsafe_fn)]

// This `bindings.rs` is the one generated by `bindgen` in `build.rs`.
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
