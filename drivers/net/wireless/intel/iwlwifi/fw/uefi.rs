//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/uefi.h
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
// Copyright(c) 2021-2026 Intel Corporation
//

// Macro flag: #define __iwl_fw_uefi__

pub const IWL_SGOM_MAP_SIZE: c_int = 339;
pub const IWL_UATS_MAP_SIZE: c_int = 339;
pub const IWL_UEFI_MIN_WTAS_REVISION: c_int = 1;
pub const IWL_UEFI_MAX_WTAS_REVISION: c_int = 2;
pub const IWL_UEFI_SPLC_REVISION: c_int = 0;
pub const IWL_UEFI_WRDD_REVISION: c_int = 0;
pub const IWL_UEFI_ECKV_REVISION: c_int = 0;
pub const IWL_UEFI_WBEM_REVISION: c_int = 0;
pub const IWL_UEFI_DSM_REVISION: c_int = 4;
pub const IWL_UEFI_PUNCTURING_REVISION: c_int = 0;
pub const IWL_UEFI_DSBR_REVISION: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnvm_sku_package {
    pub rev: u8,
    pub total_size: u32,
    pub n_skus: u8,
    pub reserved: [u32; 2],
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uefi_cnv_wlan_sgom_data {
    pub revision: u8,
    pub 1]: u8 offset_map[IWL_SGOM_MAP_SIZE -,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uefi_cnv_wlan_uats_data {
    pub revision: u8,
    pub 1]: u8 mcc_to_ap_type_map[IWL_UATS_MAP_SIZE -,
    pub __packed: },
// UNEB's layout is identical to UATS's

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uefi_cnv_common_step_data {
    pub revision: u8,
    pub step_mode: u8,
    pub cnvi_eq_channel: u8,
    pub cnvr_eq_channel: u8,
    pub radio1: u8,
    pub radio2: u8,
    pub __packed: },
pub const UEFI_PPAG_SUB_BANDS_NUM_REV4: c_int = 11;
pub const UEFI_PPAG_SUB_BANDS_NUM_REV5: c_int = 12;
pub const UEFI_PPAG_NUM_CHAINS: c_int = 2;
pub const UEFI_SAR_SUB_BANDS_NUM_REV2: c_int = 11;
pub const UEFI_SAR_SUB_BANDS_NUM_REV3: c_int = 12;
pub const UEFI_SAR_MAX_CHAINS_PER_PROFILE: c_int = 4;
pub const UEFI_GEO_NUM_BANDS_REV3: c_int = 3;
pub const UEFI_GEO_NUM_BANDS_REV4: c_int = 4;
//
// struct uefi_cnv_var_wrds - WRDS table as defined in UEFI
//
// @revision: the revision of the table
// @mode: is WRDS enbaled/disabled
// @vals: values for sar profile #1 as an array:
// vals[chain * num_of_subbands + subband] will return the right value.
// num_of_subbands depends on the revision. For revision 3, it is
// %UEFI_SAR_SUB_BANDS_NUM_REV3, for earlier revision, it is
// %UEFI_SAR_SUB_BANDS_NUM_REV2.
// The max number of chains is currently 2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uefi_cnv_var_wrds {
    pub revision: u8,
    pub mode: u32,
    pub vals: [u8; ],
    pub __packed: },

//
// struct uefi_cnv_var_ewrd - EWRD table as defined in UEFI
// @revision: the revision of the table
// @mode: is WRDS enbaled/disabled
// @num_profiles: how many additional profiles we have in this table (0-3)
// @vals: the additional SAR profiles (#2-#4) as an array of SAR profiles.
// A SAR profile is defined the &struct uefi_cnv_var_wrds::vals. The size
// of each profile depends on the number of subbands which depends on the
// revision. This is explained in &struct uefi_cnv_var_wrds.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uefi_cnv_var_ewrd {
    pub revision: u8,
    pub mode: u32,
    pub num_profiles: u32,
    pub vals: [u8; ],
    pub __packed: },

