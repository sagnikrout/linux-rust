//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/config.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_config_item {
    pub name: *mut c_char,
    pub value: *mut c_char,
    pub from_system_config: bool,
    pub node: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_config_section {
    pub name: *mut c_char,
    pub items: list_head,
    pub from_system_config: bool,
    pub node: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_config_set {
    pub sections: list_head,
}

extern "C" {
    pub fn int(: *const *const config_fn_t)(char, : *const c_char, : *mut c_void) -> typedef;
}
extern "C" {
    pub fn perf_default_config(: *const c_char, : *const c_char, : *mut c_void) -> c_int;
}
extern "C" {
    pub fn perf_config(fn: config_fn_t, : *mut c_void) -> c_int;
}
extern "C" {
    pub fn perf_config_scan(name: *const c_char, fmt: *const c_char, __scanf(2: ...), _arg: 3) -> c_int;
}
extern "C" {
    pub fn perf_config_int(dest: *mut c_int, : *const c_char, : *const c_char) -> c_int;
}
extern "C" {
    pub fn perf_config_u8(dest: *mut u8, name: *const c_char, value: *const c_char) -> c_int;
}
extern "C" {
    pub fn perf_config_u64(dest: *mut u64, : *const c_char, : *const c_char) -> c_int;
}
extern "C" {
    pub fn perf_config_bool(: *const c_char, : *const c_char) -> c_int;
}
extern "C" {
    pub fn config_error_nonbool(: *const c_char) -> c_int;
}
extern "C" {
    pub fn perf_config_system() -> c_int;
}
extern "C" {
    pub fn perf_config_global() -> c_int;
}
extern "C" {
    pub fn perf_config_set__delete(set: *mut perf_config_set);
}
extern "C" {
    pub fn perf_config__exit();
}
extern "C" {
    pub fn perf_config__set_variable(var: *const c_char, value: *const c_char) -> c_int;
}
//
// perf_config_sections__for_each - iterate thru all the sections
// @list: list_head instance to iterate
// @section: struct perf_config_section iterator
//

//
// perf_config_items__for_each - iterate thru all the items
// @list: list_head instance to iterate
// @item: struct perf_config_item iterator
//

//
// perf_config_set__for_each - iterate thru all the config section-item pairs
// @set: evlist instance to iterate
// @section: struct perf_config_section iterator
// @item: struct perf_config_item iterator
//

