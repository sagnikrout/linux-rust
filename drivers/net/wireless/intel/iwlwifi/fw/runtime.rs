//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/runtime.h
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
// Copyright (C) 2018-2026 Intel Corporation
//

// Macro flag: #define __iwl_fw_runtime_h__

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_fw_runtime_ops {
    pub ctx): *mut *mut void (dump_start)(void,
    pub ctx): *mut *mut void (dump_end)(void,
    pub host_cmd): *mut *mut *mut int (send_hcmd)(void ctx, struct iwl_host_cmd,
    pub ctx): *mut *mut bool (d3_debug_enable)(void,
}

pub const MAX_NUM_LMAC: c_int = 2;
pub const MAX_NUM_TCM: c_int = 2;
pub const MAX_NUM_RCM: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_fwrt_shared_mem_cfg {
    pub num_lmacs: c_int,
    pub num_txfifo_entries: c_int,
    pub txfifo_size: [u32; TX_FIFO_MAX_NUM],
    pub rxfifo1_size: u32,
    pub lmac: [}; MAX_NUM_LMAC],
    pub rxfifo2_size: u32,
    pub rxfifo2_control_size: u32,
    pub internal_txfifo_addr: u32,
    pub internal_txfifo_size: [u32; TX_FIFO_INTERNAL_MAX_NUM],
}

pub const IWL_FW_RUNTIME_DUMP_WK_NUM: c_int = 5;
//
// struct iwl_fwrt_dump_data - dump data
// @trig: trigger the worker was scheduled upon
// @fw_pkt: packet received from FW
// @desc: dump descriptor
// @monitor_only: only dump for monitor
//
// Note that the decision which part of the union is used
// is based on iwl_trans_dbg_ini_valid(): the 'trig' part
// is used if it is %true, the 'desc' part otherwise.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_fwrt_dump_data {
    pub trig: *mut iwl_fw_ini_trigger_tlv,
    pub fw_pkt: *mut iwl_rx_packet,
}

// must be first to be same as 'trig'
//
// struct iwl_fwrt_wk_data - dump worker data struct
// @idx: index of the worker
// @wk: worker
// @dump_data: dump data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_fwrt_wk_data {
    pub idx: u8,
    pub wk: delayed_work,
    pub dump_data: iwl_fwrt_dump_data,
}

//
// struct iwl_txf_iter_data - Tx fifo iterator data struct
// @fifo: fifo number
// @lmac: lmac number
// @fifo_size: fifo size
// @internal_txf: non zero if fifo is  internal Tx fifo
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_txf_iter_data {
    pub fifo: c_int,
    pub lmac: c_int,
    pub fifo_size: u32,
    pub internal_txf: u8,
}

