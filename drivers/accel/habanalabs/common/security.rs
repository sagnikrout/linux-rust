//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/common/security.h
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
//
// Copyright 2016-2022 HabanaLabs, Ltd.
// All Rights Reserved.
//

// special blocks

// GLBL_ERR_ADDR register offset from the start of the block
pub const HL_GLBL_ERR_ADDR_OFFSET: c_uint = 0xF44;
// GLBL_ERR_CAUSE register offset from the start of the block
pub const HL_GLBL_ERR_CAUSE_OFFSET: c_uint = 0xF48;
//
// struct hl_special_block_info - stores address details of a particular type of
// IP block which has a SPECIAL part.
//
// @block_type: block type as described in every ASIC's block_types enum.
// @base_addr: base address of the first block of particular type,
// e.g., address of NIC0_UMR0_0 of 'NIC_UMR' block.
// @major: number of major blocks of particular type.
// @minor: number of minor blocks of particular type.
// @sub_minor: number of sub minor blocks of particular type.
// @major_offset: address gap between 2 consecutive major blocks of particular type,
// e.g., offset between NIC0_UMR0_0 and NIC1_UMR0_0 is 0x80000.
// @minor_offset: address gap between 2 consecutive minor blocks of particular type,
// e.g., offset between NIC0_UMR0_0 and NIC0_UMR1_0 is 0x20000.
// @sub_minor_offset: address gap between 2 consecutive sub_minor blocks of particular
// type, e.g., offset between NIC0_UMR0_0 and NIC0_UMR0_1 is 0x1000.
//
// e.g., in Gaudi2, NIC_UMR blocks can be interpreted as:
// NIC<major>_UMR<minor>_<sub_minor> where major=12, minor=2, sub_minor=15.
// In other words, for each of 12 major numbers (i.e 0 to 11) there are
// 2 blocks with different minor numbers (i.e. 0 to 1). Again, for each minor
// number there are 15 blocks with different sub_minor numbers (i.e. 0 to 14).
// So different blocks are NIC0_UMR0_0, NIC0_UMR0_1, ..., NIC0_UMR1_0, ....,
// NIC11_UMR1_14.
//
// Struct's formatted data is located in the SOL-based auto-generated protbits headers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_special_block_info {
    pub block_type: c_int,
    pub base_addr: u32,
    pub major: u32,
    pub minor: u32,
    pub sub_minor: u32,
    pub major_offset: u32,
    pub minor_offset: u32,
    pub sub_minor_offset: u32,
}

//
// struct hl_automated_pb_cfg - represents configurations of a particular type
// of IP block which has protection bits.
//
// @addr: address details as described in hl_automation_pb_addr struct.
// @prot_map: each bit corresponds to one among 32 protection configuration regs
// (e.g., SPECIAL_GLBL_PRIV). '1' means 0xffffffff and '0' means 0x0
// to be written into the corresponding protection configuration reg.
// This bit is meaningful if same bit in data_map is 0, otherwise ignored.
// @data_map: each bit corresponds to one among 32 protection configuration regs
// (e.g., SPECIAL_GLBL_PRIV). '1' means corresponding protection
// configuration reg is to be written with a value in array pointed
// by 'data', otherwise the value is decided by 'prot_map'.
// @data: pointer to data array which stores the config value(s) to be written
// to corresponding protection configuration reg(s).
// @data_size: size of the data array.
//
// Each bit of 'data_map' and 'prot_map' fields corresponds to one among 32
// protection configuration registers e.g., SPECIAL GLBL PRIV regs (starting at
// offset 0xE80). '1' in 'data_map' means protection configuration to be done
// using configuration in data array. '0' in 'data_map" means protection
// configuration to be done as per the value of corresponding bit in 'prot_map'.
// '1' in 'prot_map' means the register to be programmed with 0xFFFFFFFF
// (all non-protected). '0' in 'prot_map' means the register to be programmed
// with 0x0 (all protected).
//
// e.g., prot_map = 0x00000001, data_map = 0xC0000000 , data = {0xff, 0x12}
// SPECIAL_GLBL_PRIV[0] = 0xFFFFFFFF
// SPECIAL_GLBL_PRIV[1..29] = 0x0
// SPECIAL_GLBL_PRIV[30] = 0xFF
// SPECIAL_GLBL_PRIV[31] = 0x12
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_automated_pb_cfg {
    pub addr: hl_special_block_info,
    pub prot_map: u32,
    pub data_map: u32,
    pub data: *const u32,
    pub data_size: u8,
}

// struct hl_special_blocks_cfg - holds special blocks cfg data.
//
// @priv_automated_pb_cfg: points to the main privileged PB array.
// @sec_automated_pb_cfg: points to the main secured PB array.
// @skip_blocks_cfg: holds arrays of block types & block ranges to be excluded.
// @priv_cfg_size: size of the main privileged PB array.
// @sec_cfg_size: size of the main secured PB array.
// @prot_lvl_priv: indication if it's a privileged/secured PB configurations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_special_blocks_cfg {
    pub priv_automated_pb_cfg: *mut hl_automated_pb_cfg,
    pub sec_automated_pb_cfg: *mut hl_automated_pb_cfg,
    pub skip_blocks_cfg: *mut hl_skip_blocks_cfg,
    pub priv_cfg_size: u32,
    pub sec_cfg_size: u32,
    pub prot_lvl_priv: u8,
}

// Automated security
// struct hl_skip_blocks_cfg - holds arrays of block types & block ranges to be
// excluded from special blocks configurations.
//
// @block_types: an array of block types NOT to be configured.
// @block_types_len: len of an array of block types not to be configured.
// @block_ranges: an array of block ranges not to be configured.
// @block_ranges_len: len of an array of block ranges not to be configured.
// @skip_block_hook: hook that will be called before initializing special blocks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_skip_blocks_cfg {
    pub block_types: *mut c_int,
    pub block_types_len: usize,
    pub block_ranges: *mut range,
    pub block_ranges_len: usize,
    pub sub_minor): u32 blk_idx, u32 major, u32 minor, u32,
}

//
// struct iterate_special_ctx - HW module special block iterator
// @fn: function to apply to each HW module special block instance
// @data: optional internal data to the function iterator
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iterate_special_ctx {
//
// callback for the HW module special block iterator
// @hdev: pointer to the habanalabs device structure
// @block_id: block (ASIC specific definition can be dcore/hdcore)
// @major: major block index within block_id
// @minor: minor block index within the major block
// @sub_minor: sub_minor block index within the minor block
// @data: function specific data
//
    pub data): *mut u32 sub_minor, void,
    pub data: *mut c_void,
}

extern "C" {
    pub fn hl_iterate_special_blocks(hdev: *mut hl_device, ctx: *mut iterate_special_ctx) -> c_int;
}
extern "C" {
    pub fn hl_check_for_glbl_errors(hdev: *mut hl_device);
}
