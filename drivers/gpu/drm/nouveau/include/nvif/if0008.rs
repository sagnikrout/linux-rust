//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvif/if0008.h
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
pub struct nvif_mmu_v0 {
    pub version: __u8,
    pub dmabits: __u8,
    pub heap_nr: __u8,
    pub type_nr: __u8,
    pub kind_nr: __u16,
}

pub const NVIF_MMU_V0_HEAP: c_uint = 0x00;
pub const NVIF_MMU_V0_TYPE: c_uint = 0x01;
pub const NVIF_MMU_V0_KIND: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_mmu_heap_v0 {
    pub version: __u8,
    pub index: __u8,
    pub pad02: [__u8; 6],
    pub size: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_mmu_type_v0 {
    pub version: __u8,
    pub index: __u8,
    pub heap: __u8,
    pub vram: __u8,
    pub host: __u8,
    pub comp: __u8,
    pub disp: __u8,
    pub kind: __u8,
    pub mappable: __u8,
    pub coherent: __u8,
    pub uncached: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_mmu_kind_v0 {
    pub version: __u8,
    pub kind_inv: __u8,
    pub count: __u16,
    pub data: [__u8; ],
}
