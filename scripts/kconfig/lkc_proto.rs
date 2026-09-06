//! Automatically rewritten from C Header to Rust Module
//! Source: scripts/kconfig/lkc_proto.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0

// confdata.c
extern "C" {
    pub fn conf_parse(name: *const c_char);
}
extern "C" {
    pub fn conf_read(name: *const c_char) -> c_int;
}
extern "C" {
    pub fn conf_read_simple(name: *const c_char, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn conf_write_defconfig(name: *const c_char) -> c_int;
}
extern "C" {
    pub fn conf_write(name: *const c_char) -> c_int;
}
extern "C" {
    pub fn conf_write_autoconf(overwrite: c_int) -> c_int;
}
extern "C" {
    pub fn conf_set_changed(val: bool);
}
extern "C" {
    pub fn conf_get_changed() -> bool;
}
extern "C" {
    pub fn conf_set_changed_callback((*fn)(bool): *mut c_void);
}
extern "C" {
    pub fn conf_set_message_callback(s): *const *const void (fn)(char);
}
extern "C" {
    pub fn conf_errors() -> bool;
}
// symbol.c
extern "C" {
    pub fn sym_lookup(name: *const c_char, flags: c_int) -> *mut symbol;
}
extern "C" {
    pub fn sym_find(name: *const c_char) -> *mut symbol;
}
extern "C" {
    pub fn print_symbol_for_listconfig(sym: *mut symbol);
}
extern "C" {
    pub fn sym_re_search(pattern: *const c_char) -> *mut *mut symbol;
}
extern "C" {
    pub fn sym_type_name(type: symbol_type) -> *const c_char;
}
extern "C" {
    pub fn sym_calc_value(sym: *mut symbol);
}
extern "C" {
    pub fn sym_dep_errors() -> bool;
}
extern "C" {
    pub fn sym_get_type(sym: *const symbol) -> symbol_type;
}
extern "C" {
    pub fn sym_tristate_within_range(sym: *const symbol, tri: tristate) -> bool;
}
extern "C" {
    pub fn sym_set_tristate_value(sym: *mut symbol, tri: tristate) -> bool;
}
extern "C" {
    pub fn choice_set_value(choice: *mut menu, sym: *mut symbol);
}
extern "C" {
    pub fn sym_toggle_tristate_value(sym: *mut symbol) -> tristate;
}
extern "C" {
    pub fn sym_string_valid(sym: *mut symbol, newval: *const c_char) -> bool;
}
extern "C" {
    pub fn sym_string_within_range(sym: *mut symbol, str: *const c_char) -> bool;
}
extern "C" {
    pub fn sym_set_string_value(sym: *mut symbol, newval: *const c_char) -> bool;
}
extern "C" {
    pub fn sym_is_changeable(sym: *const symbol) -> bool;
}
extern "C" {
    pub fn sym_get_string_value(sym: *mut symbol) -> *const c_char;
}
extern "C" {
    pub fn prop_get_type_name(type: prop_type) -> *const c_char;
}
// expr.c
