//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kexec_handover.h
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

extern "C" {
    pub fn kho_is_enabled() -> bool;
}
extern "C" {
    pub fn is_kho_boot() -> bool;
}
extern "C" {
    pub fn kho_preserve_folio(folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn kho_unpreserve_folio(folio: *mut folio);
}
extern "C" {
    pub fn kho_preserve_pages(page: *mut page, nr_pages: c_ulong) -> c_int;
}
extern "C" {
    pub fn kho_unpreserve_pages(page: *mut page, nr_pages: c_ulong);
}
extern "C" {
    pub fn kho_preserve_vmalloc(ptr: *mut c_void, preservation: *mut kho_vmalloc) -> c_int;
}
extern "C" {
    pub fn kho_unpreserve_vmalloc(preservation: *mut kho_vmalloc);
}
extern "C" {
    pub fn kho_unpreserve_free(mem: *mut c_void);
}
extern "C" {
    pub fn kho_restore_free(mem: *mut c_void);
}
extern "C" {
    pub fn kho_add_subtree(name: *const c_char, blob: *mut c_void, size: usize) -> c_int;
}
extern "C" {
    pub fn kho_remove_subtree(blob: *mut c_void);
}
extern "C" {
    pub fn kho_retrieve_subtree(name: *const c_char, phys: *mut phys_addr_t, size: *mut usize) -> c_int;
}
extern "C" {
    pub fn kho_memory_init();
}
extern "C" {
    pub fn kho_memory_init_early();
}
extern "C" {
    pub fn kho_scratch_overlap(phys: phys_addr_t, size: usize) -> bool;
}

extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}

