//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw89/acpi.h
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
// Copyright(c) 2021-2023  Realtek Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_acpi_data {
    pub len: u32,
    pub __counted_by(len): u8 buf[],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_acpi_dsm_func {
    RTW89_ACPI_DSM_FUNC_IDN_BAND_SUP = 2,
    RTW89_ACPI_DSM_FUNC_6G_DIS = 3,
    RTW89_ACPI_DSM_FUNC_6G_BP = 4,
    RTW89_ACPI_DSM_FUNC_TAS_EN = 5,
    RTW89_ACPI_DSM_FUNC_UNII4_SUP = 6,
    RTW89_ACPI_DSM_FUNC_6GHZ_SP_SUP = 7,
    RTW89_ACPI_DSM_FUNC_REG_RULES_EN = 10,
    RTW89_ACPI_DSM_FUNC_6GHZ_VLP_SUP = 11,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_acpi_conf_unii4 {
    RTW89_ACPI_CONF_UNII4_US = BIT(0),
    RTW89_ACPI_CONF_UNII4_CA = BIT(1),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_acpi_policy_mode {
    RTW89_ACPI_POLICY_BLOCK = 0,
    RTW89_ACPI_POLICY_ALLOW = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_acpi_conf_tas {
    RTW89_ACPI_CONF_TAS_US = BIT(0),
    RTW89_ACPI_CONF_TAS_CA = BIT(1),
    RTW89_ACPI_CONF_TAS_KR = BIT(2),
    RTW89_ACPI_CONF_TAS_OTHERS = BIT(7),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_acpi_country_code {
// below are allowed:
// * ISO alpha2 country code
// * EU for countries in Europe
//
    pub alpha2: [c_char; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_acpi_policy_6ghz {
    pub signature: [u8; 3],
    pub rsvd: u8,
    pub policy_mode: u8,
    pub country_count: u8,
    pub __counted_by(country_count): rtw89_acpi_country_code country_list[],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_acpi_conf_6ghz_sp {
    RTW89_ACPI_CONF_6GHZ_SP_US = BIT(0),
    RTW89_ACPI_CONF_6GHZ_SP_CA = BIT(1),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_acpi_policy_6ghz_sp {
    pub signature: [u8; 4],
    pub revision: u8,
    pub override: u8,
    pub conf: u8,
    pub rsvd: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_acpi_conf_6ghz_vlp {
    RTW89_ACPI_CONF_6GHZ_VLP_US = BIT(0),
    RTW89_ACPI_CONF_6GHZ_VLP_CA = BIT(1),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_acpi_policy_6ghz_vlp {
    pub signature: [u8; 4],
    pub revision: u8,
    pub override: u8,
    pub conf: u8,
    pub rsvd: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_acpi_policy_tas {
    pub signature: [u8; 4],
    pub revision: u8,
    pub enable: u8,
    pub enabled_countries: u8,
    pub rsvd: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_acpi_conf_reg_rules {
    RTW89_ACPI_CONF_REG_RULE_REGD_UK = BIT(0),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_acpi_policy_reg_rules {
    pub signature: [u8; 4],
    pub revision: u8,
    pub conf: u8,
    pub rsvd: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_acpi_dsm_result {
    pub value: u8,
// caller needs to free it after using
    pub policy_6ghz: *mut rtw89_acpi_policy_6ghz,
    pub policy_6ghz_sp: *mut rtw89_acpi_policy_6ghz_sp,
    pub policy_6ghz_vlp: *mut rtw89_acpi_policy_6ghz_vlp,
    pub policy_tas: *mut rtw89_acpi_policy_tas,
    pub policy_reg_rules: *mut rtw89_acpi_policy_reg_rules,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_acpi_rtag_result {
    pub tag: [u8; 4],
    pub revision: u8,
    pub domain: __le32,
    pub ant_gain_table: [u8; RTW89_ANT_GAIN_CHAIN_NUM][RTW89_ANT_GAIN_SUBBAND_NR],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_acpi_sar_cid {
    RTW89_ACPI_SAR_CID_HP = 0x5048,
    RTW89_ACPI_SAR_CID_RT = 0x5452,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_acpi_sar_rev {
    RTW89_ACPI_SAR_REV_LEGACY = 1,
    RTW89_ACPI_SAR_REV_HAS_6GHZ = 2,
}

pub const RTW89_ACPI_SAR_ANT_NR_STD: c_int = 4;
pub const RTW89_ACPI_SAR_ANT_NR_SML: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_acpi_sar_std_legacy {
    pub v: [u8; RTW89_ACPI_SAR_ANT_NR_STD][RTW89_ACPI_SAR_SUBBAND_NR_LEGACY],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_acpi_sar_std_has_6ghz {
    pub v: [u8; RTW89_ACPI_SAR_ANT_NR_STD][RTW89_ACPI_SAR_SUBBAND_NR_HAS_6GHZ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_acpi_sar_sml_legacy {
    pub v: [u8; RTW89_ACPI_SAR_ANT_NR_SML][RTW89_ACPI_SAR_SUBBAND_NR_LEGACY],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_acpi_sar_sml_has_6ghz {
    pub v: [u8; RTW89_ACPI_SAR_ANT_NR_SML][RTW89_ACPI_SAR_SUBBAND_NR_HAS_6GHZ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_acpi_static_sar_hdr {
    pub cid: __le16,
    pub rev: u8,
    pub content: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_acpi_dynamic_sar_hdr {
    pub cid: __le16,
    pub rev: u8,
    pub cnt: u8,
    pub content: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_acpi_sar_identifier {
    pub cid: rtw89_acpi_sar_cid,
    pub rev: rtw89_acpi_sar_rev,
    pub size: u8,
}

// for rtw89_acpi_sar_identifier::size

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_acpi_sar_recognition {
    pub id: rtw89_acpi_sar_identifier,
    pub geo: *const rtw89_acpi_geo_sar_handler,
    pub rfpath): *mut *mut u8 (rfpath_to_antidx)(enum rtw89_rf_path,
    pub v): *mut *mut s16 (normalize)(u8,
    pub ent): *mut rtw89_sar_entry_from_acpi,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_acpi_geo_sar_hp_val {
    pub max: u8,
    pub delta: [i8; RTW89_ACPI_SAR_ANT_NR_STD],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_acpi_geo_sar_hp_legacy_entry {
    pub val_2ghz: rtw89_acpi_geo_sar_hp_val,
    pub val_5ghz: rtw89_acpi_geo_sar_hp_val,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_acpi_geo_sar_hp_has_6ghz_entry {
    pub val_2ghz: rtw89_acpi_geo_sar_hp_val,
    pub val_5ghz: rtw89_acpi_geo_sar_hp_val,
    pub val_6ghz: rtw89_acpi_geo_sar_hp_val,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_acpi_geo_sar_regd_hp {
    RTW89_ACPI_GEO_SAR_REGD_HP_FCC = 0,
    RTW89_ACPI_GEO_SAR_REGD_HP_ETSI = 1,
    RTW89_ACPI_GEO_SAR_REGD_HP_WW = 2,

    RTW89_ACPI_GEO_SAR_REGD_NR_HP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_acpi_geo_sar_hp_legacy {
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_acpi_geo_sar_hp_has_6ghz {
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_acpi_geo_sar_rt_val {
    pub max: u8,
    pub delta: i8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_acpi_geo_sar_rt_legacy_entry {
    pub val_2ghz: rtw89_acpi_geo_sar_rt_val,
    pub val_5ghz: rtw89_acpi_geo_sar_rt_val,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_acpi_geo_sar_rt_has_6ghz_entry {
    pub val_2ghz: rtw89_acpi_geo_sar_rt_val,
    pub val_5ghz: rtw89_acpi_geo_sar_rt_val,
    pub val_6ghz: rtw89_acpi_geo_sar_rt_val,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_acpi_geo_sar_regd_rt {
    RTW89_ACPI_GEO_SAR_REGD_RT_FCC = 0,
    RTW89_ACPI_GEO_SAR_REGD_RT_ETSI = 1,
    RTW89_ACPI_GEO_SAR_REGD_RT_MKK = 2,
    RTW89_ACPI_GEO_SAR_REGD_RT_IC = 3,
    RTW89_ACPI_GEO_SAR_REGD_RT_KCC = 4,
    RTW89_ACPI_GEO_SAR_REGD_RT_WW = 5,

    RTW89_ACPI_GEO_SAR_REGD_NR_RT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_acpi_geo_sar_rt_legacy {
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_acpi_geo_sar_rt_has_6ghz {
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_acpi_geo_sar_handler {
    pub data_size: u8,
    pub ent): *mut rtw89_sar_entry_from_acpi,
}

// for rtw89_acpi_geo_sar_handler::data_size

