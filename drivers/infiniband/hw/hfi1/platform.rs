//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/hfi1/platform.h
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
// Copyright(c) 2015, 2016 Intel Corporation.
//
pub const METADATA_TABLE_FIELD_START_SHIFT: c_int = 0;
pub const METADATA_TABLE_FIELD_START_LEN_BITS: c_int = 15;
pub const METADATA_TABLE_FIELD_LEN_SHIFT: c_int = 16;
pub const METADATA_TABLE_FIELD_LEN_LEN_BITS: c_int = 16;
// Header structure
pub const PLATFORM_CONFIG_HEADER_RECORD_IDX_SHIFT: c_int = 0;
pub const PLATFORM_CONFIG_HEADER_RECORD_IDX_LEN_BITS: c_int = 6;
pub const PLATFORM_CONFIG_HEADER_TABLE_LENGTH_SHIFT: c_int = 16;
pub const PLATFORM_CONFIG_HEADER_TABLE_LENGTH_LEN_BITS: c_int = 12;
pub const PLATFORM_CONFIG_HEADER_TABLE_TYPE_SHIFT: c_int = 28;
pub const PLATFORM_CONFIG_HEADER_TABLE_TYPE_LEN_BITS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum platform_config_table_type_encoding {
    PLATFORM_CONFIG_TABLE_RESERVED,
    PLATFORM_CONFIG_SYSTEM_TABLE,
    PLATFORM_CONFIG_PORT_TABLE,
    PLATFORM_CONFIG_RX_PRESET_TABLE,
    PLATFORM_CONFIG_TX_PRESET_TABLE,
    PLATFORM_CONFIG_QSFP_ATTEN_TABLE,
    PLATFORM_CONFIG_VARIABLE_SETTINGS_TABLE,
    PLATFORM_CONFIG_TABLE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum platform_config_system_table_fields {
    SYSTEM_TABLE_RESERVED,
    SYSTEM_TABLE_NODE_STRING,
    SYSTEM_TABLE_SYSTEM_IMAGE_GUID,
    SYSTEM_TABLE_NODE_GUID,
    SYSTEM_TABLE_REVISION,
    SYSTEM_TABLE_VENDOR_OUI,
    SYSTEM_TABLE_META_VERSION,
    SYSTEM_TABLE_DEVICE_ID,
    SYSTEM_TABLE_PARTITION_ENFORCEMENT_CAP,
    SYSTEM_TABLE_QSFP_POWER_CLASS_MAX,
    SYSTEM_TABLE_QSFP_ATTENUATION_DEFAULT_12G,
    SYSTEM_TABLE_QSFP_ATTENUATION_DEFAULT_25G,
    SYSTEM_TABLE_VARIABLE_TABLE_ENTRIES_PER_PORT,
    SYSTEM_TABLE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum platform_config_port_table_fields {
    PORT_TABLE_RESERVED,
    PORT_TABLE_PORT_TYPE,
    PORT_TABLE_LOCAL_ATTEN_12G,
    PORT_TABLE_LOCAL_ATTEN_25G,
    PORT_TABLE_LINK_SPEED_SUPPORTED,
    PORT_TABLE_LINK_WIDTH_SUPPORTED,
    PORT_TABLE_AUTO_LANE_SHEDDING_ENABLED,
    PORT_TABLE_EXTERNAL_LOOPBACK_ALLOWED,
    PORT_TABLE_VL_CAP,
    PORT_TABLE_MTU_CAP,
    PORT_TABLE_TX_LANE_ENABLE_MASK,
    PORT_TABLE_LOCAL_MAX_TIMEOUT,
    PORT_TABLE_REMOTE_ATTEN_12G,
    PORT_TABLE_REMOTE_ATTEN_25G,
    PORT_TABLE_TX_PRESET_IDX_ACTIVE_NO_EQ,
    PORT_TABLE_TX_PRESET_IDX_ACTIVE_EQ,
    PORT_TABLE_RX_PRESET_IDX,
    PORT_TABLE_CABLE_REACH_CLASS,
    PORT_TABLE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum platform_config_rx_preset_table_fields {
    RX_PRESET_TABLE_RESERVED,
    RX_PRESET_TABLE_QSFP_RX_CDR_APPLY,
    RX_PRESET_TABLE_QSFP_RX_EMP_APPLY,
    RX_PRESET_TABLE_QSFP_RX_AMP_APPLY,
    RX_PRESET_TABLE_QSFP_RX_CDR,
    RX_PRESET_TABLE_QSFP_RX_EMP,
    RX_PRESET_TABLE_QSFP_RX_AMP,
    RX_PRESET_TABLE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum platform_config_tx_preset_table_fields {
    TX_PRESET_TABLE_RESERVED,
    TX_PRESET_TABLE_PRECUR,
    TX_PRESET_TABLE_ATTN,
    TX_PRESET_TABLE_POSTCUR,
    TX_PRESET_TABLE_QSFP_TX_CDR_APPLY,
    TX_PRESET_TABLE_QSFP_TX_EQ_APPLY,
    TX_PRESET_TABLE_QSFP_TX_CDR,
    TX_PRESET_TABLE_QSFP_TX_EQ,
    TX_PRESET_TABLE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum platform_config_qsfp_attn_table_fields {
    QSFP_ATTEN_TABLE_RESERVED,
    QSFP_ATTEN_TABLE_TX_PRESET_IDX,
    QSFP_ATTEN_TABLE_RX_PRESET_IDX,
    QSFP_ATTEN_TABLE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum platform_config_variable_settings_table_fields {
    VARIABLE_SETTINGS_TABLE_RESERVED,
    VARIABLE_SETTINGS_TABLE_TX_PRESET_IDX,
    VARIABLE_SETTINGS_TABLE_RX_PRESET_IDX,
    VARIABLE_SETTINGS_TABLE_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct platform_config {
    pub size: usize,
    pub data: *const u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct platform_config_data {
    pub table: *mut u32,
    pub table_metadata: *mut u32,
    pub num_table: u32,
}

//
// This struct acts as a quick reference into the platform_data binary image
// and is populated by parse_platform_config(...) depending on the specific
// META_VERSION
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct platform_config_cache {
    pub cache_valid: u8,
    pub config_tables: [platform_config_data; PLATFORM_CONFIG_TABLE_MAX],
}

// This section defines default values and encodings for the
// fields defined for each table above
//
// =====================================================
// System table encodings
// =====================================================
//
pub const PLATFORM_CONFIG_MAGIC_NUM: c_uint = 0x3d4f5041;
pub const PLATFORM_CONFIG_MAGIC_NUMBER_LEN: c_int = 4;
//
// These power classes are the same as defined in SFF 8636 spec rev 2.4
// describing byte 129 in table 6-16, except enumerated in a different order
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum platform_config_qsfp_power_class_encoding {
    QSFP_POWER_CLASS_1 = 1,
    QSFP_POWER_CLASS_2,
    QSFP_POWER_CLASS_3,
    QSFP_POWER_CLASS_4,
    QSFP_POWER_CLASS_5,
    QSFP_POWER_CLASS_6,
    QSFP_POWER_CLASS_7
}

//
// ====================================================
// Port table encodings
// ====================================================
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum platform_config_port_type_encoding {
    PORT_TYPE_UNKNOWN,
    PORT_TYPE_DISCONNECTED,
    PORT_TYPE_FIXED,
    PORT_TYPE_VARIABLE,
    PORT_TYPE_QSFP,
    PORT_TYPE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum platform_config_link_speed_supported_encoding {
    LINK_SPEED_SUPP_12G = 1,
    LINK_SPEED_SUPP_25G,
    LINK_SPEED_SUPP_12G_25G,
    LINK_SPEED_SUPP_MAX
}

//
// This is a subset (not strict) of the link downgrades
// supported. The link downgrades supported are expected
// to be supplied to the driver by another entity such as
// the fabric manager
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum platform_config_link_width_supported_encoding {
    LINK_WIDTH_SUPP_1X = 1,
    LINK_WIDTH_SUPP_2X,
    LINK_WIDTH_SUPP_2X_1X,
    LINK_WIDTH_SUPP_3X,
    LINK_WIDTH_SUPP_3X_1X,
    LINK_WIDTH_SUPP_3X_2X,
    LINK_WIDTH_SUPP_3X_2X_1X,
    LINK_WIDTH_SUPP_4X,
    LINK_WIDTH_SUPP_4X_1X,
    LINK_WIDTH_SUPP_4X_2X,
    LINK_WIDTH_SUPP_4X_2X_1X,
    LINK_WIDTH_SUPP_4X_3X,
    LINK_WIDTH_SUPP_4X_3X_1X,
    LINK_WIDTH_SUPP_4X_3X_2X,
    LINK_WIDTH_SUPP_4X_3X_2X_1X,
    LINK_WIDTH_SUPP_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum platform_config_virtual_lane_capability_encoding {
    VL_CAP_VL0 = 1,
    VL_CAP_VL0_1,
    VL_CAP_VL0_2,
    VL_CAP_VL0_3,
    VL_CAP_VL0_4,
    VL_CAP_VL0_5,
    VL_CAP_VL0_6,
    VL_CAP_VL0_7,
    VL_CAP_VL0_8,
    VL_CAP_VL0_9,
    VL_CAP_VL0_10,
    VL_CAP_VL0_11,
    VL_CAP_VL0_12,
    VL_CAP_VL0_13,
    VL_CAP_VL0_14,
    VL_CAP_MAX
}

// Max MTU
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum platform_config_mtu_capability_encoding {
    MTU_CAP_256   = 1,
    MTU_CAP_512   = 2,
    MTU_CAP_1024  = 3,
    MTU_CAP_2048  = 4,
    MTU_CAP_4096  = 5,
    MTU_CAP_8192  = 6,
    MTU_CAP_10240 = 7
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum platform_config_local_max_timeout_encoding {
    LOCAL_MAX_TIMEOUT_10_MS = 1,
    LOCAL_MAX_TIMEOUT_100_MS,
    LOCAL_MAX_TIMEOUT_1_S,
    LOCAL_MAX_TIMEOUT_10_S,
    LOCAL_MAX_TIMEOUT_100_S,
    LOCAL_MAX_TIMEOUT_1000_S
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum link_tuning_encoding {
    OPA_PASSIVE_TUNING,
    OPA_ACTIVE_TUNING,
    OPA_UNKNOWN_TUNING
}

//
// Shifts and masks for the link SI tuning values stuffed into the ASIC scratch
// registers for integrated platforms
//
pub const PORT0_PORT_TYPE_SHIFT: c_int = 0;
pub const PORT0_LOCAL_ATTEN_SHIFT: c_int = 4;
pub const PORT0_REMOTE_ATTEN_SHIFT: c_int = 10;
pub const PORT0_DEFAULT_ATTEN_SHIFT: c_int = 32;
pub const PORT1_PORT_TYPE_SHIFT: c_int = 16;
pub const PORT1_LOCAL_ATTEN_SHIFT: c_int = 20;
pub const PORT1_REMOTE_ATTEN_SHIFT: c_int = 26;
pub const PORT1_DEFAULT_ATTEN_SHIFT: c_int = 40;
pub const PORT0_PORT_TYPE_MASK: c_uint = 0xFUL;
pub const PORT0_LOCAL_ATTEN_MASK: c_uint = 0x3FUL;
pub const PORT0_REMOTE_ATTEN_MASK: c_uint = 0x3FUL;
pub const PORT0_DEFAULT_ATTEN_MASK: c_uint = 0xFFUL;
pub const PORT1_PORT_TYPE_MASK: c_uint = 0xFUL;
pub const PORT1_LOCAL_ATTEN_MASK: c_uint = 0x3FUL;
pub const PORT1_REMOTE_ATTEN_MASK: c_uint = 0x3FUL;
pub const PORT1_DEFAULT_ATTEN_MASK: c_uint = 0xFFUL;

pub const QSFP_MAX_POWER_SHIFT: c_int = 0;
pub const TX_NO_EQ_SHIFT: c_int = 4;
pub const TX_EQ_SHIFT: c_int = 25;
pub const RX_SHIFT: c_int = 46;
pub const QSFP_MAX_POWER_MASK: c_uint = 0xFUL;
pub const TX_NO_EQ_MASK: c_uint = 0x1FFFFFUL;
pub const TX_EQ_MASK: c_uint = 0x1FFFFFUL;
pub const RX_MASK: c_uint = 0xFFFFUL;

pub const TX_PRECUR_SHIFT: c_int = 0;
pub const TX_ATTN_SHIFT: c_int = 4;
pub const QSFP_TX_CDR_APPLY_SHIFT: c_int = 9;
pub const QSFP_TX_EQ_APPLY_SHIFT: c_int = 10;
pub const QSFP_TX_CDR_SHIFT: c_int = 11;
pub const QSFP_TX_EQ_SHIFT: c_int = 12;
pub const TX_POSTCUR_SHIFT: c_int = 16;
pub const TX_PRECUR_MASK: c_uint = 0xFUL;
pub const TX_ATTN_MASK: c_uint = 0x1FUL;
pub const QSFP_TX_CDR_APPLY_MASK: c_uint = 0x1UL;
pub const QSFP_TX_EQ_APPLY_MASK: c_uint = 0x1UL;
pub const QSFP_TX_CDR_MASK: c_uint = 0x1UL;
pub const QSFP_TX_EQ_MASK: c_uint = 0xFUL;
pub const TX_POSTCUR_MASK: c_uint = 0x1FUL;

pub const QSFP_RX_CDR_APPLY_SHIFT: c_int = 0;
pub const QSFP_RX_EMP_APPLY_SHIFT: c_int = 1;
pub const QSFP_RX_AMP_APPLY_SHIFT: c_int = 2;
pub const QSFP_RX_CDR_SHIFT: c_int = 3;
pub const QSFP_RX_EMP_SHIFT: c_int = 4;
pub const QSFP_RX_AMP_SHIFT: c_int = 8;
pub const QSFP_RX_CDR_APPLY_MASK: c_uint = 0x1UL;
pub const QSFP_RX_EMP_APPLY_MASK: c_uint = 0x1UL;
pub const QSFP_RX_AMP_APPLY_MASK: c_uint = 0x1UL;
pub const QSFP_RX_CDR_MASK: c_uint = 0x1UL;
pub const QSFP_RX_EMP_MASK: c_uint = 0xFUL;
pub const QSFP_RX_AMP_MASK: c_uint = 0x3UL;

pub const BITMAP_VERSION: c_int = 1;
pub const BITMAP_VERSION_SHIFT: c_int = 44;
pub const BITMAP_VERSION_MASK: c_uint = 0xFUL;

pub const CHECKSUM_SHIFT: c_int = 48;
pub const CHECKSUM_MASK: c_uint = 0xFFFFUL;

// platform.c
extern "C" {
    pub fn get_platform_config(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn free_platform_config(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn get_port_type(ppd: *mut hfi1_pportdata);
}
extern "C" {
    pub fn set_qsfp_tx(ppd: *mut hfi1_pportdata, on: c_int) -> c_int;
}
extern "C" {
    pub fn tune_serdes(ppd: *mut hfi1_pportdata);
}
