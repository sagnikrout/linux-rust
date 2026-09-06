//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/marvell/octeontx/otx_cptpf.h
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


// SPDX-License-Identifier: GPL-2.0
// Marvell OcteonTX CPT driver
//
// Copyright (C) 2019 Marvell International Ltd.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//

//
// OcteonTX CPT device structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cpt_device {
    pub /: *mut *mut *mut void __iomem reg_base; / Register start address,
    pub /: *mut *mut *mut pci_dev pdev; / Pci device handle,
    pub /: *mut *mut otx_cpt_eng_grps eng_grps;/ Engine groups information,
    pub list: list_head,
    pub /: *mut *mut u8 pf_type; / PF type SE or AE,
    pub /: *mut *mut u8 max_vfs; / Maximum number of VFs supported by the CPT,
    pub /: *mut *mut u8 vfs_enabled; / Number of enabled VFs,
}

extern "C" {
    pub fn otx_cpt_mbox_intr_handler(cpt: *mut otx_cpt_device, mbx: c_int);
}
extern "C" {
    pub fn otx_cpt_disable_all_cores(cpt: *mut otx_cpt_device);
}
