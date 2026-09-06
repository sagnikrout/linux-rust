//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/subdev/bios/extdev.h
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
pub enum nvbios_extdev_type {
    NVBIOS_EXTDEV_LM89		= 0x02,
    NVBIOS_EXTDEV_VT1103M		= 0x40,
    NVBIOS_EXTDEV_PX3540		= 0x41,
    NVBIOS_EXTDEV_VT1105M		= 0x42, /* or close enough... */
    NVBIOS_EXTDEV_INA219		= 0x4c,
    NVBIOS_EXTDEV_INA209		= 0x4d,
    NVBIOS_EXTDEV_INA3221		= 0x4e,
    NVBIOS_EXTDEV_ADT7473		= 0x70, /* can also be a LM64 */
    NVBIOS_EXTDEV_HDCP_EEPROM	= 0x90,
    NVBIOS_EXTDEV_NONE		= 0xff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvbios_extdev_func {
    pub type: u8,
    pub addr: u8,
    pub bus: u8,
}

extern "C" {
    pub fn nvbios_extdev_skip_probe(: *mut nvkm_bios) -> bool;
}
