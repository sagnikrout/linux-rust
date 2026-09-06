//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/acpi.h
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
// Copyright (C) 2018-2023, 2025-2026 Intel Corporation
//

// Macro flag: #define __iwl_fw_acpi__

pub const ACPI_SAR_PROFILE_NUM: c_int = 4;
pub const ACPI_NUM_GEO_PROFILES: c_int = 3;
pub const ACPI_NUM_GEO_PROFILES_REV3: c_int = 8;
pub const ACPI_GEO_PER_CHAIN_SIZE: c_int = 3;
pub const ACPI_SAR_NUM_CHAINS_REV0: c_int = 2;
pub const ACPI_SAR_NUM_CHAINS_REV1: c_int = 2;
pub const ACPI_SAR_NUM_CHAINS_REV2: c_int = 4;
pub const ACPI_SAR_NUM_SUB_BANDS_REV0: c_int = 5;
pub const ACPI_SAR_NUM_SUB_BANDS_REV1: c_int = 11;
pub const ACPI_SAR_NUM_SUB_BANDS_REV2: c_int = 11;
pub const ACPI_SAR_NUM_SUB_BANDS_REV3: c_int = 12;

// revision 0 and 1 are identical, except for the semantics in the FW
pub const ACPI_GEO_NUM_BANDS_REV0: c_int = 2;
pub const ACPI_GEO_NUM_BANDS_REV2: c_int = 3;
pub const ACPI_GEO_NUM_BANDS_REV4: c_int = 4;
pub const ACPI_WRDD_WIFI_DATA_SIZE: c_int = 2;
pub const ACPI_SPLC_WIFI_DATA_SIZE: c_int = 2;
pub const ACPI_ECKV_WIFI_DATA_SIZE: c_int = 2;
//
// One element for domain type,
// and one for enablement of Wi-Fi 320MHz per MCC
//
pub const ACPI_WBEM_WIFI_DATA_SIZE: c_int = 2;
//
// One element for domain type,
// and one for DSBR response data
//
pub const ACPI_DSBR_WIFI_DATA_SIZE: c_int = 2;
pub const ACPI_DSBR_WIFI_DATA_REV: c_int = 1;
//
// One element for domain type,
// and one for the status
//
pub const ACPI_GLAI_WIFI_DATA_SIZE: c_int = 2;
pub const ACPI_GLAI_MAX_STATUS: c_int = 2;
//
// TAS size: 1 elelment for type,
// 1 element for enabled field,
// 1 element for block list size,
// 16 elements for block list array
//

pub const ACPI_PPAG_NUM_CHAINS: c_int = 2;
pub const ACPI_PPAG_NUM_BANDS_V1: c_int = 5;
pub const ACPI_PPAG_NUM_BANDS_V2: c_int = 11;
pub const ACPI_PPAG_NUM_BANDS_V3: c_int = 12;

// used for ACPI PPAG table rev 5

// The Inidcator whether UEFI WIFI GUID tables are locked is read from ACPI
pub const UEFI_WIFI_GUID_UNLOCKED: c_int = 0;
pub const ACPI_DSM_REV: c_int = 0;
pub const DSM_INTERNAL_FUNC_GET_PLAT_INFO: c_int = 1;
// TBD: VPRO is BIT(0) in the result, but what's the result?
pub const DSM_INTERNAL_FUNC_PRODUCT_RESET: c_int = 2;
// DSM_INTERNAL_FUNC_PRODUCT_RESET - product reset (aka "PLDR")
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_dsm_internal_product_reset_cmds {
    DSM_INTERNAL_PLDR_CMD_GET_MODE = 1,
    DSM_INTERNAL_PLDR_CMD_SET_MODE = 2,
    DSM_INTERNAL_PLDR_CMD_GET_STATUS = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_dsm_internal_product_reset_mode {
    DSM_INTERNAL_PLDR_MODE_EN_PROD_RESET	= BIT(0),
    DSM_INTERNAL_PLDR_MODE_EN_WIFI_FLR	= BIT(1),
    DSM_INTERNAL_PLDR_MODE_EN_BT_OFF_ON	= BIT(2),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_dsm_internal_product_reset_cmd {
// cmd is from enum iwl_dsm_internal_product_reset_cmds
    pub cmd: u16,
    pub value: u16,
    pub __packed: },

pub const IWL_ACPI_WBEM_REVISION: c_int = 0;

    pub iwl_fw_runtime: struct,
    pub guid): *const guid_t,
//
// iwl_acpi_get_mcc - read MCC from ACPI, if available
//
// @fwrt: the fw runtime struct
// @mcc: output buffer (3 bytes) that will get the MCC
//
// This function tries to read the current MCC from ACPI if available.
// Return: 0 on success, or a negative error code
//
    pub mcc): *mut *mut int iwl_acpi_get_mcc(struct iwl_fw_runtime fwrt, char,
    pub dflt_pwr_limit): *mut *mut int iwl_acpi_get_pwr_limit(struct iwl_fw_runtime fwrt, u64,
//
// iwl_acpi_get_eckv - read external clock validation from ACPI, if available
//
// @fwrt: the fw runtime struct
// @extl_clk: output var (2 bytes) that will get the clk indication.
//
// This function tries to read the external clock indication
// from ACPI if available.
//
    pub extl_clk): *mut *mut int iwl_acpi_get_eckv(struct iwl_fw_runtime fwrt, u32,
    pub fwrt): *mut int iwl_acpi_get_wrds_table(struct iwl_fw_runtime,
    pub fwrt): *mut int iwl_acpi_get_ewrd_table(struct iwl_fw_runtime,
    pub fwrt): *mut int iwl_acpi_get_wgds_table(struct iwl_fw_runtime,
    pub data): *mut iwl_tas_data,
    pub fwrt): *mut int iwl_acpi_get_ppag_table(struct iwl_fw_runtime,
    pub fwrt): *mut int iwl_acpi_get_phy_filters(struct iwl_fw_runtime,
    pub fwrt): *mut void iwl_acpi_get_guid_lock_status(struct iwl_fw_runtime,
    pub value): *mut iwl_dsm_funcs func, u32,
    pub value): *mut *mut int iwl_acpi_get_wbem(struct iwl_fw_runtime fwrt, u32,
    pub value): *mut *mut int iwl_acpi_get_dsbr(struct iwl_fw_runtime fwrt, u32,

    pub ERR_PTR(-ENOENT): return,
    pub -ENOENT: return,
// dflt_pwr_limit = 0;
    pub 0: return,
    pub -ENOENT: return,
    pub -ENOENT: return,
    pub -ENOENT: return,
    pub 1: return,
    pub -ENOENT: return,
    pub -ENOENT: return,
    pub -ENOENT: return,
    pub -ENOENT: return,
    pub -ENOENT: return,
    pub -ENOENT: return,

