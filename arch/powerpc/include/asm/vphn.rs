//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/vphn.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// The H_HOME_NODE_ASSOCIATIVITY h_call returns 6 64-bit registers.
pub const VPHN_REGISTER_COUNT: c_int = 6;
//
// 6 64-bit registers unpacked into up to 24 be32 associativity values. To
// form the complete property we have to add the length in the first cell.
//

//
// The H_HOME_NODE_ASSOCIATIVITY hcall takes two values for flags:
// 1 for retrieving associativity information for a guest cpu
// 2 for retrieving associativity information for a host/hypervisor cpu
//
pub const VPHN_FLAG_VCPU: c_int = 1;
pub const VPHN_FLAG_PCPU: c_int = 2;
extern "C" {
    pub fn hcall_vphn(cpu: c_ulong, flags: u64, associativity: *mut __be32) -> c_long;
}
