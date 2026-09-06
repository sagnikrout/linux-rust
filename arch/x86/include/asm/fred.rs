//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/fred.h
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
// Macros for Flexible Return and Event Delivery (FRED)
//

//
// FRED event return instruction opcodes for ERET{S,U}; supported in
// binutils >= 2.41.
//

//
// RSP is aligned to a 64-byte boundary before used to push a new stack frame
//

//
// Used for the return address for call emulation during code patching,
// and measured in 64-byte cache lines.
//
pub const FRED_CONFIG_REDZONE_AMOUNT: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fred_info {
// Event data: CR2, DR6, ...
    pub edata: c_ulong,
    pub resv: c_ulong,
}

// Full format of the FRED stack frame
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fred_frame {
    pub regs: pt_regs,
    pub info: fred_info,
}

extern "C" {
    pub fn asm_fred_entrypoint_user();
}
extern "C" {
    pub fn asm_fred_entrypoint_kernel();
}
extern "C" {
    pub fn asm_fred_entry_from_kvm(fred_ss: struct);
}
extern "C" {
    pub fn fred_entry_from_user(regs: *mut pt_regs) -> __visible void;
}
extern "C" {
    pub fn fred_entry_from_kernel(regs: *mut pt_regs) -> __visible void;
}
extern "C" {
    pub fn __fred_entry_from_kvm(regs: *mut pt_regs) -> __visible void;
}
// Can be called from noinstr code, thus __always_inline
extern "C" {
    pub fn cpu_init_fred_exceptions();
}
extern "C" {
    pub fn cpu_init_fred_rsps();
}
extern "C" {
    pub fn fred_complete_exception_setup();
}

