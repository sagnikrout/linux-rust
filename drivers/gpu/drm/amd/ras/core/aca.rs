//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/ras/core/aca.h
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
//
// Copyright 2025 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//

pub const MAX_SOCKET_NUM_PER_HIVE: c_int = 8;
pub const MAX_AID_NUM_PER_SOCKET: c_int = 4;
pub const MAX_XCD_NUM_PER_AID: c_int = 2;
pub const MAX_ACA_RAS_BLOCK: c_int = 20;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ras_aca_reg_idx {
    ACA_REG_IDX__CTL		= 0,
    ACA_REG_IDX__STATUS		= 1,
    ACA_REG_IDX__ADDR		= 2,
    ACA_REG_IDX__MISC0		= 3,
    ACA_REG_IDX__CONFG		= 4,
    ACA_REG_IDX__IPID		= 5,
    ACA_REG_IDX__SYND		= 6,
    ACA_REG_IDX__DESTAT		= 8,
    ACA_REG_IDX__DEADDR		= 9,
    ACA_REG_IDX__CTL_MASK	= 10,
    ACA_REG_MAX_COUNT		= 16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aca_bank_reg {
    pub ecc_type: u32,
    pub seq_no: u64,
    pub regs: [u64; ACA_REG_MAX_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aca_ecc_hwip {
    ACA_ECC_HWIP__UNKNOWN = -1,
    ACA_ECC_HWIP__PSP = 0,
    ACA_ECC_HWIP__UMC,
    ACA_ECC_HWIP__SMU,
    ACA_ECC_HWIP__PCS_XGMI,
    ACA_ECC_HWIP_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aca_ecc_info {
    pub die_id: c_int,
    pub socket_id: c_int,
    pub xcd_id: c_int,
    pub hwid: c_int,
    pub mcatype: c_int,
    pub status: u64,
    pub ipid: u64,
    pub addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aca_bank_ecc {
    pub bank_info: aca_ecc_info,
    pub ce_count: u32,
    pub ue_count: u32,
    pub de_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aca_ecc_count {
    pub new_ce_count: u32,
    pub total_ce_count: u32,
    pub new_ue_count: u32,
    pub total_ue_count: u32,
    pub new_de_count: u32,
    pub total_de_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aca_xcd_ecc {
    pub ecc_err: aca_ecc_count,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aca_aid_ecc {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aca_xcd {
    pub xcd: [aca_xcd_ecc; MAX_XCD_NUM_PER_AID],
    pub xcd_num: u32,
    pub xcd: },
    pub ecc_err: aca_ecc_count,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aca_socket_ecc {
    pub aid: [aca_aid_ecc; MAX_AID_NUM_PER_SOCKET],
    pub aid_num: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aca_block_ecc {
    pub socket: [aca_socket_ecc; MAX_SOCKET_NUM_PER_HIVE],
    pub socket_num_per_hive: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aca_bank_hw_ops {
    pub data): *mut *mut *mut bool (bank_match)(struct aca_block ras_blk, void,
    pub buf): *mut *mut *mut aca_block aca_blk, void data, void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aca_block_info {
    pub name: [c_char; 32],
    pub ras_block_id: u32,
    pub hwip: aca_ecc_hwip,
    pub bank_ops: aca_bank_hw_ops,
    pub mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aca_block {
    pub blk_info: *const aca_block_info,
    pub ecc: aca_block_ecc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_aca_ip_func {
    pub block_num: u32,
    pub block_info: *const aca_block_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_aca {
    pub aca_ip_version: u32,
    pub ip_func: *const ras_aca_ip_func,
    pub aca_lock: mutex,
    pub bank_op_lock: mutex,
    pub aca_blk: [aca_block; MAX_ACA_RAS_BLOCK],
    pub ue_updated_mark: u32,
}

extern "C" {
    pub fn ras_aca_sw_init(ras_core: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_aca_sw_fini(ras_core: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_aca_hw_init(ras_core: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_aca_hw_fini(ras_core: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_aca_get_block_ecc_count(ras_core: *mut ras_core_context, blk: u32, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn ras_aca_clear_block_new_ecc_count(ras_core: *mut ras_core_context, blk: u32) -> c_int;
}
extern "C" {
    pub fn ras_aca_clear_all_blocks_ecc_count(ras_core: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_aca_update_ecc(ras_core: *mut ras_core_context, ecc_type: u32, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn ras_aca_mark_fatal_flag(ras_core: *mut ras_core_context);
}
extern "C" {
    pub fn ras_aca_clear_fatal_flag(ras_core: *mut ras_core_context);
}
