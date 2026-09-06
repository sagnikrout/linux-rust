//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/amd/nb.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_nb_bus_dev_range {
    pub bus: u8,
    pub dev_base: u8,
    pub dev_limit: u8,
}

extern "C" {
    pub fn early_is_amd_nb(value: u32) -> bool;
}
extern "C" {
    pub fn amd_flush_garts();
}
extern "C" {
    pub fn amd_numa_init() -> c_int;
}
extern "C" {
    pub fn amd_get_subcaches(_arg: c_int) -> c_int;
}
extern "C" {
    pub fn amd_set_subcaches(_arg: c_int, long: unsigned) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_l3_cache {
    pub indices: unsigned,
    pub subcaches: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_northbridge {
    pub misc: *mut pci_dev,
    pub link: *mut pci_dev,
    pub l3_cache: amd_l3_cache,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_northbridge_info {
    pub num: u16,
    pub flags: u64,
    pub nb: *mut amd_northbridge,
}

extern "C" {
    pub fn amd_nb_num() -> u16;
}
extern "C" {
    pub fn amd_nb_has_feature(feature: c_uint) -> bool;
}
// GART present only on Fam15h, up to model 0fh

pub const amd_nb_num(x): c_int = 0;

