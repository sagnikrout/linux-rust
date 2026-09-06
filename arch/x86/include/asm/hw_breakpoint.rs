//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/hw_breakpoint.h
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
// The name should probably be something dealt in
// a higher level. While dealing with the user
// (display/resolving)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_hw_breakpoint {
    pub address: c_ulong,
    pub mask: c_ulong,
    pub len: u8,
    pub type: u8,
}

// Available HW breakpoint length encodings
pub const X86_BREAKPOINT_LEN_X: c_uint = 0x40;
pub const X86_BREAKPOINT_LEN_1: c_uint = 0x40;
pub const X86_BREAKPOINT_LEN_2: c_uint = 0x44;
pub const X86_BREAKPOINT_LEN_4: c_uint = 0x4c;

pub const X86_BREAKPOINT_LEN_8: c_uint = 0x48;

// Available HW breakpoint type encodings
// trigger on instruction execute
pub const X86_BREAKPOINT_EXECUTE: c_uint = 0x80;
// trigger on memory write
pub const X86_BREAKPOINT_WRITE: c_uint = 0x81;
// trigger on memory read or write
pub const X86_BREAKPOINT_RW: c_uint = 0x83;
// Total number of available HW breakpoint registers
pub const HBP_NUM: c_int = 4;

extern "C" {
    pub fn arch_check_bp_in_kernelspace(hw: *mut arch_hw_breakpoint) -> c_int;
}
extern "C" {
    pub fn arch_install_hw_breakpoint(bp: *mut perf_event) -> c_int;
}
extern "C" {
    pub fn arch_uninstall_hw_breakpoint(bp: *mut perf_event);
}
extern "C" {
    pub fn hw_breakpoint_pmu_read(bp: *mut perf_event);
}
extern "C" {
    pub fn hw_breakpoint_pmu_unthrottle(bp: *mut perf_event);
}
extern "C" {
    pub fn encode_dr7(drnum: c_int, len: c_uint, type: c_uint) -> c_ulong;
}
extern "C" {
    pub fn decode_dr7(dr7: c_ulong, bpnum: c_int, len: *mut unsigned, type: *mut unsigned) -> c_int;
}
