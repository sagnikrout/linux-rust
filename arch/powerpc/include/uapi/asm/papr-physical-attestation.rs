//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/uapi/asm/papr-physical-attestation.h
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
// Defined in PAPR 2.13+ 21.6 Attestation Command Structures.
// User space pass this struct and the max size should be 4K.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct papr_phy_attest_io_block {
    pub version: __u8,
    pub command: __u8,
    pub TCG_major_ver: __u8,
    pub TCG_minor_ver: __u8,
    pub length: __be32,
    pub correlator: __be32,
    pub payload: [__u8; PAPR_PHYATTEST_MAX_INPUT],
}

//
// ioctl for /dev/papr-physical-attestation. Returns a attestation
// command fd handle
//

