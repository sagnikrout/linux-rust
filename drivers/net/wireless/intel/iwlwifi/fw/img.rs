//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/img.h
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
// Copyright (C) 2005-2014, 2018-2024, 2026 Intel Corporation
// Copyright (C) 2013-2015 Intel Mobile Communications GmbH
// Copyright (C) 2016 Intel Deutschland GmbH
//

// Macro flag: #define __iwl_fw_img_h__

//
// enum iwl_ucode_type - type of ucode
//
// @IWL_UCODE_REGULAR: Normal runtime ucode
// @IWL_UCODE_INIT: Initial ucode
// @IWL_UCODE_WOWLAN: Wake on Wireless enabled ucode
// @IWL_UCODE_REGULAR_USNIFFER: Normal runtime ucode when using usniffer image
// @IWL_UCODE_TYPE_MAX: (internal value)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_ucode_type {
    IWL_UCODE_REGULAR,
    IWL_UCODE_INIT,
    IWL_UCODE_WOWLAN,
    IWL_UCODE_REGULAR_USNIFFER,
    IWL_UCODE_TYPE_MAX,
}

//
// enumeration of ucode section.
// This enumeration is used directly for older firmware (before 16.0).
// For new firmware, there can be up to 4 sections (see below) but the
// first one packaged into the firmware file is the DATA section and
// some debugging code accesses that.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_ucode_sec {
    IWL_UCODE_SECTION_DATA,
    IWL_UCODE_SECTION_INST,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_ucode_capabilities {
    pub max_probe_length: u32,
    pub n_scan_channels: u32,
    pub standard_phy_calibration_size: u32,
    pub flags: u32,
    pub error_log_addr: u32,
    pub error_log_size: u32,
    pub num_stations: u32,
    pub num_links: u32,
    pub num_beacons: u32,
    pub num_mcast_key_entries: u32,
    pub nan_max_chan_switch_time: u16,
    pub NUM_IWL_UCODE_TLV_API): DECLARE_BITMAP(_api,,
    pub NUM_IWL_UCODE_TLV_CAPA): DECLARE_BITMAP(_capa,,
    pub cmd_versions: *const iwl_fw_cmd_version,
    pub n_cmd_versions: u32,
    pub cmd_bios_tables: *const iwl_fw_cmd_bios_table,
    pub n_cmd_bios_tables: u32,
}

extern "C" {
    pub fn test_bit(long)api: (, _arg: capabilities->_api) -> return;
}
extern "C" {
    pub fn test_bit(long)capa: (, _arg: capabilities->_capa) -> return;
}
// one for each uCode image (inst/data, init/runtime/wowlan)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_desc {
    pub /: *const *const *const void data; / vmalloc'ed data,
    pub /: *mut *mut u32 len; / size in bytes,
    pub /: *mut *mut u32 offset; / offset in the device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_img {
    pub sec: *mut fw_desc,
    pub num_sec: c_int,
    pub is_dual_cpus: bool,
    pub paging_mem_size: u32,
}

//
// Block paging calculations
//

pub const PAGE_PER_GROUP_2_EXP_SIZE: c_int = 3;
// 8 pages per group

// don't change, support only 32KB size

// 32K == 2^15

//
// Image paging calculations
//
pub const BLOCK_PER_IMAGE_2_EXP_SIZE: c_int = 5;
// 2^5 == 32 blocks per image

// maximum image size 1024KB

// Virtual address signature
pub const PAGING_ADDR_SIG: c_uint = 0xAA000000;

pub const PAGING_CMD_NUM_OF_PAGES_IN_LAST_GRP_POS: c_int = 0;
pub const PAGING_TLV_SECURE_MASK: c_int = 1;
// FW MSB Mask for regions/cache_control
pub const FW_ADDR_CACHE_CONTROL: c_uint = 0xC0000000UL;
//
// struct iwl_fw_paging - FW paging descriptor
// @fw_paging_phys: page phy pointer
// @fw_paging_block: pointer to the allocated block
// @fw_paging_size: page size
// @fw_offs: offset in the device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_fw_paging {
    pub fw_paging_phys: dma_addr_t,
    pub fw_paging_block: *mut page,
    pub fw_paging_size: u32,
    pub fw_offs: u32,
}

//
// enum iwl_fw_type - iwlwifi firmware type
// @IWL_FW_DVM: DVM firmware
// @IWL_FW_MVM: MVM firmware
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_fw_type {
    IWL_FW_DVM,
    IWL_FW_MVM,
}

