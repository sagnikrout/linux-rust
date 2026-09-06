//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/device-id/x86_cpu.h
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

pub type kernel_ulong_t = c_ulong;

// Wild cards for x86_cpu_id::vendor, family, model and feature
pub const X86_VENDOR_ANY: c_uint = 0xffff;
pub const X86_FAMILY_ANY: c_int = 0;
pub const X86_MODEL_ANY: c_int = 0;
pub const X86_STEPPING_ANY: c_int = 0;
pub const X86_STEP_MIN: c_int = 0;
pub const X86_STEP_MAX: c_uint = 0xf;
pub const X86_PLATFORM_ANY: c_uint = 0x0;

pub const X86_CPU_TYPE_ANY: c_int = 0;
//
// Match x86 CPUs for CPU specific drivers.
// See documentation of "x86_match_cpu" for details.
//
// MODULE_DEVICE_TABLE expects this struct to be called x86cpu_device_id.
// Although gcc seems to ignore this error, clang fails without this define.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct x86_cpu_id {
    pub vendor: __u16,
    pub family: __u16,
    pub model: __u16,
    pub steppings: __u16,
    pub /: *mut *mut __u16 feature; / bit index,
// Solely for kernel-internal use: DO NOT EXPORT to userspace!
    pub flags: __u16,
    pub platform_mask: __u8,
    pub type: __u8,
    pub driver_data: kernel_ulong_t,
}
