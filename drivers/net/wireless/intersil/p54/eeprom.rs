//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intersil/p54/eeprom.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// eeprom specific definitions for mac80211 Prism54 drivers
//
// Copyright (c) 2006, Michael Wu <flamingice@sourmilk.net>
// Copyright (c) 2007-2009, Christian Lamparter <chunkeey@web.de>
//
// Based on:
// - the islsm (softmac prism54) driver, which is:
// Copyright 2004-2006 Jean-Baptiste Note <jbnote@gmail.com>, et al.
//
// - LMAC API interface header file for STLC4560 (lmac_longbow.h)
// Copyright (C) 2007 Conexant Systems, Inc.
//
// - islmvc driver
// Copyright (C) 2001 Intersil Americas Inc.
//
// PDA defines are Copyright (C) 2005 Nokia Corporation (taken from islsm_pda.h)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pda_entry {
    pub /: *mut *mut __le16 len; / includes both code and data,
    pub code: __le16,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eeprom_pda_wrap {
    pub magic: __le32,
    pub pad: __le16,
    pub len: __le16,
    pub arm_opcode: __le32,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_iq_autocal_entry {
    pub iq_param: [__le16; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pda_iq_autocal_entry {
    pub freq: __le16,
    pub params: p54_iq_autocal_entry,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pda_channel_output_limit {
    pub freq: __le16,
    pub val_bpsk: u8,
    pub val_qpsk: u8,
    pub val_16qam: u8,
    pub val_64qam: u8,
    pub rate_set_mask: u8,
    pub rate_set_size: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pda_channel_output_limit_point_longbow {
    pub val_bpsk: __le16,
    pub val_qpsk: __le16,
    pub val_16qam: __le16,
    pub val_64qam: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pda_channel_output_limit_longbow {
    pub freq: __le16,
    pub point: [pda_channel_output_limit_point_longbow; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pda_pa_curve_data_sample_rev0 {
    pub rf_power: u8,
    pub pa_detector: u8,
    pub pcv: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pda_pa_curve_data_sample_rev1 {
    pub rf_power: u8,
    pub pa_detector: u8,
    pub data_barker: u8,
    pub data_bpsk: u8,
    pub data_qpsk: u8,
    pub data_16qam: u8,
    pub data_64qam: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pda_pa_curve_data {
    pub cal_method_rev: u8,
    pub channels: u8,
    pub points_per_channel: u8,
    pub padding: u8,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pda_rssi_cal_ext_entry {
    pub freq: __le16,
    pub mul: __le16,
    pub add: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pda_rssi_cal_entry {
    pub mul: __le16,
    pub add: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pda_country {
    pub regdomain: u8,
    pub alpha2: [u8; 2],
    pub flags: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pda_antenna_gain {
    pub /: *mut *mut u8 gain_5GHz; / 0.25 dBi units,
    pub /: *mut *mut u8 gain_2GHz; / 0.25 dBi units,
    pub antenna): } __packed,,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pda_custom_wrapper {
    pub entries: __le16,
    pub entry_size: __le16,
    pub offset: __le16,
    pub len: __le16,
    pub data: [u8; ],
    pub __packed: },
//
// this defines the PDR codes used to build PDAs as defined in document
// number 553155. The current implementation mirrors version 1.1 of the
// document and lists only PDRs supported by the ARM platform.
//
// common and choice range (0x0000 - 0x0fff)
pub const PDR_END: c_uint = 0x0000;
pub const PDR_MANUFACTURING_PART_NUMBER: c_uint = 0x0001;
pub const PDR_PDA_VERSION: c_uint = 0x0002;
pub const PDR_NIC_SERIAL_NUMBER: c_uint = 0x0003;
pub const PDR_NIC_RAM_SIZE: c_uint = 0x0005;
pub const PDR_RFMODEM_SUP_RANGE: c_uint = 0x0006;
pub const PDR_PRISM_MAC_SUP_RANGE: c_uint = 0x0007;
pub const PDR_NIC_ID: c_uint = 0x0008;
pub const PDR_MAC_ADDRESS: c_uint = 0x0101;
pub const PDR_REGULATORY_DOMAIN_LIST: c_uint = 0x0103 /* obsolete */;
pub const PDR_ALLOWED_CHAN_SET: c_uint = 0x0104;
pub const PDR_DEFAULT_CHAN: c_uint = 0x0105;
pub const PDR_TEMPERATURE_TYPE: c_uint = 0x0107;
pub const PDR_IFR_SETTING: c_uint = 0x0200;
pub const PDR_RFR_SETTING: c_uint = 0x0201;
pub const PDR_3861_BASELINE_REG_SETTINGS: c_uint = 0x0202;
pub const PDR_3861_SHADOW_REG_SETTINGS: c_uint = 0x0203;
pub const PDR_3861_IFRF_REG_SETTINGS: c_uint = 0x0204;
pub const PDR_3861_CHAN_CALIB_SET_POINTS: c_uint = 0x0300;
pub const PDR_3861_CHAN_CALIB_INTEGRATOR: c_uint = 0x0301;
pub const PDR_3842_PRISM_II_NIC_CONFIG: c_uint = 0x0400;
pub const PDR_PRISM_USB_ID: c_uint = 0x0401;
pub const PDR_PRISM_PCI_ID: c_uint = 0x0402;
pub const PDR_PRISM_PCI_IF_CONFIG: c_uint = 0x0403;
pub const PDR_PRISM_PCI_PM_CONFIG: c_uint = 0x0404;
pub const PDR_3861_MF_TEST_CHAN_SET_POINTS: c_uint = 0x0900;
pub const PDR_3861_MF_TEST_CHAN_INTEGRATORS: c_uint = 0x0901;
// ARM range (0x1000 - 0x1fff)
pub const PDR_COUNTRY_INFORMATION: c_uint = 0x1000 /* obsolete */;
pub const PDR_INTERFACE_LIST: c_uint = 0x1001;
pub const PDR_HARDWARE_PLATFORM_COMPONENT_ID: c_uint = 0x1002;
pub const PDR_OEM_NAME: c_uint = 0x1003;
pub const PDR_PRODUCT_NAME: c_uint = 0x1004;
pub const PDR_UTF8_OEM_NAME: c_uint = 0x1005;
pub const PDR_UTF8_PRODUCT_NAME: c_uint = 0x1006;
pub const PDR_COUNTRY_LIST: c_uint = 0x1007;
pub const PDR_DEFAULT_COUNTRY: c_uint = 0x1008;
pub const PDR_ANTENNA_GAIN: c_uint = 0x1100;
pub const PDR_PRISM_INDIGO_PA_CALIBRATION_DATA: c_uint = 0x1901;
pub const PDR_RSSI_LINEAR_APPROXIMATION: c_uint = 0x1902;
pub const PDR_PRISM_PA_CAL_OUTPUT_POWER_LIMITS: c_uint = 0x1903;
pub const PDR_PRISM_PA_CAL_CURVE_DATA: c_uint = 0x1904;
pub const PDR_RSSI_LINEAR_APPROXIMATION_DUAL_BAND: c_uint = 0x1905;
pub const PDR_PRISM_ZIF_TX_IQ_CALIBRATION: c_uint = 0x1906;
pub const PDR_REGULATORY_POWER_LIMITS: c_uint = 0x1907;
pub const PDR_RSSI_LINEAR_APPROXIMATION_EXTENDED: c_uint = 0x1908;
pub const PDR_RADIATED_TRANSMISSION_CORRECTION: c_uint = 0x1909;
pub const PDR_PRISM_TX_IQ_CALIBRATION: c_uint = 0x190a;
// reserved range (0x2000 - 0x7fff)
// customer range (0x8000 - 0xffff)
pub const PDR_BASEBAND_REGISTERS: c_uint = 0x8000;
pub const PDR_PER_CHANNEL_BASEBAND_REGISTERS: c_uint = 0x8001;
// used by our modificated eeprom image
pub const PDR_RSSI_LINEAR_APPROXIMATION_CUSTOM: c_uint = 0xDEAD;
pub const PDR_RSSI_LINEAR_APPROXIMATION_CUSTOMV2: c_uint = 0xCAFF;
pub const PDR_PRISM_PA_CAL_OUTPUT_POWER_LIMITS_CUSTOM: c_uint = 0xBEEF;
pub const PDR_PRISM_PA_CAL_CURVE_DATA_CUSTOM: c_uint = 0xB05D;
// Interface Definitions
pub const PDR_INTERFACE_ROLE_SERVER: c_uint = 0x0000;
pub const PDR_INTERFACE_ROLE_CLIENT: c_uint = 0x0001;
// PDR definitions for default country & country list
pub const PDR_COUNTRY_CERT_CODE: c_uint = 0x80;
pub const PDR_COUNTRY_CERT_CODE_REAL: c_uint = 0x00;
pub const PDR_COUNTRY_CERT_CODE_PSEUDO: c_uint = 0x80;
pub const PDR_COUNTRY_CERT_BAND: c_uint = 0x40;
pub const PDR_COUNTRY_CERT_BAND_2GHZ: c_uint = 0x00;
pub const PDR_COUNTRY_CERT_BAND_5GHZ: c_uint = 0x40;
pub const PDR_COUNTRY_CERT_IODOOR: c_uint = 0x30;
pub const PDR_COUNTRY_CERT_IODOOR_BOTH: c_uint = 0x00;
pub const PDR_COUNTRY_CERT_IODOOR_INDOOR: c_uint = 0x20;
pub const PDR_COUNTRY_CERT_IODOOR_OUTDOOR: c_uint = 0x30;
pub const PDR_COUNTRY_CERT_INDEX: c_uint = 0x0f;
// Specific LMAC FW/HW variant definitions
pub const PDR_SYNTH_FRONTEND_MASK: c_uint = 0x0007;
pub const PDR_SYNTH_FRONTEND_DUETTE3: c_uint = 0x0001;
pub const PDR_SYNTH_FRONTEND_DUETTE2: c_uint = 0x0002;
pub const PDR_SYNTH_FRONTEND_FRISBEE: c_uint = 0x0003;
pub const PDR_SYNTH_FRONTEND_XBOW: c_uint = 0x0004;
pub const PDR_SYNTH_FRONTEND_LONGBOW: c_uint = 0x0005;
pub const PDR_SYNTH_IQ_CAL_MASK: c_uint = 0x0018;
pub const PDR_SYNTH_IQ_CAL_PA_DETECTOR: c_uint = 0x0000;
pub const PDR_SYNTH_IQ_CAL_DISABLED: c_uint = 0x0008;
pub const PDR_SYNTH_IQ_CAL_ZIF: c_uint = 0x0010;
pub const PDR_SYNTH_FAA_SWITCH_MASK: c_uint = 0x0020;
pub const PDR_SYNTH_FAA_SWITCH_ENABLED: c_uint = 0x0020;
pub const PDR_SYNTH_24_GHZ_MASK: c_uint = 0x0040;
pub const PDR_SYNTH_24_GHZ_DISABLED: c_uint = 0x0040;
pub const PDR_SYNTH_5_GHZ_MASK: c_uint = 0x0080;
pub const PDR_SYNTH_5_GHZ_DISABLED: c_uint = 0x0080;
pub const PDR_SYNTH_RX_DIV_MASK: c_uint = 0x0100;
pub const PDR_SYNTH_RX_DIV_SUPPORTED: c_uint = 0x0100;
pub const PDR_SYNTH_TX_DIV_MASK: c_uint = 0x0200;
pub const PDR_SYNTH_TX_DIV_SUPPORTED: c_uint = 0x0200;
pub const PDR_SYNTH_ASM_MASK: c_uint = 0x0400;
pub const PDR_SYNTH_ASM_XSWON: c_uint = 0x0400;
