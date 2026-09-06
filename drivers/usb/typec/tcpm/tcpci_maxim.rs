//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/typec/tcpm/tcpci_maxim.h
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
// Copyright 2022 Google, Inc
//
// MAXIM TCPC header file.
//
pub const VENDOR_CC_STATUS2: c_uint = 0x85;

pub const TCPC_VENDOR_FLADC_STATUS: c_uint = 0x89;
pub const TCPC_VENDOR_CC_CTRL1: c_uint = 0x8c;

pub const TCPC_VENDOR_CC_CTRL2: c_uint = 0x8d;

pub const LOW_POWER_MODE_DISABLE: c_int = 0;
pub const ULTRA_LOW_POWER_MODE: c_int = 1;

pub const UA_1_SRC: c_int = 1;
pub const UA_80_SRC: c_int = 3;
pub const TCPC_VENDOR_CC_CTRL3: c_uint = 0x8e;

pub const CCWTRDEB_1MS: c_int = 1;

pub const CCWTRSEL_1V: c_uint = 0x4;

pub const WTRCYCLE_2_4_S: c_int = 0;
pub const WTRCYCLE_4_8_S: c_int = 1;
pub const TCPC_VENDOR_ADC_CTRL1: c_uint = 0x91;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum contamiant_state {
    NOT_DETECTED,
    DETECTED,
    SINK,
}

//
// @potential_contaminant:
// Last returned result to tcpm indicating whether the TCPM port
// has potential contaminant.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max_tcpci_chip {
    pub data: tcpci_data,
    pub tcpci: *mut tcpci,
    pub dev: *mut device,
    pub client: *mut i2c_client,
    pub port: *mut tcpm_port,
    pub contaminant_state: contamiant_state,
    pub veto_vconn_swap: bool,
    pub vbus_reg: *mut regulator,
}

extern "C" {
    pub fn regmap_raw_read(_arg: chip->data.regmap, _arg: reg, _arg: val, _arg: sizeof(u16)) -> return;
}
extern "C" {
    pub fn regmap_raw_write(_arg: chip->data.regmap, _arg: reg, _arg: &val, _arg: sizeof(u16)) -> return;
}
extern "C" {
    pub fn regmap_raw_read(_arg: chip->data.regmap, _arg: reg, _arg: val, _arg: sizeof(u8)) -> return;
}
extern "C" {
    pub fn regmap_raw_write(_arg: chip->data.regmap, _arg: reg, _arg: &val, _arg: sizeof(u8)) -> return;
}
//
// max_contaminant_is_contaminant - Test if CC was toggled due to contaminant
//
// @chip: Handle to a struct max_tcpci_chip
// @disconnect_while_debounce: Whether the disconnect was detected when CC
// pins were debouncing
// @cc_handled: Returns whether or not update to CC status was handled here
//
// Determine if a contaminant was detected.
//
// Returns: true if a contaminant was detected, false otherwise. cc_handled
// is updated to reflect whether or not further CC handling is required.
//
