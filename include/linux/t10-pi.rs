//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/t10-pi.h
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
// A T10 PI-capable target device can be formatted with different
// protection schemes.	Currently 0 through 3 are defined:
//
// Type 0 is regular (unprotected) I/O
//
// Type 1 defines the contents of the guard and reference tags
//
// Type 2 defines the contents of the guard and reference tags and
// uses 32-byte commands to seed the latter
//
// Type 3 defines the contents of the guard tag only
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum t10_dif_type {
    T10_PI_TYPE0_PROTECTION = 0x0,
    T10_PI_TYPE1_PROTECTION = 0x1,
    T10_PI_TYPE2_PROTECTION = 0x2,
    T10_PI_TYPE3_PROTECTION = 0x3,
}

extern "C" {
    pub fn blk_rq_pos(SECTOR_SHIFT: rq) >> (shift -) -> return;
}
//
// T10 Protection Information tuple.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct t10_pi_tuple {
    pub /: *mut *mut __be16 guard_tag; / Checksum,
    pub /: *mut *mut __be16 app_tag; / Opaque storage,
    pub /: *mut *mut __be32 ref_tag; / Target LBA or indirect LBA,
}

extern "C" {
    pub fn lower_32_bits(_arg: full_pi_ref_tag(rq)) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crc64_pi_tuple {
    pub guard_tag: __be64,
    pub app_tag: __be16,
    pub ref_tag: [__u8; 6],
}

//
// lower_48_bits() - return bits 0-47 of a number
// @n: the number we're accessing
//
extern "C" {
    pub fn lower_48_bits(_arg: full_pi_ref_tag(rq)) -> return;
}
