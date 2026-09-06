//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvfw/ls.h
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
pub struct nvfw_ls_desc_head {
    pub descriptor_size: u32,
    pub image_size: u32,
    pub tools_version: u32,
    pub app_version: u32,
    pub date: [c_char; 64],
    pub bootloader_start_offset: u32,
    pub bootloader_size: u32,
    pub bootloader_imem_offset: u32,
    pub bootloader_entry_point: u32,
    pub app_start_offset: u32,
    pub app_size: u32,
    pub app_imem_offset: u32,
    pub app_imem_entry: u32,
    pub app_dmem_offset: u32,
    pub app_resident_code_offset: u32,
    pub app_resident_code_size: u32,
    pub app_resident_data_offset: u32,
    pub app_resident_data_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvfw_ls_desc {
    pub head: nvfw_ls_desc_head,
    pub nb_overlays: u32,
    pub start: u32,
    pub size: u32,
    pub load_ovl: [}; 64],
    pub compressed: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvfw_ls_desc_v1 {
    pub head: nvfw_ls_desc_head,
    pub nb_imem_overlays: u32,
    pub nb_dmem_overlays: u32,
    pub start: u32,
    pub size: u32,
    pub load_ovl: [}; 64],
    pub compressed: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvfw_ls_desc_v2 {
    pub descriptor_size: u32,
    pub image_size: u32,
    pub tools_version: u32,
    pub app_version: u32,
    pub date: [c_char; 64],
    pub secure_bootloader: u32,
    pub bootloader_start_offset: u32,
    pub bootloader_size: u32,
    pub bootloader_imem_offset: u32,
    pub bootloader_entry_point: u32,
    pub app_start_offset: u32,
    pub app_size: u32,
    pub app_imem_offset: u32,
    pub app_imem_entry: u32,
    pub app_dmem_offset: u32,
    pub app_resident_code_offset: u32,
    pub app_resident_code_size: u32,
    pub app_resident_data_offset: u32,
    pub app_resident_data_size: u32,
    pub nb_imem_overlays: u32,
    pub nb_dmem_overlays: u32,
    pub start: u32,
    pub size: u32,
    pub load_ovl: [}; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvfw_ls_hsbl_bin_hdr {
    pub bin_magic: u32,
    pub bin_ver: u32,
    pub bin_size: u32,
    pub header_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvfw_ls_hsbl_hdr {
    pub sig_prod_offset: u32,
    pub sig_prod_size: u32,
    pub patch_loc: u32,
    pub patch_sig: u32,
    pub meta_data_offset: u32,
    pub meta_data_size: u32,
    pub num_sig: u32,
}
