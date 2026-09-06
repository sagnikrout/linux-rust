//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvif/cl0080.h
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
pub const NV_DEVICE_V0_INFO: c_uint = 0x00;
pub const NV_DEVICE_V0_TIME: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv_device_info_v0 {
    pub version: __u8,
pub const NV_DEVICE_INFO_V0_IGP: c_uint = 0x00;
pub const NV_DEVICE_INFO_V0_PCI: c_uint = 0x01;
pub const NV_DEVICE_INFO_V0_AGP: c_uint = 0x02;
pub const NV_DEVICE_INFO_V0_PCIE: c_uint = 0x03;
pub const NV_DEVICE_INFO_V0_SOC: c_uint = 0x04;
    pub platform: __u8,
    pub /: *mut *mut __u16 chipset; / from NV_PMC_BOOT_0,
    pub /: *mut *mut __u8 revision; / from NV_PMC_BOOT_0,
pub const NV_DEVICE_INFO_V0_TNT: c_uint = 0x01;
pub const NV_DEVICE_INFO_V0_CELSIUS: c_uint = 0x02;
pub const NV_DEVICE_INFO_V0_KELVIN: c_uint = 0x03;
pub const NV_DEVICE_INFO_V0_RANKINE: c_uint = 0x04;
pub const NV_DEVICE_INFO_V0_CURIE: c_uint = 0x05;
pub const NV_DEVICE_INFO_V0_TESLA: c_uint = 0x06;
pub const NV_DEVICE_INFO_V0_FERMI: c_uint = 0x07;
pub const NV_DEVICE_INFO_V0_KEPLER: c_uint = 0x08;
pub const NV_DEVICE_INFO_V0_MAXWELL: c_uint = 0x09;
pub const NV_DEVICE_INFO_V0_PASCAL: c_uint = 0x0a;
pub const NV_DEVICE_INFO_V0_VOLTA: c_uint = 0x0b;
pub const NV_DEVICE_INFO_V0_TURING: c_uint = 0x0c;
pub const NV_DEVICE_INFO_V0_AMPERE: c_uint = 0x0d;
pub const NV_DEVICE_INFO_V0_ADA: c_uint = 0x0e;
pub const NV_DEVICE_INFO_V0_HOPPER: c_uint = 0x0f;
pub const NV_DEVICE_INFO_V0_BLACKWELL: c_uint = 0x10;
    pub family: __u8,
    pub pad06: [__u8; 2],
    pub ram_size: __u64,
    pub ram_user: __u64,
    pub chip: [c_char; 16],
    pub name: [c_char; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv_device_info_v1 {
    pub version: __u8,
    pub count: __u8,
    pub pad02: [__u8; 6],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv_device_info_v1_data {
    pub /: *mut *mut *mut __u64 mthd; / NV_DEVICE_INFO_ (see below).,
    pub data: __u64,
    pub data: [}; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv_device_time_v0 {
    pub version: __u8,
    pub pad01: [__u8; 7],
    pub time: __u64,
}

// This will be returned in the mthd field for unsupported queries.

// Returns the number of available runlists.

// Returns the number of available channels (0 if per-runlist).

// Returns a mask of available engine types on runlist(data).

pub const NV_DEVICE_HOST_RUNLIST_ENGINES_SW: c_uint = 0x00000001;
pub const NV_DEVICE_HOST_RUNLIST_ENGINES_GR: c_uint = 0x00000002;
pub const NV_DEVICE_HOST_RUNLIST_ENGINES_MPEG: c_uint = 0x00000004;
pub const NV_DEVICE_HOST_RUNLIST_ENGINES_ME: c_uint = 0x00000008;
pub const NV_DEVICE_HOST_RUNLIST_ENGINES_CIPHER: c_uint = 0x00000010;
pub const NV_DEVICE_HOST_RUNLIST_ENGINES_BSP: c_uint = 0x00000020;
pub const NV_DEVICE_HOST_RUNLIST_ENGINES_VP: c_uint = 0x00000040;
pub const NV_DEVICE_HOST_RUNLIST_ENGINES_CE: c_uint = 0x00000080;
pub const NV_DEVICE_HOST_RUNLIST_ENGINES_SEC: c_uint = 0x00000100;
pub const NV_DEVICE_HOST_RUNLIST_ENGINES_MSVLD: c_uint = 0x00000200;
pub const NV_DEVICE_HOST_RUNLIST_ENGINES_MSPDEC: c_uint = 0x00000400;
pub const NV_DEVICE_HOST_RUNLIST_ENGINES_MSPPP: c_uint = 0x00000800;
pub const NV_DEVICE_HOST_RUNLIST_ENGINES_MSENC: c_uint = 0x00001000;
pub const NV_DEVICE_HOST_RUNLIST_ENGINES_VIC: c_uint = 0x00002000;
pub const NV_DEVICE_HOST_RUNLIST_ENGINES_SEC2: c_uint = 0x00004000;
pub const NV_DEVICE_HOST_RUNLIST_ENGINES_NVDEC: c_uint = 0x00008000;
pub const NV_DEVICE_HOST_RUNLIST_ENGINES_NVENC: c_uint = 0x00010000;
pub const NV_DEVICE_HOST_RUNLIST_ENGINES_NVJPG: c_uint = 0x00020000;
pub const NV_DEVICE_HOST_RUNLIST_ENGINES_OFA: c_uint = 0x00040000;
// Returns the number of available channels on runlist(data).

