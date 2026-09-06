//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/nfc/hci.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2011  Intel Corporation. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfc_hci_ops {
    pub hdev): *mut *mut int (open) (struct nfc_hci_dev,
    pub hdev): *mut *mut void (close) (struct nfc_hci_dev,
    pub hdev): *mut *mut int (load_session) (struct nfc_hci_dev,
    pub hdev): *mut *mut int (hci_ready) (struct nfc_hci_dev,
//
// xmit must always send the complete buffer before
// returning. Returned result must be 0 for success
// or negative for failure.
//
    pub skb): *mut *mut *mut int (xmit) (struct nfc_hci_dev hdev, struct sk_buff,
    pub tm_protocols): u32 im_protocols, u32,
    pub hdev): *mut *mut void (stop_poll) (struct nfc_hci_dev,
    pub gb_len): *mut *mut u8 comm_mode, u8 gb, size_t,
    pub hdev): *mut *mut int (dep_link_down)(struct nfc_hci_dev,
    pub target): *mut nfc_target,
    pub target): *mut nfc_target,
    pub cb_context): *mut data_exchange_cb_t cb, void,
    pub skb): *mut *mut *mut int (tm_send)(struct nfc_hci_dev hdev, struct sk_buff,
    pub target): *mut nfc_target,
    pub skb): *mut sk_buff,
    pub skb): *mut sk_buff,
    pub firmware_name): *const *const *const int (fw_download)(struct nfc_hci_dev hdev, char,
    pub dev): *mut *mut int (discover_se)(struct nfc_hci_dev,
    pub se_idx): *mut *mut *mut int (enable_se)(struct nfc_hci_dev dev, u32,
    pub se_idx): *mut *mut *mut int (disable_se)(struct nfc_hci_dev dev, u32,
    pub cb_context): *mut se_io_cb_t cb, void,
}

