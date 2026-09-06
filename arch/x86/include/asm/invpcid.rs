//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/invpcid.h
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
// The memory clobber is because the whole point is to invalidate
// stale TLB entries and, especially if we're flushing global
// mappings, we don't want the compiler to reorder any subsequent
// memory accesses before the TLB flush.
//
pub const INVPCID_TYPE_INDIV_ADDR: c_int = 0;
pub const INVPCID_TYPE_SINGLE_CTXT: c_int = 1;
pub const INVPCID_TYPE_ALL_INCL_GLOBAL: c_int = 2;
pub const INVPCID_TYPE_ALL_NON_GLOBAL: c_int = 3;
// Flush all mappings for a given pcid and addr, not including globals.
// Flush all mappings for a given PCID, not including globals.
// Flush all mappings, including globals, for all PCIDs.
// Flush all mappings for all PCIDs except globals.
