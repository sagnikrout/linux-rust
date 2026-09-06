//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/vendor_extensions.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright 2024 Rivos, Inc
//

//
// The extension keys of each vendor must be strictly less than this value.
//
pub const RISCV_ISA_VENDOR_EXT_MAX: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct riscv_isavendorinfo {
    pub RISCV_ISA_VENDOR_EXT_MAX): DECLARE_BITMAP(isa,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct riscv_isa_vendor_ext_data_list {
    pub is_initialized: bool,
    pub ext_data_count: usize,
    pub ext_data: *const riscv_isa_ext_data,
    pub per_hart_isa_bitmap: [riscv_isavendorinfo; NR_CPUS],
    pub all_harts_isa_bitmap: riscv_isavendorinfo,
}

//
// The alternatives need some way of distinguishing between vendor extensions
// and errata. Incrementing all of the vendor extension keys so they are at
// least 0x8000 accomplishes that.
//
pub const RISCV_VENDOR_EXT_ALTERNATIVES_BASE: c_uint = 0x8000;

extern "C" {
    pub fn __riscv_isa_vendor_extension_available(cpu: c_int, vendor: c_ulong, bit: c_uint) -> bool;
}

extern "C" {
    pub fn __riscv_isa_vendor_extension_available(_arg: VENDOR_EXT_ALL_CPUS, _arg: vendor, _arg: ext) -> return;
}
extern "C" {
    pub fn __riscv_isa_vendor_extension_available(_arg: VENDOR_EXT_ALL_CPUS, _arg: vendor, _arg: ext) -> return;
}
extern "C" {
    pub fn __riscv_isa_vendor_extension_available(_arg: cpu, _arg: vendor, _arg: ext) -> return;
}
extern "C" {
    pub fn __riscv_isa_vendor_extension_available(_arg: cpu, _arg: vendor, _arg: ext) -> return;
}
