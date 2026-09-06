//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/sram.h
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
// Defines for the SRAM driver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sram_config {
    pub (*init)(void): *mut c_int,
    pub map_only_reserved: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sram_partition {
    pub base: *mut void __iomem,
    pub pool: *mut gen_pool,
    pub battr: bin_attribute,
    pub lock: mutex,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sram_dev {
    pub config: *const sram_config,
    pub dev: *mut device,
    pub virt_base: *mut void __iomem,
    pub no_memory_wc: bool,
    pub pool: *mut gen_pool,
    pub partition: *mut sram_partition,
    pub partitions: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sram_reserve {
    pub list: list_head,
    pub start: u32,
    pub size: u32,
    pub res: resource,
    pub export: bool,
    pub pool: bool,
    pub protect_exec: bool,
    pub label: *const c_char,
}

extern "C" {
    pub fn sram_add_protect_exec(part: *mut sram_partition) -> c_int;
}

