//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/subdev/bios/i2c.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dcb_i2c_type {
// matches bios type field prior to ccb 4.1
    DCB_I2C_NV04_BIT = 0x00,
    DCB_I2C_NV4E_BIT = 0x04,
    DCB_I2C_NVIO_BIT = 0x05,
    DCB_I2C_NVIO_AUX = 0x06,
// made up - mostly
    DCB_I2C_PMGR     = 0x80,
    DCB_I2C_UNUSED   = 0xff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcb_i2c_entry {
    pub type: dcb_i2c_type,
    pub drive: u8,
    pub sense: u8,
    pub share: u8,
    pub auxch: u8,
}

extern "C" {
    pub fn dcb_i2c_table(: *mut nvkm_bios, ver: *mut u8, hdr: *mut u8, cnt: *mut u8, len: *mut u8) -> u16;
}
extern "C" {
    pub fn dcb_i2c_entry(: *mut nvkm_bios, index: u8, ver: *mut u8, len: *mut u8) -> u16;
}
extern "C" {
    pub fn dcb_i2c_parse(: *mut nvkm_bios, index: u8, : *mut dcb_i2c_entry) -> c_int;
}
