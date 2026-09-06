//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/hfi1/qsfp.h
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
// QSFP support common definitions, for hfi driver
pub const QSFP_DEV: c_uint = 0xA0;
pub const QSFP_PWR_LAG_MSEC: c_int = 2000;
pub const QSFP_MODPRS_LAG_MSEC: c_int = 20;
// 128 byte pages, per SFF 8636 rev 2.4
pub const QSFP_MAX_NUM_PAGES: c_int = 5;
//
// Below are masks for QSFP pins.  Pins are the same for HFI0 and HFI1.
// _N means asserted low
//

// QSFP is paged at 256 bytes
pub const QSFP_PAGESIZE: c_int = 256;
// Reads/writes cannot cross 128 byte boundaries
pub const QSFP_RW_BOUNDARY: c_int = 128;
// number of bytes in i2c offset for QSFP devices

// Defined fields that Intel requires of qualified cables
// Byte 0 is Identifier, not checked
// Byte 1 is reserved "status MSB"
pub const QSFP_MONITOR_VAL_START: c_int = 22;
pub const QSFP_MONITOR_VAL_END: c_int = 81;

pub const QSFP_TX_CTRL_BYTE_OFFS: c_int = 86;
pub const QSFP_PWR_CTRL_BYTE_OFFS: c_int = 93;
pub const QSFP_CDR_CTRL_BYTE_OFFS: c_int = 98;
pub const QSFP_PAGE_SELECT_BYTE_OFFS: c_int = 127;
// Byte 128 is Identifier: must be 0x0c for QSFP, or 0x0d for QSFP+
pub const QSFP_MOD_ID_OFFS: c_int = 128;
//
// Byte 129 is "Extended Identifier".
// For bits [7:6]: 0:1.5W, 1:2.0W, 2:2.5W, 3:3.5W
// For bits [1:0]: 0:Unused, 1:4W, 2:4.5W, 3:5W
//
pub const QSFP_MOD_PWR_OFFS: c_int = 129;
// Byte 130 is Connector type. Not Intel req'd
// Bytes 131..138 are Transceiver types, bit maps for various tech, none IB
// Byte 139 is encoding. code 0x01 is 8b10b. Not Intel req'd
// byte 140 is nominal bit-rate, in units of 100Mbits/sec
pub const QSFP_NOM_BIT_RATE_100_OFFS: c_int = 140;
// Byte 141 is Extended Rate Select. Not Intel req'd
// Bytes 142..145 are lengths for various fiber types. Not Intel req'd
// Byte 146 is length for Copper. Units of 1 meter
pub const QSFP_MOD_LEN_OFFS: c_int = 146;
//
// Byte 147 is Device technology. D0..3 not Intel req'd
// D4..7 select from 15 choices, translated by table:
//
pub const QSFP_MOD_TECH_OFFS: c_int = 147;
// Active Equalization includes fiber, copper full EQ, and copper near Eq

// Active Equalization includes fiber, copper full EQ, and copper far Eq

// Attenuation should be valid for copper other than full/near Eq

// Length is only valid if technology is "copper"

pub const QSFP_TECH_1490: c_int = 9;