//
// struct uefi_cnv_var_wgds - WGDS table as defined in UEFI
// @revision: the revision of the table
// @num_profiles: the number of geo profiles we have in the table.
// The first 3 are mandatory, and can have up to 8.
// @vals: a per-profile table of the offsets to add to SAR values. This is an
// array of profiles, each profile is an array of
// &struct iwl_geo_profile_band, one for each subband.
// There are %UEFI_GEO_NUM_BANDS_REV3 or %UEFI_GEO_NUM_BANDS_REV4 subbands
// depending on the revision.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uefi_cnv_var_wgds {
    pub revision: u8,
    pub num_profiles: u8,
    pub vals: [u8; ],
    pub __packed: },
// struct iwl_geo_profile_band is 3 bytes-long, but since it is not packed,
// we can't use sizeof()
//

//
// struct uefi_cnv_var_ppag - PPAG table as defined in UEFI
// @revision: the revision of the table
// @ppag_modes: values from &enum iwl_ppag_flags
// @vals: the PPAG values per chain and band as an array.
// vals[chain * num_of_subbands + subband] will return the right value.
// num_of_subbands depends on the revision. For revision 5, it is
// %UEFI_PPAG_SUB_BANDS_NUM_REV5, for earlier revision it is
// %UEFI_PPAG_SUB_BANDS_NUM_REV4.
// the max number of chains is currently 2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uefi_cnv_var_ppag {
    pub revision: u8,
    pub ppag_modes: u32,
    pub vals: [i8; ],
    pub __packed: },

//
// struct uefi_cnv_var_wtas - WTAS tabled as defined in UEFI
// @revision: the revision of the table
// @tas_selection: different options of TAS enablement.
// @black_list_size: the number of defined entried in the black list
// @black_list: a list of countries that are not allowed to use the TAS feature
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uefi_cnv_var_wtas {
    pub revision: u8,
    pub tas_selection: u32,
    pub black_list_size: u8,
    pub black_list: [u16; IWL_WTAS_BLACK_LIST_MAX],
    pub __packed: },
//
// struct uefi_cnv_var_splc - SPLC tabled as defined in UEFI
// @revision: the revision of the table
// @default_pwr_limit: The default maximum power per device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uefi_cnv_var_splc {
    pub revision: u8,
    pub default_pwr_limit: u32,
    pub __packed: },
//
// struct uefi_cnv_var_wrdd - WRDD table as defined in UEFI
// @revision: the revision of the table
// @mcc: country identifier as defined in ISO/IEC 3166-1 Alpha 2 code
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uefi_cnv_var_wrdd {
    pub revision: u8,
    pub mcc: u32,
    pub __packed: },
//
// struct uefi_cnv_var_eckv - ECKV table as defined in UEFI
// @revision: the revision of the table
// @ext_clock_valid: indicates if external 32KHz clock is valid
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uefi_cnv_var_eckv {
    pub revision: u8,
    pub ext_clock_valid: u32,
    pub __packed: },
pub const UEFI_MAX_DSM_FUNCS: c_int = 32;
//
// struct uefi_cnv_var_general_cfg - DSM-like table as defined in UEFI
// @revision: the revision of the table
// @functions: payload of the different DSM functions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uefi_cnv_var_general_cfg {
    pub revision: u8,
    pub functions: [u32; UEFI_MAX_DSM_FUNCS],
    pub __packed: },

//
// struct uefi_cnv_wlan_wbem_data - Bandwidth enablement per MCC as defined
// in UEFI
// @revision: the revision of the table
// @wbem_320mhz_per_mcc: enablement of 320MHz bandwidth per MCC
// bit 0 - if set, 320MHz is enabled for Japan
// bit 1 - if set, 320MHz is enabled for South Korea
// bit 2- 31, Reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uefi_cnv_wlan_wbem_data {
    pub revision: u8,
    pub wbem_320mhz_per_mcc: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_uefi_cnv_puncturing_flags {
    IWL_UEFI_CNV_PUNCTURING_USA_EN_MSK	= BIT(0),
    IWL_UEFI_CNV_PUNCTURING_CANADA_EN_MSK	= BIT(1),
}

