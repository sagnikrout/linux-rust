//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/typec/tipd/tps6598x.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Driver for TI TPS6598x USB Power Delivery controller family
//
// Copyright (C) 2017, Intel Corporation
// Author: Heikki Krogerus <heikki.krogerus@linux.intel.com>
//

// TPS_REG_STATUS bits

pub const TPS_STATUS_CONN_STATE_NO_CONN: c_int = 0;
pub const TPS_STATUS_CONN_STATE_DISABLED: c_int = 1;
pub const TPS_STATUS_CONN_STATE_AUDIO_CONN: c_int = 2;
pub const TPS_STATUS_CONN_STATE_DEBUG_CONN: c_int = 3;
pub const TPS_STATUS_CONN_STATE_NO_CONN_R_A: c_int = 4;
pub const TPS_STATUS_CONN_STATE_RESERVED: c_int = 5;
pub const TPS_STATUS_CONN_STATE_CONN_NO_R_A: c_int = 6;
pub const TPS_STATUS_CONN_STATE_CONN_WITH_R_A: c_int = 7;
pub const TPS_STATUS_PP_SWITCH_STATE_DISABLED: c_int = 0;
pub const TPS_STATUS_PP_SWITCH_STATE_FAULT: c_int = 1;
pub const TPS_STATUS_PP_SWITCH_STATE_OUT: c_int = 2;
pub const TPS_STATUS_PP_SWITCH_STATE_IN: c_int = 3;
pub const TPS_STATUS_POWER_SOURCE_UNKNOWN: c_int = 0;
pub const TPS_STATUS_POWER_SOURCE_VIN_3P3: c_int = 1;
pub const TPS_STATUS_POWER_SOURCE_DEAD_BAT: c_int = 2;
pub const TPS_STATUS_POWER_SOURCE_VBUS: c_int = 3;
pub const TPS_STATUS_VBUS_STATUS_VSAFE0V: c_int = 0;
pub const TPS_STATUS_VBUS_STATUS_VSAFE5V: c_int = 1;
pub const TPS_STATUS_VBUS_STATUS_PD: c_int = 2;
pub const TPS_STATUS_VBUS_STATUS_FAULT: c_int = 3;
pub const TPS_STATUS_USB_HOST_PRESENT_NO: c_int = 0;
pub const TPS_STATUS_USB_HOST_PRESENT_PD_NO_USB: c_int = 1;
pub const TPS_STATUS_USB_HOST_PRESENT_NO_PD: c_int = 2;
pub const TPS_STATUS_USB_HOST_PRESENT_PD_USB: c_int = 3;
pub const TPS_STATUS_LEGACY_NO: c_int = 0;
pub const TPS_STATUS_LEGACY_SINK: c_int = 1;
pub const TPS_STATUS_LEGACY_SOURCE: c_int = 2;
// TPS_REG_INT_* bits

// Apple-specific TPS_REG_INT_* bits

// TPS_REG_SYSTEM_POWER_STATE states
pub const TPS_SYSTEM_POWER_STATE_S0: c_uint = 0x00;
pub const TPS_SYSTEM_POWER_STATE_S3: c_uint = 0x03;
pub const TPS_SYSTEM_POWER_STATE_S4: c_uint = 0x04;
pub const TPS_SYSTEM_POWER_STATE_S5: c_uint = 0x05;
// TPS_REG_POWER_STATUS bits (masks shared by TPS_FIELD_GET accessors and FIELD_PREP)

pub const TPS_POWER_STATUS_TYPEC_CURRENT_USB: c_int = 0;
pub const TPS_POWER_STATUS_TYPEC_CURRENT_1A5: c_int = 1;
pub const TPS_POWER_STATUS_TYPEC_CURRENT_3A0: c_int = 2;
pub const TPS_POWER_STATUS_TYPEC_CURRENT_PD: c_int = 3;
pub const TPS_POWER_STATUS_BC12_STATUS_SDP: c_int = 0;
pub const TPS_POWER_STATUS_BC12_STATUS_CDP: c_int = 2;
pub const TPS_POWER_STATUS_BC12_STATUS_DCP: c_int = 3;
// TPS25750_REG_POWER_STATUS bits

pub const TPS25750_POWER_STATUS_CHARGER_DET_STATUS_DISABLED: c_int = 0;
pub const TPS25750_POWER_STATUS_CHARGER_DET_STATUS_IN_PROGRESS: c_int = 1;
pub const TPS25750_POWER_STATUS_CHARGER_DET_STATUS_NONE: c_int = 2;
pub const TPS25750_POWER_STATUS_CHARGER_DET_STATUS_SPD: c_int = 3;
pub const TPS25750_POWER_STATUS_CHARGER_DET_STATUS_BC_1_2_CPD: c_int = 4;
pub const TPS25750_POWER_STATUS_CHARGER_DET_STATUS_BC_1_2_DPD: c_int = 5;
pub const TPS25750_POWER_STATUS_CHARGER_DET_STATUS_DIV_1_DCP: c_int = 6;
pub const TPS25750_POWER_STATUS_CHARGER_DET_STATUS_DIV_2_DCP: c_int = 7;
pub const TPS25750_POWER_STATUS_CHARGER_DET_STATUS_DIV_3_DCP: c_int = 8;
pub const TPS25750_POWER_STATUS_CHARGER_DET_STATUS_1_2V_DCP: c_int = 9;
// TPS_REG_DATA_STATUS bits

// modified TPS_REG_DATA_STATUS bits for CD321x (and likely also TPS65987DDK)

// Map data status to DP spec assignments

pub const TPS_DATA_STATUS_DP_SPEC_PIN_ASSIGNMENT_E: c_int = 0;

// BOOT STATUS REG

// PD STATUS REG

pub const TPS_PD_STATUS_PORT_TYPE_SINK_SOURCE: c_int = 0;
pub const TPS_PD_STATUS_PORT_TYPE_SINK: c_int = 1;
pub const TPS_PD_STATUS_PORT_TYPE_SOURCE: c_int = 2;
pub const TPS_PD_STATUS_PORT_TYPE_SOURCE_SINK: c_int = 3;
// SLEEP CONF REG

// Start Patch Download Sequence

pub const TPS_PTCS_OUT_BYTES: c_int = 4;
pub const TPS_PTCS_STATUS: c_int = 1;
pub const TPS_PTCS_STATUS_FAIL: c_uint = 0x80;
// Patch Download
pub const TPS_PTCD_OUT_BYTES: c_int = 10;
pub const TPS_PTCD_TRANSFER_STATUS: c_int = 1;
pub const TPS_PTCD_LOADING_STATE: c_int = 2;
pub const TPS_PTCD_LOAD_ERR: c_uint = 0x09;
// Patch Download Complete
pub const TPS_PTCC_OUT_BYTES: c_int = 4;
pub const TPS_PTCC_DEV: c_int = 2;
pub const TPS_PTCC_APP: c_int = 3;
// Version Register

pub const TPS_VERSION_HW_65981_2_6: c_uint = 0x00;
pub const TPS_VERSION_HW_65987_8_DH: c_uint = 0xF7;
pub const TPS_VERSION_HW_65987_8_DK: c_uint = 0xF9;
// Int Event Register length
pub const TPS_65981_2_6_INTEVENT_LEN: c_int = 8;
pub const TPS_65987_8_INTEVENT_LEN: c_int = 11;
