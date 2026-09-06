//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/pcie/iwl-context-info.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (C) 2017 Intel Deutschland GmbH
// Copyright (C) 2018-2020, 2022, 2024-2025 Intel Corporation
//

// Macro flag: #define __iwl_context_info_file_h__
// maximum number of DRAM map entries supported by FW
pub const IWL_MAX_DRAM_ENTRY: c_int = 64;
pub const CSR_CTXT_INFO_BA: c_uint = 0x40;
//
// enum iwl_context_info_flags - Context information control flags
// @IWL_CTXT_INFO_AUTO_FUNC_INIT: If set, FW will not wait before interrupting
// the init done for driver command that configures several system modes
// @IWL_CTXT_INFO_EARLY_DEBUG: enable early debug
// @IWL_CTXT_INFO_ENABLE_CDMP: enable core dump
// @IWL_CTXT_INFO_RB_CB_SIZE: mask of the RBD Cyclic Buffer Size
// exponent, the actual size is 2**value, valid sizes are 8-2048.
// The value is four bits long. Maximum valid exponent is 12
// @IWL_CTXT_INFO_TFD_FORMAT_LONG: use long TFD Format (the
// default is short format - not supported by the driver)
// @IWL_CTXT_INFO_RB_SIZE: RB size mask
// (values are IWL_CTXT_INFO_RB_SIZE_*K)
// @IWL_CTXT_INFO_RB_SIZE_1K: Value for 1K RB size
// @IWL_CTXT_INFO_RB_SIZE_2K: Value for 2K RB size
// @IWL_CTXT_INFO_RB_SIZE_4K: Value for 4K RB size
// @IWL_CTXT_INFO_RB_SIZE_8K: Value for 8K RB size
// @IWL_CTXT_INFO_RB_SIZE_12K: Value for 12K RB size
// @IWL_CTXT_INFO_RB_SIZE_16K: Value for 16K RB size
// @IWL_CTXT_INFO_RB_SIZE_20K: Value for 20K RB size
// @IWL_CTXT_INFO_RB_SIZE_24K: Value for 24K RB size
// @IWL_CTXT_INFO_RB_SIZE_28K: Value for 28K RB size
// @IWL_CTXT_INFO_RB_SIZE_32K: Value for 32K RB size
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_context_info_flags {
    IWL_CTXT_INFO_AUTO_FUNC_INIT	= 0x0001,
    IWL_CTXT_INFO_EARLY_DEBUG	= 0x0002,
    IWL_CTXT_INFO_ENABLE_CDMP	= 0x0004,
    IWL_CTXT_INFO_RB_CB_SIZE	= 0x00f0,
    IWL_CTXT_INFO_TFD_FORMAT_LONG	= 0x0100,
    IWL_CTXT_INFO_RB_SIZE		= 0x1e00,
    IWL_CTXT_INFO_RB_SIZE_1K	= 0x1,
    IWL_CTXT_INFO_RB_SIZE_2K	= 0x2,
    IWL_CTXT_INFO_RB_SIZE_4K	= 0x4,
    IWL_CTXT_INFO_RB_SIZE_8K	= 0x8,
    IWL_CTXT_INFO_RB_SIZE_12K	= 0x9,
    IWL_CTXT_INFO_RB_SIZE_16K	= 0xa,
    IWL_CTXT_INFO_RB_SIZE_20K	= 0xb,
    IWL_CTXT_INFO_RB_SIZE_24K	= 0xc,
    IWL_CTXT_INFO_RB_SIZE_28K	= 0xd,
    IWL_CTXT_INFO_RB_SIZE_32K	= 0xe,
}

//
// struct iwl_context_info_version - version structure
// @mac_id: SKU and revision id
// @version: context information version id
// @size: the size of the context information in DWs
// @reserved: (reserved)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_context_info_version {
    pub mac_id: __le16,
    pub version: __le16,
    pub size: __le16,
    pub reserved: __le16,
    pub __packed: },
//
// struct iwl_context_info_control - version structure
// @control_flags: context information flags see &enum iwl_context_info_flags
// @reserved: (reserved)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_context_info_control {
    pub control_flags: __le32,
    pub reserved: __le32,
    pub __packed: },
