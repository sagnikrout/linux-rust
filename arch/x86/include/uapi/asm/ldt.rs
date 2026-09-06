//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/uapi/asm/ldt.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// ldt.h
//
// Definitions of structures used with the modify_ldt system call.
//
// Maximum number of LDT entries supported.
pub const LDT_ENTRIES: c_int = 8192;
// The size of each LDT entry.
pub const LDT_ENTRY_SIZE: c_int = 8;
//
// Note on 64bit base and limit is ignored and you cannot set DS/ES/CS
// not to the default values if you still want to do syscalls. This
// call is more for 32bit mode therefore.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_desc {
    pub entry_number: c_uint,
    pub base_addr: c_uint,
    pub limit: c_uint,
    pub seg_32bit:1: c_uint,
    pub contents:2: c_uint,
    pub read_exec_only:1: c_uint,
    pub limit_in_pages:1: c_uint,
    pub seg_not_present:1: c_uint,
    pub useable:1: c_uint,

//
// Because this bit is not present in 32-bit user code, user
// programs can pass uninitialized values here.  Therefore, in
// any context in which a user_desc comes from a 32-bit program,
// the kernel must act as though lm == 0, regardless of the
// actual value.
//
    pub lm:1: c_uint,

}

pub const MODIFY_LDT_CONTENTS_DATA: c_int = 0;
pub const MODIFY_LDT_CONTENTS_STACK: c_int = 1;
pub const MODIFY_LDT_CONTENTS_CODE: c_int = 2;