// Pipes
pub const NFC_HCI_DO_NOT_CREATE_PIPE: c_uint = 0x81;
pub const NFC_HCI_INVALID_PIPE: c_uint = 0x80;
pub const NFC_HCI_INVALID_GATE: c_uint = 0xFF;
pub const NFC_HCI_INVALID_HOST: c_uint = 0x80;
pub const NFC_HCI_LINK_MGMT_PIPE: c_uint = 0x00;
pub const NFC_HCI_ADMIN_PIPE: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfc_hci_gate {
    pub gate: u8,
    pub pipe: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfc_hci_pipe {
    pub gate: u8,
    pub dest_host: u8,
}

pub const NFC_HCI_MAX_CUSTOM_GATES: c_int = 50;
//
// According to specification 102 622 chapter 4.4 Pipes,
// the pipe identifier is 7 bits long.
//
pub const NFC_HCI_MAX_PIPES: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfc_hci_init_data {
    pub gate_count: u8,
    pub gates: [nfc_hci_gate; NFC_HCI_MAX_CUSTOM_GATES],
    pub session_id: [c_char; 9],
}

extern "C" {
    pub fn int(skb: *mut *mut xmit) (struct sk_buff, cb_data: *mut c_void) -> typedef;
}
pub const NFC_HCI_MAX_GATES: c_int = 256;
//
// These values can be specified by a driver to indicate it requires some
// adaptation of the HCI standard.
//
// NFC_HCI_QUIRK_SHORT_CLEAR - send HCI_ADM_CLEAR_ALL_PIPE cmd with no params
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfc_hci_dev {
    pub ndev: *mut nfc_dev,
    pub max_data_link_payload: u32,
    pub shutting_down: bool,
    pub msg_tx_mutex: mutex,
    pub msg_tx_queue: list_head,
    pub msg_tx_work: work_struct,
    pub cmd_timer: timer_list,
    pub cmd_pending_msg: *mut hci_msg,
    pub rx_hcp_frags: sk_buff_head,
    pub msg_rx_work: work_struct,
    pub msg_rx_queue: sk_buff_head,
    pub ops: *const nfc_hci_ops,
    pub llc: *mut nfc_llc,
    pub init_data: nfc_hci_init_data,
    pub clientdata: *mut c_void,
    pub gate2pipe: [u8; NFC_HCI_MAX_GATES],
    pub pipes: [nfc_hci_pipe; NFC_HCI_MAX_PIPES],
    pub sw_romlib: u8,
    pub sw_patch: u8,
    pub sw_flashlib_major: u8,
    pub sw_flashlib_minor: u8,
    pub hw_derivative: u8,
    pub hw_version: u8,
    pub hw_mpw: u8,
    pub hw_software: u8,
    pub hw_bsid: u8,
    pub async_cb_type: c_int,
    pub async_cb: data_exchange_cb_t,
    pub async_cb_context: *mut c_void,
    pub gb: *mut u8,
    pub gb_len: usize,
    pub quirks: c_ulong,
}

// hci device allocation
extern "C" {
    pub fn nfc_hci_free_device(hdev: *mut nfc_hci_dev);
}
extern "C" {
    pub fn nfc_hci_register_device(hdev: *mut nfc_hci_dev) -> c_int;
}
extern "C" {
    pub fn nfc_hci_unregister_device(hdev: *mut nfc_hci_dev);
}
extern "C" {
    pub fn nfc_hci_set_clientdata(hdev: *mut nfc_hci_dev, clientdata: *mut c_void);
}
extern "C" {
    pub fn nfc_set_vendor_cmds(_arg: hdev->ndev, _arg: cmds, _arg: n_cmds) -> return;
}
extern "C" {
    pub fn nfc_hci_driver_failure(hdev: *mut nfc_hci_dev, err: c_int);
}
extern "C" {
    pub fn nfc_hci_result_to_errno(result: u8) -> c_int;
}
extern "C" {
    pub fn nfc_hci_reset_pipes(dev: *mut nfc_hci_dev);
}
extern "C" {
    pub fn nfc_hci_reset_pipes_per_host(hdev: *mut nfc_hci_dev, host: u8);
}
// Host IDs
pub const NFC_HCI_HOST_CONTROLLER_ID: c_uint = 0x00;
pub const NFC_HCI_TERMINAL_HOST_ID: c_uint = 0x01;
pub const NFC_HCI_UICC_HOST_ID: c_uint = 0x02;
// Host Controller Gates and registry indexes
pub const NFC_HCI_ADMIN_GATE: c_uint = 0x00;
pub const NFC_HCI_ADMIN_SESSION_IDENTITY: c_uint = 0x01;
pub const NFC_HCI_ADMIN_MAX_PIPE: c_uint = 0x02;
pub const NFC_HCI_ADMIN_WHITELIST: c_uint = 0x03;
pub const NFC_HCI_ADMIN_HOST_LIST: c_uint = 0x04;
pub const NFC_HCI_LOOPBACK_GATE: c_uint = 0x04;
pub const NFC_HCI_ID_MGMT_GATE: c_uint = 0x05;
pub const NFC_HCI_ID_MGMT_VERSION_SW: c_uint = 0x01;
pub const NFC_HCI_ID_MGMT_VERSION_HW: c_uint = 0x03;
pub const NFC_HCI_ID_MGMT_VENDOR_NAME: c_uint = 0x04;
pub const NFC_HCI_ID_MGMT_MODEL_ID: c_uint = 0x05;
pub const NFC_HCI_ID_MGMT_HCI_VERSION: c_uint = 0x02;
pub const NFC_HCI_ID_MGMT_GATES_LIST: c_uint = 0x06;
pub const NFC_HCI_LINK_MGMT_GATE: c_uint = 0x06;
pub const NFC_HCI_LINK_MGMT_REC_ERROR: c_uint = 0x01;
pub const NFC_HCI_RF_READER_B_GATE: c_uint = 0x11;
pub const NFC_HCI_RF_READER_B_PUPI: c_uint = 0x03;
pub const NFC_HCI_RF_READER_B_APPLICATION_DATA: c_uint = 0x04;
pub const NFC_HCI_RF_READER_B_AFI: c_uint = 0x02;
pub const NFC_HCI_RF_READER_B_HIGHER_LAYER_RESPONSE: c_uint = 0x01;
pub const NFC_HCI_RF_READER_B_HIGHER_LAYER_DATA: c_uint = 0x05;
pub const NFC_HCI_RF_READER_A_GATE: c_uint = 0x13;
pub const NFC_HCI_RF_READER_A_UID: c_uint = 0x02;
pub const NFC_HCI_RF_READER_A_ATQA: c_uint = 0x04;
pub const NFC_HCI_RF_READER_A_APPLICATION_DATA: c_uint = 0x05;
pub const NFC_HCI_RF_READER_A_SAK: c_uint = 0x03;
pub const NFC_HCI_RF_READER_A_FWI_SFGT: c_uint = 0x06;
pub const NFC_HCI_RF_READER_A_DATARATE_MAX: c_uint = 0x01;

pub const NFC_HCI_TYPE_A_SEL_PROT_MIFARE: c_int = 0;
pub const NFC_HCI_TYPE_A_SEL_PROT_ISO14443: c_int = 1;
pub const NFC_HCI_TYPE_A_SEL_PROT_DEP: c_int = 2;
pub const NFC_HCI_TYPE_A_SEL_PROT_ISO14443_DEP: c_int = 3;
// Generic events
pub const NFC_HCI_EVT_HCI_END_OF_OPERATION: c_uint = 0x01;
pub const NFC_HCI_EVT_POST_DATA: c_uint = 0x02;
pub const NFC_HCI_EVT_HOT_PLUG: c_uint = 0x03;
// Generic commands
pub const NFC_HCI_ANY_SET_PARAMETER: c_uint = 0x01;
pub const NFC_HCI_ANY_GET_PARAMETER: c_uint = 0x02;
pub const NFC_HCI_ANY_OPEN_PIPE: c_uint = 0x03;
pub const NFC_HCI_ANY_CLOSE_PIPE: c_uint = 0x04;
// Reader RF gates events
pub const NFC_HCI_EVT_READER_REQUESTED: c_uint = 0x10;
pub const NFC_HCI_EVT_END_OPERATION: c_uint = 0x11;
// Reader Application gate events
pub const NFC_HCI_EVT_TARGET_DISCOVERED: c_uint = 0x10;
// receiving messages from lower layer
extern "C" {
    pub fn nfc_hci_recv_frame(hdev: *mut nfc_hci_dev, skb: *mut sk_buff);
}
// connecting to gates and sending hci instructions
extern "C" {
    pub fn nfc_hci_disconnect_gate(hdev: *mut nfc_hci_dev, gate: u8) -> c_int;
}
extern "C" {
    pub fn nfc_hci_disconnect_all_gates(hdev: *mut nfc_hci_dev) -> c_int;
}
extern "C" {
    pub fn nfc_hci_target_discovered(hdev: *mut nfc_hci_dev, gate: u8) -> c_int;
}
extern "C" {
    pub fn nfc_hci_sak_to_protocol(sak: u8) -> u32;
}
