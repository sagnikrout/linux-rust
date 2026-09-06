//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/nfc/st21nfca/st21nfca.h
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
// Copyright (C) 2014  STMicroelectronics SAS. All rights reserved.
//

pub const HCI_MODE: c_int = 0;
// framing in HCI mode
pub const ST21NFCA_SOF_EOF_LEN: c_int = 2;
// Almost every time value is 0
pub const ST21NFCA_HCI_LLC_LEN: c_int = 1;
// Size in worst case :
// In normal case CRC len = 2 but byte stuffing
// may appear in case one CRC byte = ST21NFCA_SOF_EOF
//
pub const ST21NFCA_HCI_LLC_CRC: c_int = 4;

// Worst case when adding byte stuffing between each byte
pub const ST21NFCA_HCI_LLC_MAX_PAYLOAD: c_int = 29;

// Reader RF commands
pub const ST21NFCA_WR_XCHG_DATA: c_uint = 0x10;
pub const ST21NFCA_DEVICE_MGNT_GATE: c_uint = 0x01;
pub const ST21NFCA_RF_READER_F_GATE: c_uint = 0x14;
pub const ST21NFCA_RF_CARD_F_GATE: c_uint = 0x24;
pub const ST21NFCA_APDU_READER_GATE: c_uint = 0xf0;
pub const ST21NFCA_CONNECTIVITY_GATE: c_uint = 0x41;
//
// ref ISO7816-3 chap 8.1. the initial character TS is followed by a
// sequence of at most 32 characters.
//
pub const ST21NFCA_ESE_MAX_LENGTH: c_int = 33;
pub const ST21NFCA_ESE_HOST_ID: c_uint = 0xc0;

pub const ST21NFCA_HCI_MODE: c_int = 0;
pub const ST21NFCA_NUM_DEVICES: c_int = 256;
pub const ST21NFCA_VENDOR_OUI: c_uint = 0x0080E1 /* STMicroelectronics */;
pub const ST21NFCA_FACTORY_MODE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st21nfca_se_status {
    pub is_ese_present: bool,
    pub is_uicc_present: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum st21nfca_state {
    ST21NFCA_ST_COLD,
    ST21NFCA_ST_READY,
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
// @HCI_DM_LOAD: Allow to load a firmware into the CLF. A complete
// packet can be more than 8KB.
// @HCI_DM_RESET: Allow to run a CLF reset in order to "commit" CLF
// configuration changes without CLF power off.
// @HCI_GET_PARAM: Allow to retrieve an HCI CLF parameter (for example the
// white list).
// @HCI_DM_FIELD_GENERATOR: Allow to generate different kind of RF
// technology. When using this command to anti-collision is done.
// @HCI_LOOPBACK: Allow to echo a command and test the Dh to CLF
// connectivity.
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
    HCI_DM_LOAD,
    HCI_DM_RESET,
    HCI_GET_PARAM,
    HCI_DM_FIELD_GENERATOR,
    HCI_LOOPBACK,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st21nfca_vendor_info {
    pub req_completion: completion,
    pub rx_skb: *mut sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st21nfca_dep_info {
    pub tx_pending: *mut sk_buff,
    pub tx_work: work_struct,
    pub curr_nfc_dep_pni: u8,
    pub idx: u32,
    pub to: u8,
    pub did: u8,
    pub bsi: u8,
    pub bri: u8,
    pub lri: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st21nfca_se_info {
    pub atr: [u8; ST21NFCA_ESE_MAX_LENGTH],
    pub req_completion: completion,
    pub bwi_timer: timer_list,
    pub /: *mut *mut int wt_timeout; / in msecs,
    pub bwi_active: bool,
    pub se_active_timer: timer_list,
    pub se_active: bool,
    pub expected_pipes: c_int,
    pub count_pipes: c_int,
    pub xch_error: bool,
    pub cb: se_io_cb_t,
    pub cb_context: *mut c_void,
    pub timeout_work: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st21nfca_hci_info {
    pub phy_ops: *const nfc_phy_ops,
    pub phy_id: *mut c_void,
    pub hdev: *mut nfc_hci_dev,
    pub se_status: *mut st21nfca_se_status,
    pub state: st21nfca_state,
    pub info_lock: mutex,
    pub async_cb_type: c_int,
    pub async_cb: data_exchange_cb_t,
    pub async_cb_context: *mut c_void,
    pub dep_info: st21nfca_dep_info,
    pub se_info: st21nfca_se_info,
    pub vendor_info: st21nfca_vendor_info,
}

extern "C" {
    pub fn st21nfca_hci_remove(hdev: *mut nfc_hci_dev);
}
extern "C" {
    pub fn st21nfca_tm_send_dep_res(hdev: *mut nfc_hci_dev, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn st21nfca_im_send_atr_req(hdev: *mut nfc_hci_dev, gb: *mut u8, gb_len: usize) -> c_int;
}
extern "C" {
    pub fn st21nfca_im_send_dep_req(hdev: *mut nfc_hci_dev, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn st21nfca_dep_init(hdev: *mut nfc_hci_dev);
}
extern "C" {
    pub fn st21nfca_dep_deinit(hdev: *mut nfc_hci_dev);
}
extern "C" {
    pub fn st21nfca_hci_discover_se(hdev: *mut nfc_hci_dev) -> c_int;
}
extern "C" {
    pub fn st21nfca_hci_enable_se(hdev: *mut nfc_hci_dev, se_idx: u32) -> c_int;
}
extern "C" {
    pub fn st21nfca_hci_disable_se(hdev: *mut nfc_hci_dev, se_idx: u32) -> c_int;
}
extern "C" {
    pub fn st21nfca_se_init(hdev: *mut nfc_hci_dev);
}
extern "C" {
    pub fn st21nfca_se_deinit(hdev: *mut nfc_hci_dev);
}
extern "C" {
    pub fn st21nfca_vendor_cmds_init(ndev: *mut nfc_hci_dev) -> c_int;
}
