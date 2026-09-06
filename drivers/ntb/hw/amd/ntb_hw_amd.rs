//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/ntb/hw/amd/ntb_hw_amd.h
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


//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// GPL LICENSE SUMMARY
//
// Copyright (C) 2016 Advanced Micro Devices, Inc. All Rights Reserved.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of version 2 of the GNU General Public License as
// published by the Free Software Foundation.
//
// BSD LICENSE
//
// Copyright (C) 2016 Advanced Micro Devices, Inc. All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
// * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// * Redistributions in binary form must reproduce the above copy
// notice, this list of conditions and the following disclaimer in
// the documentation and/or other materials provided with the
// distribution.
// * Neither the name of AMD Corporation nor the names of its
// contributors may be used to endorse or promote products derived
// from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// AMD PCIe NTB Linux driver
//
// Contact Information:
// Xiangliang Yu <Xiangliang.Yu@amd.com>
//

pub const NTB_LNK_STA_SPEED_MASK: c_uint = 0x000F0000;
pub const NTB_LNK_STA_WIDTH_MASK: c_uint = 0x03F00000;

// AMD NTB Capability
// AMD NTB register offset
// NTB control register bits
// limit register
// xlat address
// doorbell and interrupt
// event type
// SMU register
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntb_dev_data {
    pub mw_count: c_uchar,
    pub mw_idx: c_uint,
    pub is_endpoint: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_ntb_vec {
    pub ndev: *mut amd_ntb_dev,
    pub num: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_ntb_dev {
    pub ntb: ntb_dev,
    pub ntb_side: u32,
    pub lnk_sta: u32,
    pub cntl_sta: u32,
    pub peer_sta: u32,
    pub dev_data: *mut ntb_dev_data,
    pub mw_count: c_uchar,
    pub spad_count: c_uchar,
    pub db_count: c_uchar,
    pub msix_vec_count: c_uchar,
    pub db_valid_mask: u64,
    pub db_mask: u64,
    pub db_last_bit: u64,
    pub int_mask: u32,
    pub msix: *mut msix_entry,
    pub vec: *mut amd_ntb_vec,
// synchronize rmw access of db_mask and hw reg
    pub db_mask_lock: spinlock_t,
    pub self_mmio: *mut void __iomem,
    pub peer_mmio: *mut void __iomem,
    pub self_spad: c_uint,
    pub peer_spad: c_uint,
    pub hb_timer: delayed_work,
    pub debugfs_dir: *mut dentry,
    pub debugfs_info: *mut dentry,
}

extern "C" {
    pub fn amd_set_side_info_reg(ndev: *mut amd_ntb_dev, peer: bool) -> static void;
}
extern "C" {
    pub fn amd_clear_side_info_reg(ndev: *mut amd_ntb_dev, peer: bool) -> static void;
}
extern "C" {
    pub fn amd_poll_link(ndev: *mut amd_ntb_dev) -> static int;
}