//
// struct iwl_context_info_dram_nonfseq - images DRAM map
// each entry in the map represents a DRAM chunk of up to 32 KB
// @umac_img: UMAC image DRAM map
// @lmac_img: LMAC image DRAM map
// @virtual_img: paged image DRAM map
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_context_info_dram_nonfseq {
    pub umac_img: [__le64; IWL_MAX_DRAM_ENTRY],
    pub lmac_img: [__le64; IWL_MAX_DRAM_ENTRY],
    pub virtual_img: [__le64; IWL_MAX_DRAM_ENTRY],
    pub __packed: },
//
// struct iwl_context_info_rbd_cfg - RBDs configuration
// @free_rbd_addr: default queue free RB CB base address
// @used_rbd_addr: default queue used RB CB base address
// @status_wr_ptr: default queue used RB status write pointer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_context_info_rbd_cfg {
    pub free_rbd_addr: __le64,
    pub used_rbd_addr: __le64,
    pub status_wr_ptr: __le64,
    pub __packed: },
//
// struct iwl_context_info_hcmd_cfg  - command queue configuration
// @cmd_queue_addr: address of command queue
// @cmd_queue_size: number of entries
// @reserved: (reserved)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_context_info_hcmd_cfg {
    pub cmd_queue_addr: __le64,
    pub cmd_queue_size: u8,
    pub reserved: [u8; 7],
    pub __packed: },
//
// struct iwl_context_info_dump_cfg - Core Dump configuration
// @core_dump_addr: core dump (debug DRAM address) start address
// @core_dump_size: size, in DWs
// @reserved: (reserved)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_context_info_dump_cfg {
    pub core_dump_addr: __le64,
    pub core_dump_size: __le32,
    pub reserved: __le32,
    pub __packed: },
//
// struct iwl_context_info_pnvm_cfg - platform NVM data configuration
// @platform_nvm_addr: Platform NVM data start address
// @platform_nvm_size: size in DWs
// @reserved: (reserved)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_context_info_pnvm_cfg {
    pub platform_nvm_addr: __le64,
    pub platform_nvm_size: __le32,
    pub reserved: __le32,
    pub __packed: },
//
// struct iwl_context_info_early_dbg_cfg - early debug configuration for
// dumping DRAM addresses
// @early_debug_addr: early debug start address
// @early_debug_size: size in DWs
// @reserved: (reserved)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_context_info_early_dbg_cfg {
    pub early_debug_addr: __le64,
    pub early_debug_size: __le32,
    pub reserved: __le32,
    pub __packed: },
//
// struct iwl_context_info - device INIT configuration
// @version: version information of context info and HW
// @control: control flags of FH configurations
// @reserved0: (reserved)
// @rbd_cfg: default RX queue configuration
// @hcmd_cfg: command queue configuration
// @reserved1: (reserved)
// @dump_cfg: core dump data
// @edbg_cfg: early debug configuration
// @pnvm_cfg: platform nvm configuration
// @reserved2: (reserved)
// @dram: firmware image addresses in DRAM
// @reserved3: (reserved)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_context_info {
    pub version: iwl_context_info_version,
    pub control: iwl_context_info_control,
    pub reserved0: __le64,
    pub rbd_cfg: iwl_context_info_rbd_cfg,
    pub hcmd_cfg: iwl_context_info_hcmd_cfg,
    pub reserved1: [__le32; 4],
    pub dump_cfg: iwl_context_info_dump_cfg,
    pub edbg_cfg: iwl_context_info_early_dbg_cfg,
    pub pnvm_cfg: iwl_context_info_pnvm_cfg,
    pub reserved2: [__le32; 16],
    pub dram: iwl_context_info_dram_nonfseq,
    pub reserved3: [__le32; 16],
    pub /: *mut *mut } __packed; / BOOT_LOADER_CONTEXT_INFO_S,
    pub img): *const *const int iwl_pcie_ctxt_info_init(struct iwl_trans trans, struct fw_img,
    pub trans): *mut void iwl_pcie_ctxt_info_free(struct iwl_trans,
    pub trans): *mut void iwl_pcie_ctxt_info_free_paging(struct iwl_trans,
    pub ctxt_dram): *mut iwl_context_info_dram_nonfseq,
    pub phys): *mut dma_addr_t,
    pub dram): *mut iwl_dram_data,
