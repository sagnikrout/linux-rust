//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvif/ioctl.h
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
pub struct nvif_ioctl_v0 {
// New members MUST be added within the struct_group() macro below.
    pub version: __u8,
pub const NVIF_IOCTL_V0_SCLASS: c_uint = 0x01;
pub const NVIF_IOCTL_V0_NEW: c_uint = 0x02;
pub const NVIF_IOCTL_V0_DEL: c_uint = 0x03;
pub const NVIF_IOCTL_V0_MTHD: c_uint = 0x04;
pub const NVIF_IOCTL_V0_MAP: c_uint = 0x07;
pub const NVIF_IOCTL_V0_UNMAP: c_uint = 0x08;
    pub type: __u8,
    pub pad02: [__u8; 4],
pub const NVIF_IOCTL_V0_OWNER_NVIF: c_uint = 0x00;
pub const NVIF_IOCTL_V0_OWNER_ANY: c_uint = 0xff;
    pub owner: __u8,
pub const NVIF_IOCTL_V0_ROUTE_NVIF: c_uint = 0x00;
pub const NVIF_IOCTL_V0_ROUTE_HIDDEN: c_uint = 0xff;
    pub route: __u8,
    pub token: __u64,
    pub object: __u64,
    pub /: *mut *mut __u8 data[]; / ioctl data (below),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_ioctl_sclass_v0 {
// nvif_ioctl ...
    pub version: __u8,
    pub count: __u8,
    pub pad02: [__u8; 6],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_ioctl_sclass_oclass_v0 {
    pub oclass: __s32,
    pub minver: __s16,
    pub maxver: __s16,
    pub oclass: [}; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_ioctl_new_v0 {
// nvif_ioctl ...
    pub version: __u8,
    pub pad01: [__u8; 6],
    pub route: __u8,
    pub token: __u64,
    pub object: __u64,
    pub handle: __u32,
    pub oclass: __s32,
    pub /: *mut *mut __u8 data[]; / class data (class.h),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_ioctl_del {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_ioctl_mthd_v0 {
// New members MUST be added within the struct_group() macro below.
// nvif_ioctl ...
    pub version: __u8,
    pub method: __u8,
    pub pad02: [__u8; 6],
    pub /: *mut *mut __u8 data[]; / method data (class.h),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_ioctl_map_v0 {
// nvif_ioctl ...
    pub version: __u8,
pub const NVIF_IOCTL_MAP_V0_IO: c_uint = 0x00;
pub const NVIF_IOCTL_MAP_V0_VA: c_uint = 0x01;
    pub type: __u8,
    pub pad02: [__u8; 6],
    pub handle: __u64,
    pub length: __u64,
    pub data: [__u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_ioctl_unmap {
}
