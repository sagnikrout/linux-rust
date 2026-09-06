//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/nfc/mei_phy.h
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


// SPDX-License-Identifier: GPL-2.0

pub const MEI_NFC_HEADER_SIZE: c_int = 10;
pub const MEI_NFC_MAX_HCI_PAYLOAD: c_int = 300;
//
// struct nfc_mei_phy - NFC description of the MEI PHY and interface functions
//
// @cldev: mei client device
// @hdev:   nfc hci device
//
// @send_wq: send completion wait queue
// @fw_ivn: NFC Interface Version Number
// @vendor_id: NFC manufacturer ID
// @radio_type: NFC radio type
// @reserved: reserved for alignment
// @req_id:  message counter
// @recv_req_id: reception message counter
// @powered: the device is in powered state
// @hard_fault: < 0 if hardware error occurred
// and prevents normal operation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfc_mei_phy {
    pub cldev: *mut mei_cl_device,
    pub hdev: *mut nfc_hci_dev,
    pub send_wq: wait_queue_head_t,
    pub fw_ivn: u8,
    pub vendor_id: u8,
    pub radio_type: u8,
    pub reserved: u8,
    pub req_id: u16,
    pub recv_req_id: u16,
    pub powered: c_int,
    pub hard_fault: c_int,
}

extern "C" {
    pub fn nfc_mei_phy_free(phy: *mut nfc_mei_phy);
}
