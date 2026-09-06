//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/nfc/digital.h
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
// NFC Digital Protocol stack
// Copyright (c) 2013, Intel Corporation.
//

//
// Configuration types for in_configure_hw and tg_configure_hw.
//
// RF technology values passed as param argument to in_configure_hw and
// tg_configure_hw for NFC_DIGITAL_CONFIG_RF_TECH configuration type.
//
// Framing configuration passed as param argument to in_configure_hw and
// tg_configure_hw for NFC_DIGITAL_CONFIG_FRAMING configuration type.
//
pub const DIGITAL_MDAA_NFCID1_SIZE: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct digital_tg_mdaa_params {
    pub sens_res: u16,
    pub nfcid1: [u8; DIGITAL_MDAA_NFCID1_SIZE],
    pub sel_res: u8,
    pub nfcid2: [u8; NFC_NFCID2_MAXSIZE],
    pub sc: u16,
}

//
// nfc_digital_cmd_complete_t - Definition of command result callback
//
// @ddev: nfc_digital_device ref
// @arg: user data
// @resp: response data
//
// resp pointer can be an error code and will be checked with IS_ERR() macro.
// The callback is responsible for freeing resp sk_buff.
//
// Device side NFC Digital operations
//
// Initiator mode:
// @in_configure_hw: Hardware configuration for RF technology and communication
// framing in initiator mode. This is a synchronous function.
// @in_send_cmd: Initiator mode data exchange using RF technology and framing
// previously set with in_configure_hw. The peer response is returned
// through callback cb. If an io error occurs or the peer didn't reply
// within the specified timeout (ms), the error code is passed back through
// the resp pointer. This is an asynchronous function.
//
// Target mode: Only NFC-DEP protocol is supported in target mode.
// @tg_configure_hw: Hardware configuration for RF technology and communication
// framing in target mode. This is a synchronous function.
// @tg_send_cmd: Target mode data exchange using RF technology and framing
// previously set with tg_configure_hw. The peer next command is returned
// through callback cb. If an io error occurs or the peer didn't reply
// within the specified timeout (ms), the error code is passed back through
// the resp pointer. This is an asynchronous function.
// @tg_listen: Put the device in listen mode waiting for data from the peer
// device. This is an asynchronous function.
// @tg_listen_mdaa: If supported, put the device in automatic listen mode with
// mode detection and automatic anti-collision. In this mode, the device
// automatically detects the RF technology and executes the anti-collision
// detection using the command responses specified in mdaa_params. The
// mdaa_params structure contains SENS_RES, NFCID1, and SEL_RES for 106A RF
// tech. NFCID2 and system code (sc) for 212F and 424F. The driver returns
// the NFC-DEP ATR_REQ command through cb. The digital stack deducts the RF
// tech by analyzing the SoD of the frame containing the ATR_REQ command.
// This is an asynchronous function.
// @tg_listen_md: If supported, put the device in automatic listen mode with
// mode detection but without automatic anti-collision. In this mode, the
// device automatically detects the RF technology.  What the actual
// RF technology is can be retrieved by calling @tg_get_rf_tech.
// The digital stack will then perform the appropriate anti-collision
// sequence.  This is an asynchronous function.
// @tg_get_rf_tech: Required when @tg_listen_md is supported, unused otherwise.
// Return the RF Technology that was detected by the @tg_listen_md call.
// This is a synchronous function.
//
// @switch_rf: Turns device radio on or off. The stack does not call explicitly
// switch_rf to turn the radio on. A call to in|tg_configure_hw must turn
// the device radio on.
// @abort_cmd: Discard the last sent command.
//
// Notes: Asynchronous functions have a timeout parameter. It is the driver
// responsibility to call the digital stack back through the
// nfc_digital_cmd_complete_t callback when no RF respsonse has been
// received within the specified time (in milliseconds). In that case the
// driver must set the resp sk_buff to ERR_PTR(-ETIMEDOUT).
// Since the digital stack serializes commands to be sent, it's mandatory
// for the driver to handle the timeout correctly. Otherwise the stack
// would not be able to send new commands, waiting for the reply of the
// current one.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfc_digital_ops {
    pub param): c_int,
    pub arg): *mut c_void,
    pub param): c_int,
    pub arg): *mut c_void,
    pub arg): *mut nfc_digital_cmd_complete_t cb, void,
    pub arg): *mut c_void,
    pub arg): *mut nfc_digital_cmd_complete_t cb, void,
    pub rf_tech): *mut *mut *mut int (tg_get_rf_tech)(struct nfc_digital_dev ddev, u8,
    pub on): *mut *mut *mut int (switch_rf)(struct nfc_digital_dev ddev, bool,
    pub ddev): *mut *mut void (abort_cmd)(struct nfc_digital_dev,
}

extern "C" {
    pub fn int(ddev: *mut *mut digital_poll_t)(struct nfc_digital_dev, rf_tech: u8) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct digital_poll_tech {
    pub rf_tech: u8,
    pub poll_func: digital_poll_t,
}

//
// Driver capabilities - bit mask made of the following values
//
// @NFC_DIGITAL_DRV_CAPS_IN_CRC: The driver handles CRC calculation in initiator
// mode.
// @NFC_DIGITAL_DRV_CAPS_TG_CRC: The driver handles CRC calculation in target
// mode.
//
pub const NFC_DIGITAL_DRV_CAPS_IN_CRC: c_uint = 0x0001;
pub const NFC_DIGITAL_DRV_CAPS_TG_CRC: c_uint = 0x0002;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfc_digital_dev {
    pub nfc_dev: *mut nfc_dev,
    pub ops: *const nfc_digital_ops,
    pub protocols: u32,
    pub tx_headroom: c_int,
    pub tx_tailroom: c_int,
    pub driver_capabilities: u32,
    pub driver_data: *mut c_void,
    pub poll_techs: [digital_poll_tech; NFC_DIGITAL_POLL_MODE_COUNT_MAX],
    pub poll_tech_count: u8,
    pub poll_tech_index: u8,
    pub poll_lock: mutex,
    pub cmd_work: work_struct,
    pub cmd_complete_work: work_struct,
    pub cmd_queue: list_head,
    pub cmd_lock: mutex,
    pub poll_work: delayed_work,
    pub curr_protocol: u8,
    pub curr_rf_tech: u8,
    pub curr_nfc_dep_pni: u8,
    pub did: u8,
    pub dep_rwt: u16,
    pub local_payload_max: u8,
    pub remote_payload_max: u8,
    pub chaining_skb: *mut sk_buff,
    pub data_exch: *mut digital_data_exch,
    pub atn_count: c_int,
    pub nack_count: c_int,
    pub saved_skb: *mut sk_buff,
    pub target_fsc: u16,
    pub skb): *mut *mut int (skb_check_crc)(struct sk_buff,
    pub skb): *mut *mut void (skb_add_crc)(struct sk_buff,
}

extern "C" {
    pub fn nfc_digital_free_device(ndev: *mut nfc_digital_dev);
}
extern "C" {
    pub fn nfc_digital_register_device(ndev: *mut nfc_digital_dev) -> c_int;
}
extern "C" {
    pub fn nfc_digital_unregister_device(ndev: *mut nfc_digital_dev);
}
