//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/nfc/nfc.h
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
// Copyright (C) 2011 Instituto Nokia de Tecnologia
// Copyright (C) 2014 Marvell International Ltd.
//
// Authors:
// Lauro Ramos Venancio <lauro.venancio@openbossa.org>
// Aloisio Almeida Jr <aloisio.almeida@openbossa.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfc_phy_ops {
    pub skb): *mut *mut *mut int (write)(void dev_id, struct sk_buff,
    pub dev_id): *mut *mut int (enable)(void,
    pub dev_id): *mut *mut void (disable)(void,
}

//
// data_exchange_cb_t - Definition of nfc_data_exchange callback
//
// @context: nfc_data_exchange cb_context parameter
// @skb: response data
// @err: If an error has occurred during data exchange, it is the
// error number. Zero means no error.
//
// When a rx or tx package is lost or corrupted or the target gets out
// of the operating field, err is -EIO.
//
extern "C" {
    pub fn void(context: *mut *mut se_io_cb_t)(void, apdu: *mut u8, apdu_len: usize, err: c_int) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfc_ops {
    pub dev): *mut *mut int (dev_up)(struct nfc_dev,
    pub dev): *mut *mut int (dev_down)(struct nfc_dev,
    pub tm_protocols): u32 im_protocols, u32,
    pub dev): *mut *mut void (stop_poll)(struct nfc_dev,
    pub gb_len): *mut *mut u8 comm_mode, u8 gb, size_t,
    pub dev): *mut *mut int (dep_link_down)(struct nfc_dev,
    pub protocol): u32,
    pub mode): *mut *mut nfc_target target, u8,
    pub cb_context): *mut c_void,
    pub skb): *mut *mut *mut int (tm_send)(struct nfc_dev dev, struct sk_buff,
    pub target): *mut *mut *mut int (check_presence)(struct nfc_dev dev, struct nfc_target,
    pub firmware_name): *const *const *const int (fw_download)(struct nfc_dev dev, char,
// Secure Element API
    pub dev): *mut *mut int (discover_se)(struct nfc_dev,
    pub se_idx): *mut *mut *mut int (enable_se)(struct nfc_dev dev, u32,
    pub se_idx): *mut *mut *mut int (disable_se)(struct nfc_dev dev, u32,
    pub cb_context): *mut se_io_cb_t cb, void,
}

pub const NFC_MAX_GT_LEN: c_int = 48;
pub const NFC_ATR_RES_GT_OFFSET: c_int = 15;
pub const NFC_ATR_REQ_GT_OFFSET: c_int = 14;
//
// struct nfc_target - NFC target description
//
// @sens_res: 2 bytes describing the target SENS_RES response, if the target
// is a type A one. The %sens_res most significant byte must be byte 2
// as described by the NFC Forum digital specification (i.e. the platform
// configuration one) while %sens_res least significant byte is byte 1.
// @ats_len: length of Answer To Select in bytes
// @ats: Answer To Select returned by an ISO 14443 Type A target upon activation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfc_target {
    pub idx: u32,
    pub supported_protocols: u32,
    pub sens_res: u16,
    pub sel_res: u8,
    pub nfcid1_len: u8,
    pub nfcid1: [u8; NFC_NFCID1_MAXSIZE],
    pub nfcid2_len: u8,
    pub nfcid2: [u8; NFC_NFCID2_MAXSIZE],
    pub sensb_res_len: u8,
    pub sensb_res: [u8; NFC_SENSB_RES_MAXSIZE],
    pub sensf_res_len: u8,
    pub sensf_res: [u8; NFC_SENSF_RES_MAXSIZE],
    pub hci_reader_gate: u8,
    pub logical_idx: u8,
    pub is_iso15693: u8,
    pub iso15693_dsfid: u8,
    pub iso15693_uid: [u8; NFC_ISO15693_UID_MAXSIZE],
    pub ats_len: u8,
    pub ats: [u8; NFC_ATS_MAXSIZE],
}

//
// nfc_se - A structure for NFC accessible secure elements.
//
// @idx: The secure element index. User space will enable or
// disable a secure element by its index.
// @type: The secure element type. It can be SE_UICC or
// SE_EMBEDDED.
// @state: The secure element state, either enabled or disabled.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfc_se {
    pub list: list_head,
    pub idx: u32,
    pub type: u16,
    pub state: u16,
}

