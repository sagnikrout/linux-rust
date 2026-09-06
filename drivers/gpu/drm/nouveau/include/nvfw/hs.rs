//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvfw/hs.h
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
pub struct nvfw_hs_header {
    pub sig_dbg_offset: u32,
    pub sig_dbg_size: u32,
    pub sig_prod_offset: u32,
    pub sig_prod_size: u32,
    pub patch_loc: u32,
    pub patch_sig: u32,
    pub hdr_offset: u32,
    pub hdr_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvfw_hs_header_v2 {
    pub sig_prod_offset: u32,
    pub sig_prod_size: u32,
    pub patch_loc: u32,
    pub patch_sig: u32,
    pub meta_data_offset: u32,
    pub meta_data_size: u32,
    pub num_sig: u32,
    pub header_offset: u32,
    pub header_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvfw_hs_load_header {
    pub non_sec_code_off: u32,
    pub non_sec_code_size: u32,
    pub data_dma_base: u32,
    pub data_size: u32,
    pub num_apps: u32,
    pub apps: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvfw_hs_load_header_v2 {
    pub os_code_offset: u32,
    pub os_code_size: u32,
    pub os_data_offset: u32,
    pub os_data_size: u32,
    pub num_apps: u32,
    pub offset: u32,
    pub size: u32,
    pub data_offset: u32,
    pub data_size: u32,
    pub __counted_by(num_apps): } app[],
}
