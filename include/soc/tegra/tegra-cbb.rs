//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/tegra/tegra-cbb.h
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
// Copyright (c) 2021-2022, NVIDIA CORPORATION. All rights reserved
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_cbb_error {
    pub code: *const c_char,
    pub source: *const c_char,
    pub desc: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_cbb {
    pub dev: *mut device,
    pub ops: *const tegra_cbb_ops,
    pub node: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_cbb_ops {
    pub v): *mut *mut *mut *mut int (debugfs_show)(struct tegra_cbb cbb, struct seq_file s, void,
    pub cbb): *mut *mut int (interrupt_enable)(struct tegra_cbb,
    pub cbb): *mut *mut void (error_enable)(struct tegra_cbb,
    pub cbb): *mut *mut void (fault_enable)(struct tegra_cbb,
    pub cbb): *mut *mut void (stall_enable)(struct tegra_cbb,
    pub cbb): *mut *mut void (error_clear)(struct tegra_cbb,
    pub cbb): *mut *mut u32 (get_status)(struct tegra_cbb,
}

extern "C" {
    pub fn tegra_cbb_print_err(file: *mut seq_file, fmt: *const c_char, ...);
}
extern "C" {
    pub fn tegra_cbb_print_cache(file: *mut seq_file, cache: u32);
}
extern "C" {
    pub fn tegra_cbb_print_prot(file: *mut seq_file, prot: u32);
}
extern "C" {
    pub fn tegra_cbb_register(cbb: *mut tegra_cbb) -> c_int;
}
extern "C" {
    pub fn tegra_cbb_fault_enable(cbb: *mut tegra_cbb);
}
extern "C" {
    pub fn tegra_cbb_stall_enable(cbb: *mut tegra_cbb);
}
extern "C" {
    pub fn tegra_cbb_error_clear(cbb: *mut tegra_cbb);
}
extern "C" {
    pub fn tegra_cbb_get_status(cbb: *mut tegra_cbb) -> u32;
}
