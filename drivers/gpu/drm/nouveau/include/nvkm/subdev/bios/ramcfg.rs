//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/subdev/bios/ramcfg.h
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


// SPDX-License-Identifier: MIT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvbios_ramcfg {
    pub rammap_ver: unsigned,
    pub rammap_hdr: unsigned,
    pub rammap_min: unsigned,
    pub rammap_max: unsigned,
    pub rammap_00_16_20:1: unsigned,
    pub rammap_00_16_40:1: unsigned,
    pub rammap_00_17_02:1: unsigned,
}

// empty: 4
// empty: 6
// empty: 8
// empty: 15
// empty: 17
// empty: 22, 23
extern "C" {
    pub fn nvbios_ramcfg_count(: *mut nvkm_bios) -> u8;
}
extern "C" {
    pub fn nvbios_ramcfg_index(: *mut nvkm_subdev) -> u8;
}
