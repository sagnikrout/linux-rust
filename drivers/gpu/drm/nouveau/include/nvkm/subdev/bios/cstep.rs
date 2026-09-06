//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/subdev/bios/cstep.h
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
pub struct nvbios_cstepE {
    pub pstate: u8,
    pub index: u8,
}

extern "C" {
    pub fn nvbios_cstepEe(: *mut nvkm_bios, idx: c_int, ver: *mut u8, hdr: *mut u8) -> u32;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvbios_cstepX {
    pub freq: u32,
    pub unkn: [u8; 2],
    pub voltage: u8,
}

extern "C" {
    pub fn nvbios_cstepXe(: *mut nvkm_bios, idx: c_int, ver: *mut u8, hdr: *mut u8) -> u32;
}
