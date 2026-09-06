//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/codetag.h
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
//
// code tagging framework
//

// codetag flags

//
// An instance of this structure is created in a special ELF section at every
// code location being tagged.  At runtime, the special section is treated as
// an array of these.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct codetag {
    pub flags: c_uint,
    pub lineno: c_uint,
    pub modname: *const c_char,
    pub function: *const c_char,
    pub filename: *const c_char,
    pub __aligned(8): },
#[repr(C)]
#[derive(Copy, Clone)]
pub union codetag_ref {
    pub ct: *mut codetag,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct codetag_type_desc {
    pub section: *const c_char,
    pub tag_size: usize,
    pub end): *mut *mut codetag start, codetag,
    pub end): *mut *mut codetag start, codetag,

    pub new_mod): *mut *mut *mut void (module_replaced)(struct module mod, struct module,
    pub size): *mut *mut *mut bool (needs_section_mem)(struct module mod, unsigned long,
    pub align): unsigned int prepend, unsigned long,
    pub used): *mut *mut *mut void (free_section_mem)(struct module mod, bool,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct codetag_iterator {
    pub cttype: *mut codetag_type,
    pub cmod: *mut codetag_module,
    pub mod_id: c_ulong,
    pub ct: *mut codetag,
    pub mod_seq: c_ulong,
}

extern "C" {
    pub fn codetag_lock_module_list(cttype: *mut codetag_type);
}
extern "C" {
    pub fn codetag_trylock_module_list(cttype: *mut codetag_type) -> bool;
}
extern "C" {
    pub fn codetag_unlock_module_list(cttype: *mut codetag_type);
}
extern "C" {
    pub fn codetag_get_content_id(cttype: *mut codetag_type) -> c_ulong;
}
extern "C" {
    pub fn codetag_get_count(cttype: *mut codetag_type) -> c_uint;
}
extern "C" {
    pub fn codetag_get_ct_iter(cttype: *mut codetag_type) -> codetag_iterator;
}
extern "C" {
    pub fn codetag_to_text(out: *mut seq_buf, ct: *mut codetag);
}

extern "C" {
    pub fn codetag_free_module_sections(mod: *mut module);
}
extern "C" {
    pub fn codetag_module_replaced(mod: *mut module, new_mod: *mut module);
}
extern "C" {
    pub fn codetag_load_module(mod: *mut module) -> c_int;
}
extern "C" {
    pub fn codetag_unload_module(mod: *mut module);
}

