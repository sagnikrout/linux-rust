//! Automatically rewritten from C Header to Rust Module
//! Source: include/hyperv/hvgdk_ext.h
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
// Type definitions for the Microsoft Hypervisor.
//

// Extended hypercalls
pub const HV_EXT_CALL_QUERY_CAPABILITIES: c_uint = 0x8001;
pub const HV_EXT_CALL_MEMORY_HEAT_HINT: c_uint = 0x8003;
// Extended hypercalls
// HV_EXT_OUTPUT_QUERY_CAPABILITIES

//
// The whole argument should fit in a page to be able to pass to the hypervisor
// in one hypercall.
//

// HvExtCallMemoryHeatHint hypercall
pub const HV_EXT_MEMORY_HEAT_HINT_TYPE_COLD_DISCARD: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_memory_hint {
    pub /: *mut *mut *mut u64 heat_type : 2; / HV_EXTMEM_HEAT_HINT_,
    pub 62: u64 reserved :,
    pub ranges: [hv_gpa_page_range; ],
    pub __packed: },
