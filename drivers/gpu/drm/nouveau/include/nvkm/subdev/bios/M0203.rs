//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/subdev/bios/M0203.h
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
pub struct nvbios_M0203T {
pub const M0203T_TYPE_RAMCFG: c_uint = 0x00;
    pub type: u8,
    pub pointer: u16,
}

extern "C" {
    pub fn nvbios_M0203Te(: *mut nvkm_bios, ver: *mut u8, hdr: *mut u8, cnt: *mut u8, len: *mut u8) -> u32;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvbios_M0203E {
pub const M0203E_TYPE_DDR2: c_uint = 0x0;
pub const M0203E_TYPE_DDR3: c_uint = 0x1;
pub const M0203E_TYPE_GDDR3: c_uint = 0x2;
pub const M0203E_TYPE_GDDR5: c_uint = 0x3;
pub const M0203E_TYPE_HBM2: c_uint = 0x6;
pub const M0203E_TYPE_GDDR5X: c_uint = 0x8;
pub const M0203E_TYPE_GDDR6: c_uint = 0x9;
pub const M0203E_TYPE_SKIP: c_uint = 0xf;
    pub type: u8,
    pub strap: u8,
    pub group: u8,
}

extern "C" {
    pub fn nvbios_M0203Ee(: *mut nvkm_bios, idx: c_int, ver: *mut u8, hdr: *mut u8) -> u32;
}
