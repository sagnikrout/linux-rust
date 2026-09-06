//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/ntb/hw/intel/ntb_hw_gen1.h
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
// Copyright(c) 2012-2017 Intel Corporation. All rights reserved.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of version 2 of the GNU General Public License as
// published by the Free Software Foundation.
//
// BSD LICENSE
//
// Copyright(c) 2012-2017 Intel Corporation. All rights reserved.
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

// Intel Gen1 Xeon hardware
pub const XEON_PBAR23LMT_OFFSET: c_uint = 0x0000;
pub const XEON_PBAR45LMT_OFFSET: c_uint = 0x0008;
pub const XEON_PBAR4LMT_OFFSET: c_uint = 0x0008;
pub const XEON_PBAR5LMT_OFFSET: c_uint = 0x000c;
pub const XEON_PBAR23XLAT_OFFSET: c_uint = 0x0010;
pub const XEON_PBAR45XLAT_OFFSET: c_uint = 0x0018;
pub const XEON_PBAR4XLAT_OFFSET: c_uint = 0x0018;
pub const XEON_PBAR5XLAT_OFFSET: c_uint = 0x001c;
pub const XEON_SBAR23LMT_OFFSET: c_uint = 0x0020;
pub const XEON_SBAR45LMT_OFFSET: c_uint = 0x0028;
pub const XEON_SBAR4LMT_OFFSET: c_uint = 0x0028;
pub const XEON_SBAR5LMT_OFFSET: c_uint = 0x002c;
pub const XEON_SBAR23XLAT_OFFSET: c_uint = 0x0030;
pub const XEON_SBAR45XLAT_OFFSET: c_uint = 0x0038;
pub const XEON_SBAR4XLAT_OFFSET: c_uint = 0x0038;
pub const XEON_SBAR5XLAT_OFFSET: c_uint = 0x003c;
pub const XEON_SBAR0BASE_OFFSET: c_uint = 0x0040;
pub const XEON_SBAR23BASE_OFFSET: c_uint = 0x0048;
pub const XEON_SBAR45BASE_OFFSET: c_uint = 0x0050;
pub const XEON_SBAR4BASE_OFFSET: c_uint = 0x0050;
pub const XEON_SBAR5BASE_OFFSET: c_uint = 0x0054;
pub const XEON_SBDF_OFFSET: c_uint = 0x005c;
pub const XEON_NTBCNTL_OFFSET: c_uint = 0x0058;
pub const XEON_PDOORBELL_OFFSET: c_uint = 0x0060;
pub const XEON_PDBMSK_OFFSET: c_uint = 0x0062;
pub const XEON_SDOORBELL_OFFSET: c_uint = 0x0064;
pub const XEON_SDBMSK_OFFSET: c_uint = 0x0066;
pub const XEON_USMEMMISS_OFFSET: c_uint = 0x0070;
pub const XEON_SPAD_OFFSET: c_uint = 0x0080;
pub const XEON_PBAR23SZ_OFFSET: c_uint = 0x00d0;
pub const XEON_PBAR45SZ_OFFSET: c_uint = 0x00d1;
pub const XEON_PBAR4SZ_OFFSET: c_uint = 0x00d1;
pub const XEON_SBAR23SZ_OFFSET: c_uint = 0x00d2;
pub const XEON_SBAR45SZ_OFFSET: c_uint = 0x00d3;
pub const XEON_SBAR4SZ_OFFSET: c_uint = 0x00d3;
pub const XEON_PPD_OFFSET: c_uint = 0x00d4;
pub const XEON_PBAR5SZ_OFFSET: c_uint = 0x00d5;
pub const XEON_SBAR5SZ_OFFSET: c_uint = 0x00d6;
pub const XEON_WCCNTRL_OFFSET: c_uint = 0x00e0;
pub const XEON_UNCERRSTS_OFFSET: c_uint = 0x014c;
pub const XEON_CORERRSTS_OFFSET: c_uint = 0x0158;
pub const XEON_LINK_STATUS_OFFSET: c_uint = 0x01a2;
pub const XEON_SPCICMD_OFFSET: c_uint = 0x0504;
pub const XEON_DEVCTRL_OFFSET: c_uint = 0x0598;
pub const XEON_DEVSTS_OFFSET: c_uint = 0x059a;
pub const XEON_SLINK_STATUS_OFFSET: c_uint = 0x05a2;
pub const XEON_B2B_SPAD_OFFSET: c_uint = 0x0100;
pub const XEON_B2B_DOORBELL_OFFSET: c_uint = 0x0140;
pub const XEON_B2B_XLAT_OFFSETL: c_uint = 0x0144;
pub const XEON_B2B_XLAT_OFFSETU: c_uint = 0x0148;
pub const XEON_PPD_CONN_MASK: c_uint = 0x03;
pub const XEON_PPD_CONN_TRANSPARENT: c_uint = 0x00;
pub const XEON_PPD_CONN_B2B: c_uint = 0x01;
pub const XEON_PPD_CONN_RP: c_uint = 0x02;
pub const XEON_PPD_DEV_MASK: c_uint = 0x10;
pub const XEON_PPD_DEV_USD: c_uint = 0x00;
pub const XEON_PPD_DEV_DSD: c_uint = 0x10;
pub const XEON_PPD_SPLIT_BAR_MASK: c_uint = 0x40;

