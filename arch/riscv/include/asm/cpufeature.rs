//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/cpufeature.h
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
// Copyright 2022-2024 Rivos, Inc
//

//
// These are probed via a device_initcall(), via either the SBI or directly
// from the corresponding CSRs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct riscv_cpuinfo {
    pub mvendorid: c_ulong,
    pub marchid: c_ulong,
    pub mimpid: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct riscv_isainfo {
    pub RISCV_ISA_EXT_MAX): DECLARE_BITMAP(isa,,
}

// Per-cpu ISA extensions.
extern "C" {
    pub fn riscv_user_isa_enable() -> void __init;
}

// Used to declare pure "lasso" extension (Zk for instance)

// Used to declare extensions that are a superset of other extensions (Zvbb for instance)

extern "C" {
    pub fn check_unaligned_access_emulated_all_cpus() -> bool __init;
}
extern "C" {
    pub fn unaligned_access_init();
}
extern "C" {
    pub fn cpu_online_unaligned_access_init(cpu: c_uint) -> c_int;
}

extern "C" {
    pub fn unaligned_emulation_finish();
}
extern "C" {
    pub fn unaligned_ctl_available() -> bool;
}

extern "C" {
    pub fn misaligned_traps_can_delegate() -> bool;
}

extern "C" {
    pub fn check_vector_unaligned_access_emulated_all_cpus() -> bool __init;
}

extern "C" {
    pub fn check_vector_unaligned_access_emulated(__always_unused: *mut *mut work_work);
}

extern "C" {
    pub fn static_branch_likely(_arg: &fast_unaligned_access_speed_key) -> return;
}

extern "C" {
    pub fn riscv_get_elf_hwcap() -> c_ulong;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct riscv_isa_ext_data {
    pub id: c_uint,
    pub name: *const c_char,
    pub property: *const c_char,
    pub subset_ext_ids: *const c_uint,
    pub subset_ext_size: c_uint,
    pub isa_bitmap): *const *const *const int (validate)(struct riscv_isa_ext_data data, unsigned long,
}

extern "C" {
    pub fn riscv_isa_extension_base(isa_bitmap: *const c_ulong) -> c_ulong;
}
extern "C" {
    pub fn __riscv_isa_extension_available(_arg: hart_isa[cpu].isa, _arg: ext) -> return;
}
extern "C" {
    pub fn __riscv_isa_extension_available(_arg: hart_isa[cpu].isa, _arg: ext) -> return;
}
