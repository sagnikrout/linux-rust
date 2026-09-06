//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/pcie/iwl-context-info-v2.h
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
// Copyright (C) 2018, 2020-2025 Intel Corporation
//

// Macro flag: #define __iwl_context_info_file_v2_h__

pub const CSR_CTXT_INFO_BOOT_CTRL: c_uint = 0x0;
pub const CSR_CTXT_INFO_ADDR: c_uint = 0x118;
pub const CSR_IML_DATA_ADDR: c_uint = 0x120;
pub const CSR_IML_SIZE_ADDR: c_uint = 0x128;
pub const CSR_IML_RESP_ADDR: c_uint = 0x12c;
pub const UNFRAGMENTED_PNVM_PAYLOADS_NUMBER: c_int = 2;
// Set bit for enabling automatic function boot

// Set bit for initiating function boot

//
// enum iwl_prph_scratch_mtr_format - tfd size configuration
// @IWL_PRPH_MTR_FORMAT_16B: 16 bit tfd
// @IWL_PRPH_MTR_FORMAT_32B: 32 bit tfd
// @IWL_PRPH_MTR_FORMAT_64B: 64 bit tfd
// @IWL_PRPH_MTR_FORMAT_256B: 256 bit tfd
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_prph_scratch_mtr_format {
    IWL_PRPH_MTR_FORMAT_16B = 0x0,
    IWL_PRPH_MTR_FORMAT_32B = 0x40000,
    IWL_PRPH_MTR_FORMAT_64B = 0x80000,
    IWL_PRPH_MTR_FORMAT_256B = 0xC0000,
}

//
// enum iwl_prph_scratch_flags - PRPH scratch control flags
// @IWL_PRPH_SCRATCH_IMR_DEBUG_EN: IMR support for debug
// @IWL_PRPH_SCRATCH_EARLY_DEBUG_EN: enable early debug conf
// @IWL_PRPH_SCRATCH_EDBG_DEST_DRAM: use DRAM, with size allocated
// in hwm config.
// @IWL_PRPH_SCRATCH_EDBG_DEST_INTERNAL: use buffer on SRAM
// @IWL_PRPH_SCRATCH_EDBG_DEST_ST_ARBITER: use st arbiter, mainly for
// multicomm.
// @IWL_PRPH_SCRATCH_EDBG_DEST_TB22DTF: route debug data to SoC HW
// @IWL_PRPH_SCRATCH_RB_SIZE_4K: Use 4K RB size (the default is 2K)
// @IWL_PRPH_SCRATCH_MTR_MODE: format used for completion - 0: for
// completion descriptor, 1 for responses (legacy)
// @IWL_PRPH_SCRATCH_MTR_FORMAT: a mask for the size of the tfd.
// There are 4 optional values: 0: 16 bit, 1: 32 bit, 2: 64 bit,
// 3: 256 bit.
// @IWL_PRPH_SCRATCH_RB_SIZE_EXT_MASK: RB size full information, ignored
// by older firmware versions, so set IWL_PRPH_SCRATCH_RB_SIZE_4K
// appropriately; use the below values for this.
// @IWL_PRPH_SCRATCH_RB_SIZE_EXT_8K: 8kB RB size
// @IWL_PRPH_SCRATCH_RB_SIZE_EXT_12K: 12kB RB size
// @IWL_PRPH_SCRATCH_RB_SIZE_EXT_16K: 16kB RB size
// @IWL_PRPH_SCRATCH_SCU_FORCE_ACTIVE: Indicate fw to set SCU_FORCE_ACTIVE
// upon reset.
// @IWL_PRPH_SCRATCH_TOP_RESET: request TOP reset
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_prph_scratch_flags {
    IWL_PRPH_SCRATCH_IMR_DEBUG_EN		= BIT(1),
    IWL_PRPH_SCRATCH_EARLY_DEBUG_EN		= BIT(4),
    IWL_PRPH_SCRATCH_EDBG_DEST_DRAM		= BIT(8),
    IWL_PRPH_SCRATCH_EDBG_DEST_INTERNAL	= BIT(9),
    IWL_PRPH_SCRATCH_EDBG_DEST_ST_ARBITER	= BIT(10),
    IWL_PRPH_SCRATCH_EDBG_DEST_TB22DTF	= BIT(11),
    IWL_PRPH_SCRATCH_RB_SIZE_4K		= BIT(16),
    IWL_PRPH_SCRATCH_MTR_MODE		= BIT(17),
    IWL_PRPH_SCRATCH_MTR_FORMAT		= BIT(18) | BIT(19),
    IWL_PRPH_SCRATCH_RB_SIZE_EXT_MASK	= 0xf << 20,
    IWL_PRPH_SCRATCH_RB_SIZE_EXT_8K		= 8 << 20,
    IWL_PRPH_SCRATCH_RB_SIZE_EXT_12K	= 9 << 20,
    IWL_PRPH_SCRATCH_RB_SIZE_EXT_16K	= 10 << 20,
    IWL_PRPH_SCRATCH_SCU_FORCE_ACTIVE	= BIT(29),
    IWL_PRPH_SCRATCH_TOP_RESET		= BIT(30),
}