pub const QSFP_OUI_AMPHENOL: c_uint = 0x415048;
pub const QSFP_OUI_FINISAR: c_uint = 0x009065;
pub const QSFP_OUI_GORE: c_uint = 0x002177;
// Bytes 148..163 are Vendor Name, Left-justified Blank-filled
pub const QSFP_VEND_OFFS: c_int = 148;
pub const QSFP_VEND_LEN: c_int = 16;
// Byte 164 is IB Extended transceiver codes Bits D0..3 are SDR,DDR,QDR,EDR
pub const QSFP_IBXCV_OFFS: c_int = 164;
// Bytes 165..167 are Vendor OUI number
pub const QSFP_VOUI_OFFS: c_int = 165;
pub const QSFP_VOUI_LEN: c_int = 3;
// Bytes 168..183 are Vendor Part Number, string
pub const QSFP_PN_OFFS: c_int = 168;
pub const QSFP_PN_LEN: c_int = 16;
// Bytes 184,185 are Vendor Rev. Left Justified, Blank-filled
pub const QSFP_REV_OFFS: c_int = 184;
pub const QSFP_REV_LEN: c_int = 2;
//
// Bytes 186,187 are Wavelength, if Optical. Not Intel req'd
// If copper, they are attenuation in dB:
// Byte 186 is at 2.5Gb/sec (SDR), Byte 187 at 5.0Gb/sec (DDR)
//
pub const QSFP_ATTEN_OFFS: c_int = 186;
pub const QSFP_ATTEN_LEN: c_int = 2;
//
// Bytes 188,189 are Wavelength tolerance, if optical
// If copper, they are attenuation in dB:
// Byte 188 is at 12.5 Gb/s, Byte 189 at 25 Gb/s
//
pub const QSFP_CU_ATTEN_7G_OFFS: c_int = 188;
pub const QSFP_CU_ATTEN_12G_OFFS: c_int = 189;
// Byte 190 is Max Case Temp. Not Intel req'd
// Byte 191 is LSB of sum of bytes 128..190. Not Intel req'd
pub const QSFP_CC_OFFS: c_int = 191;
pub const QSFP_EQ_INFO_OFFS: c_int = 193;
pub const QSFP_CDR_INFO_OFFS: c_int = 194;
// Bytes 196..211 are Serial Number, String
pub const QSFP_SN_OFFS: c_int = 196;
pub const QSFP_SN_LEN: c_int = 16;
// Bytes 212..219 are date-code YYMMDD (MM==1 for Jan)
pub const QSFP_DATE_OFFS: c_int = 212;
pub const QSFP_DATE_LEN: c_int = 6;
// Bytes 218,219 are optional lot-code, string
pub const QSFP_LOT_OFFS: c_int = 218;
pub const QSFP_LOT_LEN: c_int = 2;
// Bytes 220, 221 indicate monitoring options, Not Intel req'd
// Byte 222 indicates nominal bitrate in units of 250Mbits/sec
pub const QSFP_NOM_BIT_RATE_250_OFFS: c_int = 222;
// Byte 223 is LSB of sum of bytes 192..222
pub const QSFP_CC_EXT_OFFS: c_int = 223;
//
// Interrupt flag masks
//
pub const QSFP_DATA_NOT_READY: c_uint = 0x01;
pub const QSFP_HIGH_TEMP_ALARM: c_uint = 0x80;
pub const QSFP_LOW_TEMP_ALARM: c_uint = 0x40;
pub const QSFP_HIGH_TEMP_WARNING: c_uint = 0x20;
pub const QSFP_LOW_TEMP_WARNING: c_uint = 0x10;
pub const QSFP_HIGH_VCC_ALARM: c_uint = 0x80;
pub const QSFP_LOW_VCC_ALARM: c_uint = 0x40;
pub const QSFP_HIGH_VCC_WARNING: c_uint = 0x20;
pub const QSFP_LOW_VCC_WARNING: c_uint = 0x10;
pub const QSFP_HIGH_POWER_ALARM: c_uint = 0x88;
pub const QSFP_LOW_POWER_ALARM: c_uint = 0x44;
pub const QSFP_HIGH_POWER_WARNING: c_uint = 0x22;
pub const QSFP_LOW_POWER_WARNING: c_uint = 0x11;
pub const QSFP_HIGH_BIAS_ALARM: c_uint = 0x88;
pub const QSFP_LOW_BIAS_ALARM: c_uint = 0x44;
pub const QSFP_HIGH_BIAS_WARNING: c_uint = 0x22;
pub const QSFP_LOW_BIAS_WARNING: c_uint = 0x11;

//
// struct qsfp_data encapsulates state of QSFP device for one port.
// it will be part of port-specific data if a board supports QSFP.
//
// Since multiple board-types use QSFP, and their pport_data structs
// differ (in the chip-specific section), we need a pointer to its head.
//
// Avoiding premature optimization, we will have one work_struct per port,
// and let the qsfp_lock arbitrate access to common resources.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qsfp_data {
// Helps to find our way
    pub ppd: *mut hfi1_pportdata,
    pub qsfp_work: work_struct,
    pub 128]: *mut *mut u8 cache[QSFP_MAX_NUM_PAGES,
// protect qsfp data
    pub qsfp_lock: spinlock_t,
    pub check_interrupt_flags: u8,
    pub reset_needed: u8,
    pub limiting_active: u8,
    pub cache_valid: u8,
    pub cache_refresh_required: u8,
}

extern "C" {
    pub fn get_qsfp_power_class(power_byte: u8) -> c_int;
}
extern "C" {
    pub fn qsfp_mod_present(ppd: *mut hfi1_pportdata) -> c_int;
}
extern "C" {
    pub fn set_up_i2c(dd: *mut hfi1_devdata, ad: *mut hfi1_asic_data) -> c_int;
}
extern "C" {
    pub fn clean_up_i2c(dd: *mut hfi1_devdata, ad: *mut hfi1_asic_data);
}
