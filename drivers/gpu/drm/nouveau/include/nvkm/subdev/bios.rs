//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/subdev/bios.h
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
pub struct nvkm_bios {
    pub subdev: nvkm_subdev,
    pub size: u32,
    pub data: *mut u8,
    pub image0_size: u32,
    pub imaged_addr: u32,
    pub bmp_offset: u32,
    pub bit_offset: u32,
    pub major: u8,
    pub chip: u8,
    pub minor: u8,
    pub micro: u8,
    pub patch: u8,
    pub version: },
}

extern "C" {
    pub fn nvbios_checksum(data: *const u8, size: c_int) -> u8;
}
extern "C" {
    pub fn nvbios_findstr(data: *const u8, size: c_int, str: *const c_char, len: c_int) -> u16;
}
extern "C" {
    pub fn nvbios_memcmp(: *mut nvkm_bios, addr: u32, : *const c_char, len: u32) -> c_int;
}
extern "C" {
    pub fn nvbios_rd08(: *mut nvkm_bios, addr: u32) -> u8;
}
extern "C" {
    pub fn nvbios_rd16(: *mut nvkm_bios, addr: u32) -> u16;
}
extern "C" {
    pub fn nvbios_rd32(: *mut nvkm_bios, addr: u32) -> u32;
}
extern "C" {
    pub fn nvkm_bios_new(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, : *mut nvkm_bios) -> c_int;
}
