//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/vmcore_info.h
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
// The per-cpu notes area is a list of notes terminated by a "NULL"
// note header.  For kdump, the code in vmcore.c runs in the context
// of the second kernel to combine them into one note.
//

// Per cpu memory for storing cpu states in case of system crash.
extern "C" {
    pub fn crash_update_vmcoreinfo_safecopy(ptr: *mut c_void);
}
extern "C" {
    pub fn crash_save_vmcoreinfo();
}
extern "C" {
    pub fn arch_crash_save_vmcoreinfo();
}
extern "C" {
    pub fn vmcoreinfo_append_str(fmt: *const c_char, ...);
}
extern "C" {
    pub fn paddr_vmcoreinfo_note() -> phys_addr_t;
}

extern "C" {
    pub fn final_note(buf: *mut Elf_Word);
}

extern "C" {
    pub fn hwerr_log_error_type(src: hwerr_error_type);
}

