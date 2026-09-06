//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/cavium/cpt/cptpf.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2016 Cavium, Inc.
//

pub const CSR_DELAY: c_int = 30;
pub const CPT_MAX_CORE_GROUPS: c_int = 8;
pub const CPT_MAX_SE_CORES: c_int = 10;
pub const CPT_MAX_AE_CORES: c_int = 6;

pub const CPT_MAX_VF_NUM: c_int = 16;
pub const CPT_PF_MSIX_VECTORS: c_int = 3;

pub const CPT_UCODE_VERSION_SZ: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct microcode {
    pub is_mc_valid: u8,
    pub is_ae: u8,
    pub group: u8,
    pub num_cores: u8,
    pub code_size: u32,
    pub core_mask: u64,
    pub version: [u8; CPT_UCODE_VERSION_SZ],
// Base info
    pub phys_base: dma_addr_t,
    pub code: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpt_vf_info {
    pub state: u8,
    pub priority: u8,
    pub id: u8,
    pub qlen: u32,
}

//
// cpt device structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpt_device {
    pub /: *mut *mut u16 flags; / Flags to hold device status bits,
    pub /: *mut *mut u8 num_vf_en; / Number of VFs enabled (0...CPT_MAX_VF_NUM),
    pub /: *mut *mut cpt_vf_info vfinfo[CPT_MAX_VF_NUM]; / Per VF info,
    pub /: *mut *mut *mut void __iomem reg_base; / Register start address,
    pub /: *mut *mut *mut pci_dev pdev; / pci device handle,
    pub mcode: [microcode; CPT_MAX_CORE_GROUPS],
    pub /: *mut *mut u8 next_mc_idx; / next microcode index,
    pub next_group: u8,
    pub max_se_cores: u8,
    pub max_ae_cores: u8,
}

extern "C" {
    pub fn cpt_mbox_intr_handler(cpt: *mut cpt_device, mbx: c_int);
}
