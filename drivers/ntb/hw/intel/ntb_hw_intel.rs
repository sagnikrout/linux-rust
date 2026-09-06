//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/ntb/hw/intel/ntb_hw_intel.h
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
// Copyright(c) 2012 Intel Corporation. All rights reserved.
// Copyright (C) 2015 EMC Corporation. All Rights Reserved.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of version 2 of the GNU General Public License as
// published by the Free Software Foundation.
//
// BSD LICENSE
//
// Copyright(c) 2012 Intel Corporation. All rights reserved.
// Copyright (C) 2015 EMC Corporation. All Rights Reserved.
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
// * Neither the name of Intel Corporation nor the names of its
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
// Intel PCIe NTB Linux driver
//

// PCI device IDs
pub const PCI_DEVICE_ID_INTEL_NTB_B2B_JSF: c_uint = 0x3725;
pub const PCI_DEVICE_ID_INTEL_NTB_PS_JSF: c_uint = 0x3726;
pub const PCI_DEVICE_ID_INTEL_NTB_SS_JSF: c_uint = 0x3727;
pub const PCI_DEVICE_ID_INTEL_NTB_B2B_SNB: c_uint = 0x3C0D;
pub const PCI_DEVICE_ID_INTEL_NTB_PS_SNB: c_uint = 0x3C0E;
pub const PCI_DEVICE_ID_INTEL_NTB_SS_SNB: c_uint = 0x3C0F;
pub const PCI_DEVICE_ID_INTEL_NTB_B2B_IVT: c_uint = 0x0E0D;
pub const PCI_DEVICE_ID_INTEL_NTB_PS_IVT: c_uint = 0x0E0E;
pub const PCI_DEVICE_ID_INTEL_NTB_SS_IVT: c_uint = 0x0E0F;
pub const PCI_DEVICE_ID_INTEL_NTB_B2B_HSX: c_uint = 0x2F0D;
pub const PCI_DEVICE_ID_INTEL_NTB_PS_HSX: c_uint = 0x2F0E;
pub const PCI_DEVICE_ID_INTEL_NTB_SS_HSX: c_uint = 0x2F0F;
pub const PCI_DEVICE_ID_INTEL_NTB_B2B_BDX: c_uint = 0x6F0D;
pub const PCI_DEVICE_ID_INTEL_NTB_PS_BDX: c_uint = 0x6F0E;
pub const PCI_DEVICE_ID_INTEL_NTB_SS_BDX: c_uint = 0x6F0F;
pub const PCI_DEVICE_ID_INTEL_NTB_B2B_SKX: c_uint = 0x201C;
pub const PCI_DEVICE_ID_INTEL_NTB_B2B_ICX: c_uint = 0x347e;
pub const PCI_DEVICE_ID_INTEL_NTB_B2B_GNR: c_uint = 0x0db4;
pub const PCI_DEVICE_ID_INTEL_NTB_B2B_DMR: c_uint = 0x7868;
// Ntb control and link status

pub const NTB_LNK_STA_ACTIVE_BIT: c_uint = 0x2000;
pub const NTB_LNK_STA_SPEED_MASK: c_uint = 0x000f;
pub const NTB_LNK_STA_WIDTH_MASK: c_uint = 0x03f0;

// flags to indicate unsafe api

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_ntb_reg {
    pub ndev): *mut *mut int (poll_link)(struct intel_ntb_dev,
    pub ndev): *mut *mut int (link_is_up)(struct intel_ntb_dev,
    pub mmio): *const *const u64 (db_ioread)(void __iomem,
    pub mmio): *mut *mut void (db_iowrite)(u64 db_bits, void __iomem,
    pub ntb_ctl: c_ulong,
    pub db_size: resource_size_t,
    pub mw_bar: [c_int; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_ntb_alt_reg {
    pub db_bell: c_ulong,
    pub db_mask: c_ulong,
    pub db_clear: c_ulong,
    pub spad: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_ntb_xlat_reg {
    pub bar0_base: c_ulong,
    pub bar2_xlat: c_ulong,
    pub bar2_limit: c_ulong,
    pub bar2_idx: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_b2b_addr {
    pub bar0_addr: phys_addr_t,
    pub bar2_addr64: phys_addr_t,
    pub bar4_addr64: phys_addr_t,
    pub bar4_addr32: phys_addr_t,
    pub bar5_addr32: phys_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_ntb_vec {
    pub ndev: *mut intel_ntb_dev,
    pub num: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_ntb_dev {
    pub ntb: ntb_dev,
// offset of peer bar0 in b2b bar
    pub b2b_off: c_ulong,
// mw idx used to access peer bar0
    pub b2b_idx: c_uint,
// BAR45 is split into BAR4 and BAR5
    pub bar4_split: bool,
    pub ntb_ctl: u32,
    pub lnk_sta: u32,
    pub mw_count: c_uchar,
    pub spad_count: c_uchar,
    pub db_count: c_uchar,
    pub db_vec_count: c_uchar,
    pub db_vec_shift: c_uchar,
    pub db_valid_mask: u64,
    pub db_link_mask: u64,
    pub db_mask: u64,
// synchronize rmw access of db_mask and hw reg
    pub db_mask_lock: spinlock_t,
    pub msix: *mut msix_entry,
    pub vec: *mut intel_ntb_vec,
    pub reg: *const intel_ntb_reg,
    pub self_reg: *const intel_ntb_alt_reg,
    pub peer_reg: *const intel_ntb_alt_reg,
    pub xlat_reg: *const intel_ntb_xlat_reg,
    pub self_mmio: *mut void __iomem,
    pub peer_mmio: *mut void __iomem,
    pub peer_addr: phys_addr_t,
    pub last_ts: c_ulong,
    pub hb_timer: delayed_work,
    pub hwerr_flags: c_ulong,
    pub unsafe_flags: c_ulong,
    pub unsafe_flags_ignore: c_ulong,
    pub debugfs_dir: *mut dentry,
    pub debugfs_info: *mut dentry,
// gen4 entries
    pub dev_up: c_int,
}