pub const XEON_MW_COUNT: c_int = 2;
pub const HSX_SPLIT_BAR_MW_COUNT: c_int = 3;
pub const XEON_DB_COUNT: c_int = 15;
pub const XEON_DB_LINK: c_int = 15;

pub const XEON_DB_MSIX_VECTOR_COUNT: c_int = 4;
pub const XEON_DB_MSIX_VECTOR_SHIFT: c_int = 5;
pub const XEON_DB_TOTAL_SHIFT: c_int = 16;
pub const XEON_SPAD_COUNT: c_int = 16;
// Use the following addresses for translation between b2b ntb devices in case
// the hardware default values are not reliable.
pub const XEON_B2B_BAR0_ADDR: c_uint = 0x1000000000000000ull;
pub const XEON_B2B_BAR2_ADDR64: c_uint = 0x2000000000000000ull;
pub const XEON_B2B_BAR4_ADDR64: c_uint = 0x4000000000000000ull;
pub const XEON_B2B_BAR4_ADDR32: c_uint = 0x20000000u;
pub const XEON_B2B_BAR5_ADDR32: c_uint = 0x40000000u;
// The peer ntb secondary config space is 32KB fixed size
pub const XEON_B2B_MIN_SIZE: c_uint = 0x8000;
// flags to indicate hardware errata

extern "C" {
    pub fn xeon_ppd_topo(ndev: *mut intel_ntb_dev, ppd: u8) -> ntb_topo;
}
extern "C" {
    pub fn ndev_db_read(ndev: *mut intel_ntb_dev, mmio: *mut void __iomem) -> u64;
}
extern "C" {
    pub fn ndev_mw_to_bar(ndev: *mut intel_ntb_dev, idx: c_int) -> c_int;
}
extern "C" {
    pub fn intel_ntb_mw_count(ntb: *mut ntb_dev, pidx: c_int) -> c_int;
}
extern "C" {
    pub fn intel_ntb_peer_mw_count(ntb: *mut ntb_dev) -> c_int;
}
extern "C" {
    pub fn intel_ntb_link_disable(ntb: *mut ntb_dev) -> c_int;
}
extern "C" {
    pub fn intel_ntb_db_valid_mask(ntb: *mut ntb_dev) -> u64;
}
extern "C" {
    pub fn intel_ntb_db_vector_count(ntb: *mut ntb_dev) -> c_int;
}
extern "C" {
    pub fn intel_ntb_db_vector_mask(ntb: *mut ntb_dev, db_vector: c_int) -> u64;
}
extern "C" {
    pub fn intel_ntb_db_set_mask(ntb: *mut ntb_dev, db_bits: u64) -> c_int;
}
extern "C" {
    pub fn intel_ntb_db_clear_mask(ntb: *mut ntb_dev, db_bits: u64) -> c_int;
}
extern "C" {
    pub fn intel_ntb_spad_is_unsafe(ntb: *mut ntb_dev) -> c_int;
}
extern "C" {
    pub fn intel_ntb_spad_count(ntb: *mut ntb_dev) -> c_int;
}
extern "C" {
    pub fn intel_ntb_spad_read(ntb: *mut ntb_dev, idx: c_int) -> u32;
}
extern "C" {
    pub fn intel_ntb_spad_write(ntb: *mut ntb_dev, idx: c_int, val: u32) -> c_int;
}
extern "C" {
    pub fn intel_ntb_peer_spad_read(ntb: *mut ntb_dev, pidx: c_int, sidx: c_int) -> u32;
}
extern "C" {
    pub fn xeon_link_is_up(ndev: *mut intel_ntb_dev) -> c_int;
}
