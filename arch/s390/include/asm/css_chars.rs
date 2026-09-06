//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/css_chars.h
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
pub struct css_general_char {
    pub 12: u64 :,
    pub /: *mut *mut u64 dynio : 1; / bit 12,
    pub 4: u64 :,
    pub /: *mut *mut u64 eadm : 1; / bit 17,
    pub 23: u64 :,
    pub /: *mut *mut u64 aif : 1; / bit 41,
    pub 3: u64 :,
    pub /: *mut *mut u64 mcss : 1; / bit 45,
    pub /: *mut *mut u64 fcs : 1; / bit 46,
    pub 1: u64 :,
    pub /: *mut *mut u64 ext_mb : 1; / bit 48,
    pub 7: u64 :,
    pub /: *mut *mut u64 aif_tdd : 1; / bit 56,
    pub 1: u64 :,
    pub /: *mut *mut u64 qebsm : 1; / bit 58,
    pub 2: u64 :,
    pub /: *mut *mut u64 aiv : 1; / bit 61,
    pub 2: u64 :,
    pub 3: u64 :,
    pub /: *mut *mut u64 aif_qdio : 1;/ bit 67,
    pub 12: u64 :,
    pub /: *mut *mut u64 eadm_rf : 1; / bit 80,
    pub 1: u64 :,
    pub /: *mut *mut u64 cib : 1; / bit 82,
    pub 5: u64 :,
    pub /: *mut *mut u64 fcx : 1; / bit 88,
    pub 19: u64 :,
    pub /: *mut *mut u64 alt_ssi : 1; / bit 108,
    pub 1: u64 :,
    pub /: *mut *mut u64 narf : 1; / bit 110,
    pub 5: u64 :,
    pub /: *mut *mut u64 enarf: 1; / bit 116,
    pub 6: u64 :,
    pub /: *mut *mut u64 util_str : 1;/ bit 123,
    pub __packed: },
    pub css_general_characteristics: extern struct css_general_char,