//
// enum iwl_prph_scratch_ext_flags - PRPH scratch control ext flags
// @IWL_PRPH_SCRATCH_EXT_EXT_FSEQ: external FSEQ image provided
// @IWL_PRPH_SCRATCH_EXT_URM_FW: switch to URM mode based on fw setting
// @IWL_PRPH_SCRATCH_EXT_URM_PERM: switch to permanent URM mode
// @IWL_PRPH_SCRATCH_EXT_32KHZ_CLK_VALID: use external 32 KHz clock
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_prph_scratch_ext_flags {
    IWL_PRPH_SCRATCH_EXT_EXT_FSEQ		= BIT(0),
    IWL_PRPH_SCRATCH_EXT_URM_FW		= BIT(4),
    IWL_PRPH_SCRATCH_EXT_URM_PERM		= BIT(5),
    IWL_PRPH_SCRATCH_EXT_32KHZ_CLK_VALID	= BIT(8),
}

//
// struct iwl_prph_scratch_version - version structure
// @mac_id: SKU and revision id
// @version: prph scratch information version id
// @size: the size of the context information in DWs
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_prph_scratch_version {
    pub mac_id: __le16,
    pub version: __le16,
    pub size: __le16,
    pub reserved: __le16,
    pub /: *mut *mut } __packed; / PERIPH_SCRATCH_VERSION_S,
//
// struct iwl_prph_scratch_control - control structure
// @control_flags: context information flags see &enum iwl_prph_scratch_flags
// @control_flags_ext: context information for extended flags,
// see &enum iwl_prph_scratch_ext_flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_prph_scratch_control {
    pub control_flags: __le32,
    pub control_flags_ext: __le32,
    pub /: *mut *mut } __packed; / PERIPH_SCRATCH_CONTROL_S,
//
// struct iwl_prph_scratch_pnvm_cfg - PNVM scratch
// @pnvm_base_addr: PNVM start address
// @pnvm_size: the size of the PNVM image in bytes
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_prph_scratch_pnvm_cfg {
    pub pnvm_base_addr: __le64,
    pub pnvm_size: __le32,
    pub reserved: __le32,
    pub /: *mut *mut } __packed; / PERIPH_SCRATCH_PNVM_CFG_S,
//
// struct iwl_prph_scratch_mem_desc_addr_array - DRAM
// @mem_descs: array of dram addresses.
// Each address is the beginning of a PNVM payload.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_prph_scratch_mem_desc_addr_array {
    pub mem_descs: [__le64; IPC_DRAM_MAP_ENTRY_NUM_MAX],
    pub /: *mut *mut } __packed; / PERIPH_SCRATCH_MEM_DESC_ADDR_ARRAY_S_VER_1,
//
// struct iwl_prph_scratch_hwm_cfg - hwm config
// @hwm_base_addr: hwm start address
// @hwm_size: hwm size in DWs
// @debug_token_config: debug preset
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_prph_scratch_hwm_cfg {
    pub hwm_base_addr: __le64,
    pub hwm_size: __le32,
    pub debug_token_config: __le32,
    pub /: *mut *mut } __packed; / PERIPH_SCRATCH_HWM_CFG_S,