//
// struct iwl_fw_runtime - runtime data for firmware
// @trans: transport pointer
// @fw: firmware image
// @dev: device pointer
// @ops: user ops
// @ops_ctx: user ops context
// @fw_paging_db: paging database
// @num_of_paging_blk: number of paging blocks
// @num_of_pages_in_last_blk: number of pages in the last block
// @smem_cfg: saved firmware SMEM configuration
// @cur_fw_img: current firmware image, must be maintained by
// the driver by calling &iwl_fw_set_current_image()
// @dump: debug dump data
// @ap_type_cmd: AP type tables (for enablement on 6 GHz)
// @ap_type_cmd_valid: if &ap_type_cmd is valid
// @uefi_tables_lock_status: The status of the WIFI GUID UEFI variables lock:
// 0: Unlocked, 1 and 2: Locked.
// Only read the UEFI variables if locked.
// @sar_profiles: sar profiles as read from WRDS/EWRD BIOS tables
// @geo_profiles: geographic profiles as read from WGDS BIOS table
// @geo_bios_source: see &enum bios_source
// @phy_filters: specific phy filters as read from WPFC BIOS table
// @ppag_bios_rev: PPAG BIOS revision
// @ppag_bios_source: see &enum bios_source
// @dsm_funcs_valid: bitmap indicating which DSM values are valid,
// zero (default initialization) means it hasn't been read yet,
// and BIT(0) is set when it has since function 0 also has this
// bitmap and is always supported.
// If the bit is set for a specific function, then the corresponding
// entry in &dsm_values is valid.
// @dsm_values: cache of the DSM values. The validity of each entry is
// determined by &dsm_funcs_valid.
// @geo_enabled: WGDS table is present
// @geo_num_profiles: number of geo profiles
// @geo_rev: geo profiles table revision
// @ppag_chains: PPAG table data
// @ppag_flags: PPAG flags
// @reduced_power_flags: reduced power flags
// @sanitize_ctx: context for dump sanitizer
// @sanitize_ops: dump sanitizer ops
// @sar_chain_a_profile: SAR chain A profile
// @sar_chain_b_profile: SAR chain B profile
// @sgom_enabled: SGOM enabled
// @sgom_table: SGOM table
// @timestamp: timestamp marker data
// @timestamp.wk: timestamp marking worker
// @timestamp.seq: timestamp marking sequence
// @timestamp.delay: timestamp marking worker delay
// @tpc_enabled: TPC enabled
// @dsm_source: one of &enum bios_source. UEFI, ACPI or NONE
// @dsm_revision: the revision of the DSM table
// @wbem_source: one of &enum bios_source for the WBEM table
// @wbem_revision: the revision of the WBEM table
// @puncturing_source: one of &enum bios_source for the puncturing table
// @puncturing_revision: the revision of the puncturing table
// @bios_puncturing: per-country puncturing enablement bitmap from BIOS
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_fw_runtime {
    pub trans: *mut iwl_trans,
    pub fw: *const iwl_fw,
    pub dev: *mut device,
    pub ops: *const iwl_fw_runtime_ops,
    pub ops_ctx: *mut c_void,
    pub sanitize_ops: *const iwl_dump_sanitize_ops,
    pub sanitize_ctx: *mut c_void,
// Paging
    pub fw_paging_db: [iwl_fw_paging; NUM_OF_FW_PAGING_BLOCKS],
    pub num_of_paging_blk: u16,
    pub num_of_pages_in_last_blk: u16,
    pub cur_fw_img: iwl_ucode_type,
// memory configuration
    pub smem_cfg: iwl_fwrt_shared_mem_cfg,
// debug
    pub wks: [iwl_fwrt_wk_data; IWL_FW_RUNTIME_DUMP_WK_NUM],
    pub active_wks: c_ulong,
    pub conf: u8,
// ts of the beginning of a non-collect fw dbg data period
    pub non_collect_ts_start: [c_ulong; IWL_FW_INI_TIME_POINT_NUM],
    pub d3_debug_data: *mut u32,
    pub lmac_err_id: [u32; MAX_NUM_LMAC],
    pub umac_err_id: u32,
    pub txf_iter_data: iwl_txf_iter_data,
    pub type: u8,
    pub subtype: u8,
    pub lmac_major: u32,
    pub lmac_minor: u32,
    pub umac_major: u32,
    pub umac_minor: u32,
    pub fw_ver: },
    pub dump: },

    pub wk: delayed_work,
    pub delay: u32,

    pub seq: u64,
    pub timestamp: },

    pub tpc_enabled: bool,
    pub sar_profiles: [iwl_sar_profile; BIOS_SAR_MAX_PROFILE_NUM],
    pub sar_chain_a_profile: u8,
    pub sar_chain_b_profile: u8,
    pub reduced_power_flags: u8,
    pub geo_profiles: [iwl_geo_profile; BIOS_GEO_MAX_PROFILE_NUM],
    pub geo_bios_source: bios_source,
    pub geo_rev: u32,
    pub geo_num_profiles: u32,
    pub geo_enabled: bool,
    pub ppag_chains: [iwl_ppag_chain; IWL_NUM_CHAIN_LIMITS],
    pub ppag_flags: u32,
    pub ppag_bios_rev: u8,
    pub ppag_bios_source: u8,
    pub sgom_table: iwl_sar_offset_mapping_cmd,
    pub sgom_enabled: bool,
    pub ap_type_cmd: iwl_mcc_allowed_ap_type_cmd,
    pub ap_type_cmd_valid: bool,
    pub uefi_tables_lock_status: u8,
    pub phy_filters: iwl_phy_specific_cfg,
    pub dsm_source: bios_source,
    pub dsm_revision: u8,

    pub dsm_funcs_valid: u32,
    pub dsm_values: [u32; DSM_FUNC_NUM_FUNCS],
    pub wbem_source: bios_source,
    pub wbem_revision: u8,
    pub puncturing_source: bios_source,
    pub puncturing_revision: u8,
    pub bios_puncturing: u32,
}

extern "C" {
    pub fn iwl_fw_runtime_suspend(fwrt: *mut iwl_fw_runtime);
}
extern "C" {
    pub fn iwl_fw_runtime_resume(fwrt: *mut iwl_fw_runtime);
}
extern "C" {
    pub fn iwl_init_paging(fwrt: *mut iwl_fw_runtime, type: iwl_ucode_type) -> c_int;
}
extern "C" {
    pub fn iwl_free_fw_paging(fwrt: *mut iwl_fw_runtime);
}
extern "C" {
    pub fn iwl_get_shared_mem_conf(fwrt: *mut iwl_fw_runtime);
}
extern "C" {
    pub fn iwl_set_soc_latency(fwrt: *mut iwl_fw_runtime) -> c_int;
}
extern "C" {
    pub fn iwl_configure_rxq(fwrt: *mut iwl_fw_runtime) -> c_int;
}
