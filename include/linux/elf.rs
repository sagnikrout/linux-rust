//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/elf.h
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

// Executables for which elf_read_implies_exec() returns TRUE will

pub const ELF32_GNU_PROPERTY_ALIGN: c_int = 4;
pub const ELF64_GNU_PROPERTY_ALIGN: c_int = 8;

// Optional callbacks to write extra ELF notes.

extern "C" {
    pub fn elf_coredump_extra_notes_size() -> c_int;
}
extern "C" {
    pub fn elf_coredump_extra_notes_write(cprm: *mut coredump_params) -> c_int;
}

//
// NT_GNU_PROPERTY_TYPE_0 header:
// Keep this internal until/unless there is an agreed UAPI definition.
// pr_type values (GNU_PROPERTY_*) are public and defined in the UAPI header.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnu_property {
    pub pr_type: u32,
    pub pr_datasz: u32,
}

