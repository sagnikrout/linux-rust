//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvif/if000c.h
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


#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_vmm_v0 {
    pub version: __u8,
    pub page_nr: __u8,
pub const NVIF_VMM_V0_TYPE_UNMANAGED: c_uint = 0x00;
pub const NVIF_VMM_V0_TYPE_MANAGED: c_uint = 0x01;
pub const NVIF_VMM_V0_TYPE_RAW: c_uint = 0x02;
    pub type: __u8,
    pub pad03: [__u8; 5],
    pub addr: __u64,
    pub size: __u64,
    pub data: [__u8; ],
}

pub const NVIF_VMM_V0_PAGE: c_uint = 0x00;
pub const NVIF_VMM_V0_GET: c_uint = 0x01;
pub const NVIF_VMM_V0_PUT: c_uint = 0x02;
pub const NVIF_VMM_V0_MAP: c_uint = 0x03;
pub const NVIF_VMM_V0_UNMAP: c_uint = 0x04;
pub const NVIF_VMM_V0_PFNMAP: c_uint = 0x05;
pub const NVIF_VMM_V0_PFNCLR: c_uint = 0x06;
pub const NVIF_VMM_V0_RAW: c_uint = 0x07;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_vmm_page_v0 {
    pub version: __u8,
    pub index: __u8,
    pub shift: __u8,
    pub sparse: __u8,
    pub vram: __u8,
    pub host: __u8,
    pub comp: __u8,
    pub pad07: [__u8; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_vmm_get_v0 {
    pub version: __u8,
pub const NVIF_VMM_GET_V0_ADDR: c_uint = 0x00;
pub const NVIF_VMM_GET_V0_PTES: c_uint = 0x01;
pub const NVIF_VMM_GET_V0_LAZY: c_uint = 0x02;
    pub type: __u8,
    pub sparse: __u8,
    pub page: __u8,
    pub align: __u8,
    pub pad05: [__u8; 3],
    pub size: __u64,
    pub addr: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_vmm_put_v0 {
    pub version: __u8,
    pub pad01: [__u8; 7],
    pub addr: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_vmm_map_v0 {
    pub version: __u8,
    pub pad01: [__u8; 7],
    pub addr: __u64,
    pub size: __u64,
    pub memory: __u64,
    pub offset: __u64,
    pub data: [__u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_vmm_unmap_v0 {
    pub version: __u8,
    pub pad01: [__u8; 7],
    pub addr: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_vmm_raw_v0 {
    pub version: __u8,
pub const NVIF_VMM_RAW_V0_GET: c_uint = 0x0;
pub const NVIF_VMM_RAW_V0_PUT: c_uint = 0x1;
pub const NVIF_VMM_RAW_V0_MAP: c_uint = 0x2;
pub const NVIF_VMM_RAW_V0_UNMAP: c_uint = 0x3;
pub const NVIF_VMM_RAW_V0_SPARSE: c_uint = 0x4;
    pub op: __u8,
    pub sparse: __u8,
    pub ref: __u8,
    pub shift: __u8,
    pub argc: __u32,
    pub pad01: [__u8; 7],
    pub addr: __u64,
    pub size: __u64,
    pub offset: __u64,
    pub memory: __u64,
    pub argv: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_vmm_pfnmap_v0 {
    pub version: __u8,
    pub page: __u8,
    pub pad02: [__u8; 6],
    pub addr: __u64,
    pub size: __u64,
pub const NVIF_VMM_PFNMAP_V0_ADDR: c_uint = 0xfffffffffffff000ULL;
pub const NVIF_VMM_PFNMAP_V0_ADDR_SHIFT: c_int = 12;
pub const NVIF_VMM_PFNMAP_V0_APER: c_uint = 0x00000000000000f0ULL;
pub const NVIF_VMM_PFNMAP_V0_HOST: c_uint = 0x0000000000000000ULL;
pub const NVIF_VMM_PFNMAP_V0_VRAM: c_uint = 0x0000000000000010ULL;
pub const NVIF_VMM_PFNMAP_V0_A: c_uint = 0x0000000000000004ULL;
pub const NVIF_VMM_PFNMAP_V0_W: c_uint = 0x0000000000000002ULL;
pub const NVIF_VMM_PFNMAP_V0_V: c_uint = 0x0000000000000001ULL;
pub const NVIF_VMM_PFNMAP_V0_NONE: c_uint = 0x0000000000000000ULL;
    pub phys: [__u64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_vmm_pfnclr_v0 {
    pub version: __u8,
    pub pad01: [__u8; 7],
    pub addr: __u64,
    pub size: __u64,
}
