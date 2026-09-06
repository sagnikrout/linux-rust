//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/pvclock-abi.h
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
// These structs MUST NOT be changed.
// They are the ABI between hypervisor and guest OS.
// Both Xen and KVM are using this.
//
// pvclock_vcpu_time_info holds the system time and the tsc timestamp
// of the last update. So the guest can use the tsc delta to get a
// more precise system time.  There is one per virtual cpu.
//
// pvclock_wall_clock references the point in time when the system
// time was zero (usually boot time), thus the guest calculates the
// current wall clock by adding the system time.
//
// Protocol for the "version" fields is: hypervisor raises it (making
// it uneven) before it starts updating the fields and raises it again
// (making it even) when it is done.  Thus the guest can make sure the
// time values it got are consistent by checking the version before
// and after reading them.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvclock_vcpu_time_info {
    pub version: u32,
    pub pad0: u32,
    pub tsc_timestamp: u64,
    pub system_time: u64,
    pub tsc_to_system_mul: u32,
    pub tsc_shift: i8,
    pub flags: u8,
    pub pad: [u8; 2],
    pub /: *mut *mut } __attribute__((__packed__)); / 32 bytes,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvclock_wall_clock {
    pub version: u32,
    pub sec: u32,
    pub nsec: u32,
    pub __attribute__((__packed__)): },

// PVCLOCK_COUNTS_FROM_ZERO broke ABI and can't be used anymore.

