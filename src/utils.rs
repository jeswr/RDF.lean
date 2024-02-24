extern crate lean_sys;
use std::{
    slice,
    str::{self, Utf8Error},
};

use self::lean_sys::{
    lean_array_push, lean_array_size, lean_array_uget, lean_initialize, lean_mk_empty_array,
    lean_mk_string_from_bytes, lean_obj_res, lean_string_cstr, lean_string_len,
};

pub fn lean_string_utf8(s: lean_obj_res) -> &'static [u8] {
    unsafe { slice::from_raw_parts(lean_string_cstr(s), lean_string_len(s)) }
}

pub fn lean_string_str(s: lean_obj_res) -> Result<&'static str, Utf8Error> {
    std::str::from_utf8(lean_string_utf8(s))
}

pub fn lean_mk_string_from_str(s: &str) -> lean_obj_res {
    unsafe { lean_mk_string_from_bytes(s.as_ptr(), s.len()) }
}

pub fn lean_mk_string_from_string(s: String) -> lean_obj_res {
    unsafe { lean_mk_string_from_bytes(s.as_ptr(), s.len()) }
}