//
// struct iwl_fw_dbg - debug data
//
// @dest_tlv: points to debug destination TLV (typically SRAM or DRAM)
// @n_dest_reg: num of reg_ops in dest_tlv
// @conf_tlv: array of pointers to configuration HCMDs
// @trigger_tlv: array of pointers to triggers TLVs
// @trigger_tlv_len: lengths of the @dbg_trigger_tlv entries
// @mem_tlv: Runtime addresses to dump
// @n_mem_tlv: number of runtime addresses
// @dump_mask: bitmask of dump regions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_fw_dbg {
    pub dest_tlv: *mut iwl_fw_dbg_dest_tlv_v1,
    pub n_dest_reg: u8,
    pub conf_tlv: [*mut iwl_fw_dbg_conf_tlv; FW_DBG_CONF_MAX],
    pub trigger_tlv: [*mut iwl_fw_dbg_trigger_tlv; FW_DBG_TRIGGER_MAX],
    pub trigger_tlv_len: [usize; FW_DBG_TRIGGER_MAX],
    pub mem_tlv: *mut iwl_fw_dbg_mem_seg_tlv,
    pub n_mem_tlv: usize,
    pub dump_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_dump_exclude {
    pub size: u32 addr,,
}

//
// struct iwl_fw - variables associated with the firmware
//
// @ucode_ver: ucode version from the ucode file
// @fw_version: firmware version string
// @img: ucode image like ucode_rt, ucode_init, ucode_wowlan.
// @iml_len: length of the image loader image
// @iml: image loader fw image
// @ucode_capa: capabilities parsed from the ucode file.
// @enhance_sensitivity_table: device can do enhanced sensitivity.
// @init_evtlog_ptr: event log offset for init ucode.
// @init_evtlog_size: event log size for init ucode.
// @init_errlog_ptr: error log offset for init ucode.
// @inst_evtlog_ptr: event log offset for runtime ucode.
// @inst_evtlog_size: event log size for runtime ucode.
// @inst_errlog_ptr: error log offset for runtime ucode.
// @type: firmware type (&enum iwl_fw_type)
// @human_readable: human readable version
// we get the ALIVE from the uCode
// @phy_integration_ver: PHY integration version string
// @phy_integration_ver_len: length of @phy_integration_ver
// @dump_excl: image dump exclusion areas for RT image
// @dump_excl_wowlan: image dump exclusion areas for WoWLAN image
// @pnvm_data: PNVM data embedded in the .ucode file, if any
// @pnvm_size: size of the embedded PNVM data
// @dbg: debug data, see &struct iwl_fw_dbg
// @default_calib: default calibration data
// @phy_config: PHY configuration flags
// @valid_rx_ant: valid RX antenna bitmap
// @valid_tx_ant: valid TX antenna bitmap
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_fw {
    pub ucode_ver: u32,
    pub fw_version: [c_char; 128],
// ucode images
    pub img: [fw_img; IWL_UCODE_TYPE_MAX],
    pub iml_len: usize,
    pub iml: *mut u8,
    pub ucode_capa: iwl_ucode_capabilities,
    pub enhance_sensitivity_table: bool,
    pub init_errlog_ptr: u32 init_evtlog_ptr, init_evtlog_size,,
    pub inst_errlog_ptr: u32 inst_evtlog_ptr, inst_evtlog_size,,
    pub default_calib: [iwl_tlv_calib_ctrl; IWL_UCODE_TYPE_MAX],
    pub phy_config: u32,
    pub valid_tx_ant: u8,
    pub valid_rx_ant: u8,
    pub type: iwl_fw_type,
    pub human_readable: [u8; FW_VER_HUMAN_READABLE_SZ],
    pub dbg: iwl_fw_dbg,
    pub phy_integration_ver: *mut u8,
    pub phy_integration_ver_len: u32,
    pub dump_excl_wowlan: [iwl_dump_exclude dump_excl[2],; 2],
    pub pnvm_data: *const c_void,
    pub pnvm_size: u32,
}

extern "C" {
    pub fn iwl_fw_lookup_cmd_ver(fw: *const iwl_fw, cmd_id: u32, def: u8) -> u8;
}
extern "C" {
    pub fn iwl_fw_lookup_notif_ver(fw: *const iwl_fw, grp: u8, cmd: u8, def: u8) -> u8;
}
pub const FW_SYSASSERT_CPU_MASK: c_uint = 0xf0000000;
pub const FW_SYSASSERT_PNVM_MISSING: c_uint = 0x0010070d;
