//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/subdev/bios/pmu.h
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
pub struct nvbios_pmuT {
}

extern "C" {
    pub fn nvbios_pmuTe(: *mut nvkm_bios, ver: *mut u8, hdr: *mut u8, cnt: *mut u8, len: *mut u8) -> u32;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvbios_pmuE {
    pub type: u8,
    pub data: u32,
}

extern "C" {
    pub fn nvbios_pmuEe(: *mut nvkm_bios, idx: c_int, ver: *mut u8, hdr: *mut u8) -> u32;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvbios_pmuR {
    pub boot_addr_pmu: u32,
    pub boot_addr: u32,
    pub boot_size: u32,
    pub code_addr_pmu: u32,
    pub code_addr: u32,
    pub code_size: u32,
    pub init_addr_pmu: u32,
    pub data_addr_pmu: u32,
    pub data_addr: u32,
    pub data_size: u32,
    pub args_addr_pmu: u32,
}

extern "C" {
    pub fn nvbios_pmuRm(: *mut nvkm_bios, type: u8, : *mut nvbios_pmuR) -> bool;
}
