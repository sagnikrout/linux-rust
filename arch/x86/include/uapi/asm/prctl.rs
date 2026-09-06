//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/uapi/asm/prctl.h
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
pub const ARCH_SET_GS: c_uint = 0x1001;
pub const ARCH_SET_FS: c_uint = 0x1002;
pub const ARCH_GET_FS: c_uint = 0x1003;
pub const ARCH_GET_GS: c_uint = 0x1004;
pub const ARCH_GET_CPUID: c_uint = 0x1011;
pub const ARCH_SET_CPUID: c_uint = 0x1012;
pub const ARCH_GET_XCOMP_SUPP: c_uint = 0x1021;
pub const ARCH_GET_XCOMP_PERM: c_uint = 0x1022;
pub const ARCH_REQ_XCOMP_PERM: c_uint = 0x1023;
pub const ARCH_GET_XCOMP_GUEST_PERM: c_uint = 0x1024;
pub const ARCH_REQ_XCOMP_GUEST_PERM: c_uint = 0x1025;
pub const ARCH_XCOMP_TILECFG: c_int = 17;
pub const ARCH_XCOMP_TILEDATA: c_int = 18;
pub const ARCH_MAP_VDSO_X32: c_uint = 0x2001;
pub const ARCH_MAP_VDSO_32: c_uint = 0x2002;
pub const ARCH_MAP_VDSO_64: c_uint = 0x2003;
// Don't use 0x3001-0x3004 because of old glibcs
pub const ARCH_GET_UNTAG_MASK: c_uint = 0x4001;
pub const ARCH_ENABLE_TAGGED_ADDR: c_uint = 0x4002;
pub const ARCH_GET_MAX_TAG_BITS: c_uint = 0x4003;
pub const ARCH_FORCE_TAGGED_SVA: c_uint = 0x4004;
pub const ARCH_SHSTK_ENABLE: c_uint = 0x5001;
pub const ARCH_SHSTK_DISABLE: c_uint = 0x5002;
pub const ARCH_SHSTK_LOCK: c_uint = 0x5003;
pub const ARCH_SHSTK_UNLOCK: c_uint = 0x5004;
pub const ARCH_SHSTK_STATUS: c_uint = 0x5005;
// ARCH_SHSTK_ features bits