//
// struct iwl_prph_scratch_rbd_cfg - RBDs configuration
// @free_rbd_addr: default queue free RB CB base address
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_prph_scratch_rbd_cfg {
    pub free_rbd_addr: __le64,
    pub reserved: __le32,
    pub /: *mut *mut } __packed; / PERIPH_SCRATCH_RBD_CFG_S,
//
// struct iwl_prph_scratch_uefi_cfg - prph scratch reduce power table
// @base_addr: reduce power table address
// @size: the size of the entire power table image
// @reserved: (reserved)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_prph_scratch_uefi_cfg {
    pub base_addr: __le64,
    pub size: __le32,
    pub reserved: __le32,
    pub /: *mut *mut } __packed; / PERIPH_SCRATCH_UEFI_CFG_S,
//
// struct iwl_prph_scratch_step_cfg - prph scratch step configuration
// @mbx_addr_0: [0:7] revision,
// [8:15] cnvi_to_cnvr length,
// [16:23] cnvr_to_cnvi channel length,
// [24:31] radio1 reserved
// @mbx_addr_1: [0:7] radio2 reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_prph_scratch_step_cfg {
    pub mbx_addr_0: __le32,
    pub mbx_addr_1: __le32,
    pub __packed: },
//
// struct iwl_prph_scratch_ctrl_cfg - prph scratch ctrl and config
// @version: version information of context info and HW
// @control: control flags of FH configurations
// @pnvm_cfg: ror configuration
// @hwm_cfg: hwm configuration
// @rbd_cfg: default RX queue configuration
// @reduce_power_cfg: UEFI power reduction table
// @step_cfg: step configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_prph_scratch_ctrl_cfg {
    pub version: iwl_prph_scratch_version,
    pub control: iwl_prph_scratch_control,
    pub pnvm_cfg: iwl_prph_scratch_pnvm_cfg,
    pub hwm_cfg: iwl_prph_scratch_hwm_cfg,
    pub rbd_cfg: iwl_prph_scratch_rbd_cfg,
    pub reduce_power_cfg: iwl_prph_scratch_uefi_cfg,
    pub step_cfg: iwl_prph_scratch_step_cfg,
    pub /: *mut *mut } __packed; / PERIPH_SCRATCH_CTRL_CFG_S,
pub const IWL_NUM_DRAM_FSEQ_ENTRIES: c_int = 8;
//
// struct iwl_context_info_dram_fseq - images DRAM map (with fseq)
// each entry in the map represents a DRAM chunk of up to 32 KB
// @common: UMAC/LMAC/virtual images
// @fseq_img: FSEQ image DRAM map
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_context_info_dram_fseq {
    pub common: iwl_context_info_dram_nonfseq,
    pub fseq_img: [__le64; IWL_NUM_DRAM_FSEQ_ENTRIES],
    pub /: *mut *mut } __packed; / PERIPH_SCRATCH_DRAM_MAP_S,
//
// struct iwl_prph_scratch - peripheral scratch mapping
// @ctrl_cfg: control and configuration of prph scratch
// @dram: firmware images addresses in DRAM
// @fseq_override: FSEQ override parameters
// @step_analog_params: STEP analog calibration values
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_prph_scratch {
    pub ctrl_cfg: iwl_prph_scratch_ctrl_cfg,
    pub fseq_override: __le32,
    pub step_analog_params: __le32,
    pub reserved: [__le32; 8],
    pub dram: iwl_context_info_dram_fseq,
    pub /: *mut *mut } __packed; / PERIPH_SCRATCH_S,
//
// struct iwl_prph_info - peripheral information
// @boot_stage_mirror: reflects the value in the Boot Stage CSR register
// @ipc_status_mirror: reflects the value in the IPC Status CSR register
// @sleep_notif: indicates the peripheral sleep status
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_prph_info {
    pub boot_stage_mirror: __le32,
    pub ipc_status_mirror: __le32,
    pub sleep_notif: __le32,
    pub reserved: __le32,
    pub /: *mut *mut } __packed; / PERIPH_INFO_S,
