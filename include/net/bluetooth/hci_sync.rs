//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/bluetooth/hci_sync.h
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
//
// BlueZ - Bluetooth protocol stack for Linux
//
// Copyright (C) 2021 Intel Corporation
//

pub const HCI_REQ_DONE: c_int = 0;
pub const HCI_REQ_PEND: c_int = 1;
pub const HCI_REQ_CANCELED: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_request {
    pub hdev: *mut hci_dev,
    pub cmd_q: sk_buff_head,
// If something goes wrong when building the HCI request, the error
// value is stored in this field.
//
    pub err: c_int,
}

extern "C" {
    pub fn int(hdev: *mut *mut hci_cmd_sync_work_func_t)(struct hci_dev, data: *mut c_void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cmd_sync_work_entry {
    pub list: list_head,
    pub func: hci_cmd_sync_work_func_t,
    pub data: *mut c_void,
    pub destroy: hci_cmd_sync_work_destroy_t,
}

// Function with sync suffix shall not be called with hdev->lock held as they
// wait the command to complete and in the meantime an event could be received
// which could attempt to acquire hdev->lock causing a deadlock.
//
extern "C" {
    pub fn __hci_reset_sync(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn hci_cmd_sync_init(hdev: *mut hci_dev);
}
extern "C" {
    pub fn hci_cmd_sync_clear(hdev: *mut hci_dev);
}
extern "C" {
    pub fn hci_cmd_sync_cancel(hdev: *mut hci_dev, err: c_int);
}
extern "C" {
    pub fn hci_cmd_sync_cancel_sync(hdev: *mut hci_dev, err: c_int);
}
extern "C" {
    pub fn hci_update_eir_sync(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn hci_update_class_sync(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn hci_update_eir_sync(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn hci_update_class_sync(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn hci_update_name_sync(hdev: *mut hci_dev, name: *const u8) -> c_int;
}
extern "C" {
    pub fn hci_write_ssp_mode_sync(hdev: *mut hci_dev, mode: u8) -> c_int;
}
extern "C" {
    pub fn hci_update_scan_rsp_data_sync(hdev: *mut hci_dev, instance: u8) -> c_int;
}
extern "C" {
    pub fn hci_update_adv_data_sync(hdev: *mut hci_dev, instance: u8) -> c_int;
}
extern "C" {
    pub fn hci_update_adv_data(hdev: *mut hci_dev, instance: u8) -> c_int;
}
extern "C" {
    pub fn hci_setup_ext_adv_instance_sync(hdev: *mut hci_dev, instance: u8) -> c_int;
}
extern "C" {
    pub fn hci_start_ext_adv_sync(hdev: *mut hci_dev, instance: u8) -> c_int;
}
extern "C" {
    pub fn hci_enable_ext_advertising_sync(hdev: *mut hci_dev, instance: u8) -> c_int;
}
extern "C" {
    pub fn hci_enable_advertising_sync(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn hci_enable_advertising(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn hci_disable_per_advertising_sync(hdev: *mut hci_dev, instance: u8) -> c_int;
}
extern "C" {
    pub fn hci_disable_advertising_sync(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn hci_update_passive_scan_sync(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn hci_update_passive_scan(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn hci_read_rssi_sync(hdev: *mut hci_dev, handle: __le16) -> c_int;
}
extern "C" {
    pub fn hci_read_tx_power_sync(hdev: *mut hci_dev, handle: __le16, type: u8) -> c_int;
}
extern "C" {
    pub fn hci_write_sc_support_sync(hdev: *mut hci_dev, val: u8) -> c_int;
}
extern "C" {
    pub fn hci_read_clock_sync(hdev: *mut hci_dev, cp: *mut hci_cp_read_clock) -> c_int;
}
extern "C" {
    pub fn hci_write_fast_connectable_sync(hdev: *mut hci_dev, enable: bool) -> c_int;
}
extern "C" {
    pub fn hci_update_scan_sync(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn hci_update_scan(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn hci_write_le_host_supported_sync(hdev: *mut hci_dev, le: u8, simul: u8) -> c_int;
}
extern "C" {
    pub fn hci_reset_sync(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn hci_dev_open_sync(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn hci_dev_close_sync(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn hci_powered_update_sync(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn hci_set_powered_sync(hdev: *mut hci_dev, val: u8) -> c_int;
}
extern "C" {
    pub fn hci_update_discoverable_sync(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn hci_update_discoverable(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn hci_update_connectable_sync(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn hci_inquiry_sync(hdev: *mut hci_dev, length: u8, num_rsp: u8) -> c_int;
}
extern "C" {
    pub fn hci_start_discovery_sync(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn hci_stop_discovery_sync(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn hci_suspend_sync(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn hci_resume_sync(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn hci_abort_conn_sync(hdev: *mut hci_dev, conn: *mut hci_conn, reason: u8) -> c_int;
}
extern "C" {
    pub fn hci_le_create_cis_sync(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn hci_le_remove_cig_sync(hdev: *mut hci_dev, handle: u8) -> c_int;
}
extern "C" {
    pub fn hci_le_terminate_big_sync(hdev: *mut hci_dev, handle: u8, reason: u8) -> c_int;
}
extern "C" {
    pub fn hci_le_big_terminate_sync(hdev: *mut hci_dev, handle: u8) -> c_int;
}
extern "C" {
    pub fn hci_le_pa_terminate_sync(hdev: *mut hci_dev, handle: u16) -> c_int;
}
extern "C" {
    pub fn hci_connect_acl_sync(hdev: *mut hci_dev, conn: *mut hci_conn) -> c_int;
}
extern "C" {
    pub fn hci_connect_le_sync(hdev: *mut hci_dev, conn: *mut hci_conn) -> c_int;
}
extern "C" {
    pub fn hci_cancel_connect_sync(hdev: *mut hci_dev, conn: *mut hci_conn) -> c_int;
}
extern "C" {
    pub fn hci_le_conn_rate_request(hdev: *mut hci_dev, conn: *mut hci_conn) -> c_int;
}
extern "C" {
    pub fn hci_connect_pa_sync(hdev: *mut hci_dev, conn: *mut hci_conn) -> c_int;
}
extern "C" {
    pub fn hci_connect_big_sync(hdev: *mut hci_dev, conn: *mut hci_conn) -> c_int;
}
extern "C" {
    pub fn hci_past_sync(conn: *mut hci_conn, le: *mut hci_conn) -> c_int;
}
extern "C" {
    pub fn hci_le_read_remote_features(conn: *mut hci_conn) -> c_int;
}
extern "C" {
    pub fn hci_acl_change_pkt_type(conn: *mut hci_conn, pkt_type: u16) -> c_int;
}
extern "C" {
    pub fn hci_le_set_phy(conn: *mut hci_conn, tx_phys: u8, rx_phys: u8) -> c_int;
}