//
// nfc_evt_transaction - A struct for NFC secure element event transaction.
//
// @aid: The application identifier triggering the event
//
// @aid_len: The application identifier length [5:16]
//
// @params: The application parameters transmitted during the transaction
//
// @params_len: The applications parameters length [0:255]
//
pub const NFC_MIN_AID_LENGTH: c_int = 5;
pub const NFC_MAX_AID_LENGTH: c_int = 16;
pub const NFC_MAX_PARAMS_LENGTH: c_int = 255;
pub const NFC_EVT_TRANSACTION_AID_TAG: c_uint = 0x81;
pub const NFC_EVT_TRANSACTION_PARAMS_TAG: c_uint = 0x82;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfc_evt_transaction {
    pub aid_len: u32,
    pub aid: [u8; NFC_MAX_AID_LENGTH],
    pub params_len: u8,
    pub params: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfc_genl_data {
    pub poll_req_portid: u32,
    pub genl_data_mutex: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfc_vendor_cmd {
    pub vendor_id: __u32,
    pub subcmd: __u32,
    pub data_len): *mut *mut *mut *mut int (doit)(struct nfc_dev dev, void data, size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfc_dev {
    pub idx: c_int,
    pub target_next_idx: u32,
    pub targets: *mut nfc_target,
    pub n_targets: c_int,
    pub targets_generation: c_int,
    pub dev: device,
    pub dev_up: bool,
    pub fw_download_in_progress: bool,
    pub rf_mode: u8,
    pub polling: bool,
    pub active_target: *mut nfc_target,
    pub dep_link_up: bool,
    pub genl_data: nfc_genl_data,
    pub supported_protocols: u32,
    pub secure_elements: list_head,
    pub tx_headroom: c_int,
    pub tx_tailroom: c_int,
    pub check_pres_timer: timer_list,
    pub check_pres_work: work_struct,
    pub shutting_down: bool,
    pub rfkill: *mut rfkill,
    pub vendor_cmds: *const nfc_vendor_cmd,
    pub n_vendor_cmds: c_int,
    pub ops: *const nfc_ops,
    pub cur_cmd_info: *mut genl_info,
}

//
// nfc_free_device - free nfc device
//
// @dev: The nfc device to free
//
extern "C" {
    pub fn nfc_register_device(dev: *mut nfc_dev) -> c_int;
}
extern "C" {
    pub fn nfc_unregister_rfkill(dev: *mut nfc_dev);
}
extern "C" {
    pub fn nfc_remove_device(dev: *mut nfc_dev);
}
extern "C" {
    pub fn nfc_unregister_device(dev: *mut nfc_dev);
}
//
// nfc_set_parent_dev - set the parent device
//
// @nfc_dev: The nfc device whose parent is being set
// @dev: The parent device
//
// nfc_set_drvdata - set driver specific data
//
// @dev: The nfc device
// @data: Pointer to driver specific data
//
// nfc_get_drvdata - get driver specific data
//
// @dev: The nfc device
//
extern "C" {
    pub fn dev_get_drvdata(_arg: &dev->dev) -> return;
}
//
// nfc_device_name - get the nfc device name
//
// @dev: The nfc device whose name to return
//
extern "C" {
    pub fn dev_name(_arg: &dev->dev) -> return;
}
extern "C" {
    pub fn nfc_target_lost(dev: *mut nfc_dev, target_idx: u32) -> c_int;
}
extern "C" {
    pub fn nfc_tm_deactivated(dev: *mut nfc_dev) -> c_int;
}
extern "C" {
    pub fn nfc_tm_data_received(dev: *mut nfc_dev, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn nfc_driver_failure(dev: *mut nfc_dev, err: c_int);
}
extern "C" {
    pub fn nfc_se_connectivity(dev: *mut nfc_dev, se_idx: u8) -> c_int;
}
extern "C" {
    pub fn nfc_add_se(dev: *mut nfc_dev, se_idx: u32, type: u16) -> c_int;
}
extern "C" {
    pub fn nfc_remove_se(dev: *mut nfc_dev, se_idx: u32) -> c_int;
}
extern "C" {
    pub fn nfc_vendor_cmd_reply(skb: *mut sk_buff) -> c_int;
}
//
// nfc_vendor_cmd_alloc_reply_skb - allocate vendor command reply
// @dev: nfc device
// @oui: vendor oui
// @approxlen: an upper bound of the length of the data that will
// be put into the skb
//
// This function allocates and pre-fills an skb for a reply to
// a vendor command. Since it is intended for a reply, calling
// it outside of a vendor command's doit() operation is invalid.
//
// The returned skb is pre-filled with some identifying data in
// a way that any data that is put into the skb (with skb_put(),
// nla_put() or similar) will end up being within the
// %NFC_ATTR_VENDOR_DATA attribute, so all that needs to be done
// with the skb is adding data for the corresponding userspace tool
// which can then read that data out of the vendor data attribute.
// You must not modify the skb in any other way.
//
// When done, call nfc_vendor_cmd_reply() with the skb and return
// its error code as the result of the doit() operation.
//
// Return: An allocated and pre-filled skb. %NULL if any errors happen.
//