//
// struct uefi_cnv_var_puncturing_data - controlling channel
// puncturing for few countries.
// @revision: the revision of the table
// @puncturing: enablement of channel puncturing per mcc
// see &enum iwl_uefi_cnv_puncturing_flags.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uefi_cnv_var_puncturing_data {
    pub revision: u8,
    pub puncturing: u32,
    pub __packed: },
//
// struct uefi_cnv_wlan_dsbr_data - BIOS STEP configuration information
// @revision: the revision of the table
// @config: STEP configuration flags:
// bit 8, switch to URM depending on FW setting
// bit 9, switch to URM
//
// Platform information for STEP configuration/workarounds.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uefi_cnv_wlan_dsbr_data {
    pub revision: u8,
    pub config: u32,
    pub __packed: },
//
// struct uefi_cnv_wpfc_data - BIOS Wi-Fi PHY filter Configuration
// @revision: the revision of the table
// @chains: configuration of each of the chains (a-d)
//
// specific PHY filter configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uefi_cnv_wpfc_data {
    pub revision: u8,
    pub chains: [u32; 4],
    pub __packed: },
//
// This is known to be broken on v4.19 and to work on v5.4.  Until we
// figure out why this is the case and how to make it work, simply
// disable the feature in old kernels.
//

    pub len): *mut *mut *mut void iwl_uefi_get_pnvm(struct iwl_trans trans, size_t,
    pub len): *mut *mut *mut u8 iwl_uefi_get_reduced_power(struct iwl_trans trans, size_t,
    pub sku_id[3]): __le32,
    pub trans): *mut void iwl_uefi_get_step_table(struct iwl_trans,
    pub pnvm_data): *mut u32 tlv_len, struct iwl_pnvm_image,
    pub fwrt): *mut int iwl_uefi_get_wrds_table(struct iwl_fw_runtime,
    pub fwrt): *mut int iwl_uefi_get_ewrd_table(struct iwl_fw_runtime,
    pub fwrt): *mut int iwl_uefi_get_wgds_table(struct iwl_fw_runtime,
    pub fwrt): *mut int iwl_uefi_get_ppag_table(struct iwl_fw_runtime,
    pub data): *mut iwl_tas_data,
    pub dflt_pwr_limit): *mut u64,
    pub mcc): *mut *mut int iwl_uefi_get_mcc(struct iwl_fw_runtime fwrt, char,
    pub extl_clk): *mut *mut int iwl_uefi_get_eckv(struct iwl_fw_runtime fwrt, u32,
    pub value): *mut *mut int iwl_uefi_get_wbem(struct iwl_fw_runtime fwrt, u32,
    pub value): *mut u32,
    pub fwrt): *mut *mut void iwl_uefi_get_sgom_table(struct iwl_trans trans, struct iwl_fw_runtime,
    pub fwrt): *mut iwl_fw_runtime,
    pub fwrt): *mut iwl_fw_runtime,
    pub fwrt): *mut int iwl_uefi_get_puncturing(struct iwl_fw_runtime,
    pub value): *mut *mut int iwl_uefi_get_dsbr(struct iwl_fw_runtime fwrt, u32,
    pub fwrt): *mut int iwl_uefi_get_phy_filters(struct iwl_fw_runtime,

    pub ERR_PTR(-EOPNOTSUPP): return,
    pub -EOPNOTSUPP: return,
    pub ERR_PTR(-EOPNOTSUPP): return,
    pub 0: return,
    pub -ENOENT: return,
    pub -ENOENT: return,
    pub -ENOENT: return,
    pub -ENOENT: return,
    pub -ENOENT: return,
// dflt_pwr_limit = 0;
    pub 0: return,
    pub -ENOENT: return,
    pub -ENOENT: return,
    pub -ENOENT: return,
    pub -ENOENT: return,
    pub 0: return,
    pub -ENOENT: return,
    pub -ENOENT: return,

