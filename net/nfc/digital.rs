//! Automatically rewritten from C Header to Rust Module
//! Source: net/nfc/digital.h
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

pub const DIGITAL_CMD_IN_SEND: c_int = 0;
pub const DIGITAL_CMD_TG_SEND: c_int = 1;
pub const DIGITAL_CMD_TG_LISTEN: c_int = 2;
pub const DIGITAL_CMD_TG_LISTEN_MDAA: c_int = 3;
pub const DIGITAL_CMD_TG_LISTEN_MD: c_int = 4;
pub const DIGITAL_MAX_HEADER_LEN: c_int = 7;
pub const DIGITAL_CRC_LEN: c_int = 2;
pub const DIGITAL_SENSF_NFCID2_NFC_DEP_B1: c_uint = 0x01;
pub const DIGITAL_SENSF_NFCID2_NFC_DEP_B2: c_uint = 0xFE;
pub const DIGITAL_SENS_RES_NFC_DEP: c_uint = 0x0100;
pub const DIGITAL_SEL_RES_NFC_DEP: c_uint = 0x40;
pub const DIGITAL_SENSF_FELICA_SC: c_uint = 0xFFFF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct digital_data_exch {
    pub cb: data_exchange_cb_t,
    pub cb_context: *mut c_void,
}

extern "C" {
    pub fn digital_in_configure_hw(ddev: *mut nfc_digital_dev, type: c_int, param: c_int) -> c_int;
}
extern "C" {
    pub fn digital_poll_next_tech(ddev: *mut nfc_digital_dev);
}
extern "C" {
    pub fn digital_in_send_sens_req(ddev: *mut nfc_digital_dev, rf_tech: u8) -> c_int;
}
extern "C" {
    pub fn digital_in_send_sensb_req(ddev: *mut nfc_digital_dev, rf_tech: u8) -> c_int;
}
extern "C" {
    pub fn digital_in_send_sensf_req(ddev: *mut nfc_digital_dev, rf_tech: u8) -> c_int;
}
extern "C" {
    pub fn digital_in_send_iso15693_inv_req(ddev: *mut nfc_digital_dev, rf_tech: u8) -> c_int;
}
extern "C" {
    pub fn digital_in_recv_mifare_res(resp: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn digital_tg_configure_hw(ddev: *mut nfc_digital_dev, type: c_int, param: c_int) -> c_int;
}
extern "C" {
    pub fn digital_tg_send_dep_res(ddev: *mut nfc_digital_dev, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn digital_tg_listen_nfca(ddev: *mut nfc_digital_dev, rf_tech: u8) -> c_int;
}
extern "C" {
    pub fn digital_tg_listen_nfcf(ddev: *mut nfc_digital_dev, rf_tech: u8) -> c_int;
}
extern "C" {
    pub fn u16(_arg: *mut crc_func_t)(u16, : *const u8, _arg: usize) -> typedef;
}
pub const CRC_A_INIT: c_uint = 0x6363;
pub const CRC_B_INIT: c_uint = 0xFFFF;
pub const CRC_F_INIT: c_uint = 0x0000;
extern "C" {
    pub fn digital_skb_check_crc(_arg: skb, _arg: crc_ccitt, _arg: CRC_A_INIT, _arg: 0, _arg: 0) -> return;
}
extern "C" {
    pub fn digital_skb_check_crc(_arg: skb, _arg: crc_ccitt, _arg: CRC_B_INIT, _arg: 1, _arg: 0) -> return;
}
extern "C" {
    pub fn digital_skb_check_crc(_arg: skb, _arg: crc_itu_t, _arg: CRC_F_INIT, _arg: 0, _arg: 1) -> return;
}