//
// struct iwl_context_info_v2 - device INIT configuration
// @version: version of the context information
// @size: size of context information in DWs
// @config: context in which the peripheral would execute - a subset of
// capability csr register published by the peripheral
// @prph_info_base_addr: the peripheral information structure start address
// @cr_head_idx_arr_base_addr: the completion ring head index array
// start address
// @tr_tail_idx_arr_base_addr: the transfer ring tail index array
// start address
// @cr_tail_idx_arr_base_addr: the completion ring tail index array
// start address
// @tr_head_idx_arr_base_addr: the transfer ring head index array
// start address
// @cr_idx_arr_size: number of entries in the completion ring index array
// @tr_idx_arr_size: number of entries in the transfer ring index array
// @mtr_base_addr: the message transfer ring start address
// @mcr_base_addr: the message completion ring start address
// @mtr_size: number of entries which the message transfer ring can hold
// @mcr_size: number of entries which the message completion ring can hold
// @mtr_doorbell_vec: the doorbell vector associated with the message
// transfer ring
// @mcr_doorbell_vec: the doorbell vector associated with the message
// completion ring
// @mtr_msi_vec: the MSI which shall be generated by the peripheral after
// completing a transfer descriptor in the message transfer ring
// @mcr_msi_vec: the MSI which shall be generated by the peripheral after
// completing a completion descriptor in the message completion ring
// @mtr_opt_header_size: the size of the optional header in the transfer
// descriptor associated with the message transfer ring in DWs
// @mtr_opt_footer_size: the size of the optional footer in the transfer
// descriptor associated with the message transfer ring in DWs
// @mcr_opt_header_size: the size of the optional header in the completion
// descriptor associated with the message completion ring in DWs
// @mcr_opt_footer_size: the size of the optional footer in the completion
// descriptor associated with the message completion ring in DWs
// @msg_rings_ctrl_flags: message rings control flags
// @prph_info_msi_vec: the MSI which shall be generated by the peripheral
// after updating the Peripheral Information structure
// @prph_scratch_base_addr: the peripheral scratch structure start address
// @prph_scratch_size: the size of the peripheral scratch structure in DWs
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_context_info_v2 {
    pub version: __le16,
    pub size: __le16,
    pub config: __le32,
    pub prph_info_base_addr: __le64,
    pub cr_head_idx_arr_base_addr: __le64,
    pub tr_tail_idx_arr_base_addr: __le64,
    pub cr_tail_idx_arr_base_addr: __le64,
    pub tr_head_idx_arr_base_addr: __le64,
    pub cr_idx_arr_size: __le16,
    pub tr_idx_arr_size: __le16,
    pub mtr_base_addr: __le64,
    pub mcr_base_addr: __le64,
    pub mtr_size: __le16,
    pub mcr_size: __le16,
    pub mtr_doorbell_vec: __le16,
    pub mcr_doorbell_vec: __le16,
    pub mtr_msi_vec: __le16,
    pub mcr_msi_vec: __le16,
    pub mtr_opt_header_size: u8,
    pub mtr_opt_footer_size: u8,
    pub mcr_opt_header_size: u8,
    pub mcr_opt_footer_size: u8,
    pub msg_rings_ctrl_flags: __le16,
    pub prph_info_msi_vec: __le16,
    pub prph_scratch_base_addr: __le64,
    pub prph_scratch_size: __le32,
    pub reserved: __le32,
    pub /: *mut *mut } __packed; / IPC_CONTEXT_INFO_S,
    pub img): *const fw_img,
    pub trans): *mut void iwl_pcie_ctxt_info_v2_kick(struct iwl_trans,
    pub alive): *mut *mut void iwl_pcie_ctxt_info_v2_free(struct iwl_trans trans, bool,
    pub capa): *const iwl_ucode_capabilities,
    pub capa): *const iwl_ucode_capabilities,
    pub capa): *const iwl_ucode_capabilities,
    pub capa): *const iwl_ucode_capabilities,
