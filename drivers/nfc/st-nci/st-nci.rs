//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/nfc/st-nci/st-nci.h
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
// NCI based Driver for STMicroelectronics NFC Chip
//
// Copyright (C) 2014  STMicroelectronics SAS. All rights reserved.
//

// Define private flags:
pub const ST_NCI_RUNNING: c_int = 1;
pub const ST_NCI_CORE_PROP: c_uint = 0x01;
pub const ST_NCI_SET_NFC_MODE: c_uint = 0x02;
//
// ref ISO7816-3 chap 8.1. the initial character TS is followed by a
// sequence of at most 32 characters.
//
pub const ST_NCI_ESE_MAX_LENGTH: c_int = 33;
pub const ST_NCI_DEVICE_MGNT_GATE: c_uint = 0x01;
pub const ST_NCI_VENDOR_OUI: c_uint = 0x0080E1 /* STMicroelectronics */;
pub const ST_NCI_FACTORY_MODE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_mode_set_cmd {
    pub cmd_type: u8,
    pub mode: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nci_mode_set_rsp {
    pub status: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_nci_se_status {
    pub is_ese_present: bool,
    pub is_uicc_present: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_nci_se_info {
    pub se_status: *mut st_nci_se_status,
    pub atr: [u8; ST_NCI_ESE_MAX_LENGTH],
    pub req_completion: completion,
    pub bwi_timer: timer_list,
    pub /: *mut *mut int wt_timeout; / in msecs,
    pub bwi_active: bool,
    pub se_active_timer: timer_list,
    pub se_active: bool,
    pub xch_error: bool,
    pub cb: se_io_cb_t,
    pub cb_context: *mut c_void,
}

//
// enum nfc_vendor_cmds - supported nfc vendor commands
//
// @FACTORY_MODE: Allow to set the driver into a mode where no secure element
// are activated. It does not consider any NFC_ATTR_VENDOR_DATA.
// @HCI_CLEAR_ALL_PIPES: Allow to execute a HCI clear all pipes command.
// It does not consider any NFC_ATTR_VENDOR_DATA.
// @HCI_DM_PUT_DATA: Allow to configure specific CLF registry as for example
// RF trimmings or low level drivers configurations (I2C, SPI, SWP).
// @HCI_DM_UPDATE_AID: Allow to configure an AID routing into the CLF routing
// table following RF technology, CLF mode or protocol.
// @HCI_DM_GET_INFO: Allow to retrieve CLF information.
// @HCI_DM_GET_DATA: Allow to retrieve CLF configurable data such as low
// level drivers configurations or RF trimmings.
// @HCI_DM_DIRECT_LOAD: Allow to load a firmware into the CLF. A complete
// packet can be more than 8KB.
// @HCI_DM_RESET: Allow to run a CLF reset in order to "commit" CLF
// configuration changes without CLF power off.
// @HCI_GET_PARAM: Allow to retrieve an HCI CLF parameter (for example the
// white list).
// @HCI_DM_FIELD_GENERATOR: Allow to generate different kind of RF
// technology. When using this command to anti-collision is done.
// @LOOPBACK: Allow to echo a command and test the Dh to CLF connectivity.
// @HCI_DM_VDC_MEASUREMENT_VALUE: Allow to measure the field applied on the
// CLF antenna. A value between 0 and 0x0f is returned. 0 is maximum.
// @HCI_DM_FWUPD_START: Allow to put CLF into firmware update mode. It is a
// specific CLF command as there is no GPIO for this.
// @HCI_DM_FWUPD_END:  Allow to complete firmware update.
// @HCI_DM_VDC_VALUE_COMPARISON: Allow to compare the field applied on the
// CLF antenna to a reference value.
// @MANUFACTURER_SPECIFIC: Allow to retrieve manufacturer specific data
// received during a NCI_CORE_INIT_CMD.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfc_vendor_cmds {
    FACTORY_MODE,
    HCI_CLEAR_ALL_PIPES,
    HCI_DM_PUT_DATA,
    HCI_DM_UPDATE_AID,
    HCI_DM_GET_INFO,
    HCI_DM_GET_DATA,
    HCI_DM_DIRECT_LOAD,
    HCI_DM_RESET,
    HCI_GET_PARAM,
    HCI_DM_FIELD_GENERATOR,
    LOOPBACK,
    HCI_DM_FWUPD_START,
    HCI_DM_FWUPD_END,
    HCI_DM_VDC_MEASUREMENT_VALUE,
    HCI_DM_VDC_VALUE_COMPARISON,
    MANUFACTURER_SPECIFIC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_nci_info {
    pub ndlc: *mut llt_ndlc,
    pub flags: c_ulong,
    pub se_info: st_nci_se_info,
}

extern "C" {
    pub fn st_nci_remove(ndev: *mut nci_dev);
}
extern "C" {
    pub fn st_nci_se_init(ndev: *mut nci_dev, se_status: *mut st_nci_se_status) -> c_int;
}
extern "C" {
    pub fn st_nci_se_deinit(ndev: *mut nci_dev);
}
extern "C" {
    pub fn st_nci_discover_se(ndev: *mut nci_dev) -> c_int;
}
extern "C" {
    pub fn st_nci_enable_se(ndev: *mut nci_dev, se_idx: u32) -> c_int;
}
extern "C" {
    pub fn st_nci_disable_se(ndev: *mut nci_dev, se_idx: u32) -> c_int;
}
extern "C" {
    pub fn st_nci_hci_load_session(ndev: *mut nci_dev) -> c_int;
}
extern "C" {
    pub fn st_nci_vendor_cmds_init(ndev: *mut nci_dev) -> c_int;
}
