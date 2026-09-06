//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvif/cl0002.h
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
pub struct nv_dma_v0 {
    pub version: __u8,
pub const NV_DMA_V0_TARGET_VM: c_uint = 0x00;
pub const NV_DMA_V0_TARGET_VRAM: c_uint = 0x01;
pub const NV_DMA_V0_TARGET_PCI: c_uint = 0x02;
pub const NV_DMA_V0_TARGET_PCI_US: c_uint = 0x03;
pub const NV_DMA_V0_TARGET_AGP: c_uint = 0x04;
    pub target: __u8,
pub const NV_DMA_V0_ACCESS_VM: c_uint = 0x00;
pub const NV_DMA_V0_ACCESS_RD: c_uint = 0x01;
pub const NV_DMA_V0_ACCESS_WR: c_uint = 0x02;

    pub access: __u8,
    pub pad03: [__u8; 5],
    pub start: __u64,
    pub limit: __u64,
// ... chipset-specific class data
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv50_dma_v0 {
    pub version: __u8,
pub const NV50_DMA_V0_PRIV_VM: c_uint = 0x00;
pub const NV50_DMA_V0_PRIV_US: c_uint = 0x01;
pub const NV50_DMA_V0_PRIV__S: c_uint = 0x02;
    pub priv: __u8,
pub const NV50_DMA_V0_PART_VM: c_uint = 0x00;
pub const NV50_DMA_V0_PART_256: c_uint = 0x01;
pub const NV50_DMA_V0_PART_1KB: c_uint = 0x02;
    pub part: __u8,
pub const NV50_DMA_V0_COMP_NONE: c_uint = 0x00;
pub const NV50_DMA_V0_COMP_1: c_uint = 0x01;
pub const NV50_DMA_V0_COMP_2: c_uint = 0x02;
pub const NV50_DMA_V0_COMP_VM: c_uint = 0x03;
    pub comp: __u8,
pub const NV50_DMA_V0_KIND_PITCH: c_uint = 0x00;
pub const NV50_DMA_V0_KIND_VM: c_uint = 0x7f;
    pub kind: __u8,
    pub pad05: [__u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gf100_dma_v0 {
    pub version: __u8,
pub const GF100_DMA_V0_PRIV_VM: c_uint = 0x00;
pub const GF100_DMA_V0_PRIV_US: c_uint = 0x01;
pub const GF100_DMA_V0_PRIV__S: c_uint = 0x02;
    pub priv: __u8,
pub const GF100_DMA_V0_KIND_PITCH: c_uint = 0x00;
pub const GF100_DMA_V0_KIND_VM: c_uint = 0xff;
    pub kind: __u8,
    pub pad03: [__u8; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gf119_dma_v0 {
    pub version: __u8,
pub const GF119_DMA_V0_PAGE_LP: c_uint = 0x00;
pub const GF119_DMA_V0_PAGE_SP: c_uint = 0x01;
    pub page: __u8,
pub const GF119_DMA_V0_KIND_PITCH: c_uint = 0x00;
pub const GF119_DMA_V0_KIND_VM: c_uint = 0xff;
    pub kind: __u8,
    pub pad03: [__u8; 5],
}
