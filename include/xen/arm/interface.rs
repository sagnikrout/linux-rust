//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/arm/interface.h
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
// Guest OS interface to ARM Xen.
//
// Stefano Stabellini <stefano.stabellini@eu.citrix.com>, Citrix, 2012
//

// (uint64_t *)&(hnd) = 0;	\

// Explicitly size integers that represent pfns in the interface with
// Xen so that we can have one ABI that works for 32 and 64 bit guests.
// Note that this means that the xen_pfn_t type may be capable of
// representing pfn's which the guest cannot represent in its own pfn
// type. However since pfn space is controlled by the guest this is
// fine since it simply wouldn't be able to create any sure pfns in
// the first place.
//
pub type xen_pfn_t = u64;

pub type xen_ulong_t = u64;

pub type xen_long_t = i64;

// Guest handles for primitive C types.
// Maximum number of virtual CPUs in multi-processor guests.
pub const MAX_VIRT_CPUS: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_vcpu_info {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_shared_info {
// TODO: Move pvclock definitions some place arch independent
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
// It is OK to have a 12 bytes struct with no padding because it is packed
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvclock_wall_clock {
    pub version: u32,
    pub sec: u32,
    pub nsec: u32,
    pub sec_hi: u32,
    pub __attribute__((__packed__)): },

