//! Automatically rewritten from C Header to Rust Module
//! Source: net/nfc/nfc.h
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
//
// Authors:
// Lauro Ramos Venancio <lauro.venancio@openbossa.org>
// Aloisio Almeida Jr <aloisio.almeida@openbossa.org>
//

pub const NFC_TARGET_MODE_IDLE: c_int = 0;
pub const NFC_TARGET_MODE_SLEEP: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfc_protocol {
    pub id: c_int,
    pub proto: *mut proto,
    pub owner: *mut module,
    pub kern): *const *const nfc_protocol nfc_proto, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfc_rawsock {
    pub sk: sock,
    pub dev: *mut nfc_dev,
    pub target_idx: u32,
    pub tx_work: work_struct,
    pub tx_work_scheduled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfc_sock_list {
    pub head: hlist_head,
    pub lock: rwlock_t,
}

extern "C" {
    pub fn nfc_llcp_mac_is_down(dev: *mut nfc_dev);
}
extern "C" {
    pub fn nfc_llcp_register_device(dev: *mut nfc_dev) -> c_int;
}
extern "C" {
    pub fn nfc_llcp_unregister_device(dev: *mut nfc_dev);
}
extern "C" {
    pub fn nfc_llcp_set_remote_gb(dev: *mut nfc_dev, gb: *const u8, gb_len: u8) -> c_int;
}
extern "C" {
    pub fn nfc_llcp_data_received(dev: *mut nfc_dev, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn nfc_llcp_local_put(local: *mut nfc_llcp_local) -> c_int;
}
extern "C" {
    pub fn nfc_llcp_init() -> int __init;
}
extern "C" {
    pub fn nfc_llcp_exit();
}
extern "C" {
    pub fn nfc_llcp_free_sdp_tlv(sdp: *mut nfc_llcp_sdp_tlv);
}
extern "C" {
    pub fn nfc_llcp_free_sdp_tlv_list(head: *mut hlist_head);
}
extern "C" {
    pub fn rawsock_init() -> int __init;
}
extern "C" {
    pub fn rawsock_exit();
}
extern "C" {
    pub fn af_nfc_init() -> int __init;
}
extern "C" {
    pub fn af_nfc_exit();
}
extern "C" {
    pub fn nfc_proto_register(nfc_proto: *const nfc_protocol) -> c_int;
}
extern "C" {
    pub fn nfc_proto_unregister(nfc_proto: *const nfc_protocol);
}
extern "C" {
    pub fn nfc_genl_init() -> int __init;
}
extern "C" {
    pub fn nfc_genl_exit();
}
extern "C" {
    pub fn nfc_genl_data_init(genl_data: *mut nfc_genl_data);
}
extern "C" {
    pub fn nfc_genl_data_exit(genl_data: *mut nfc_genl_data);
}
extern "C" {
    pub fn nfc_genl_targets_found(dev: *mut nfc_dev) -> c_int;
}
extern "C" {
    pub fn nfc_genl_target_lost(dev: *mut nfc_dev, target_idx: u32) -> c_int;
}
extern "C" {
    pub fn nfc_genl_device_added(dev: *mut nfc_dev) -> c_int;
}
extern "C" {
    pub fn nfc_genl_device_removed(dev: *mut nfc_dev) -> c_int;
}
extern "C" {
    pub fn nfc_genl_dep_link_down_event(dev: *mut nfc_dev) -> c_int;
}
extern "C" {
    pub fn nfc_genl_tm_activated(dev: *mut nfc_dev, protocol: u32) -> c_int;
}
extern "C" {
    pub fn nfc_genl_tm_deactivated(dev: *mut nfc_dev) -> c_int;
}
extern "C" {
    pub fn nfc_genl_llc_send_sdres(dev: *mut nfc_dev, sdres_list: *mut hlist_head) -> c_int;
}
extern "C" {
    pub fn nfc_genl_se_added(dev: *mut nfc_dev, se_idx: u32, type: u16) -> c_int;
}
extern "C" {
    pub fn nfc_genl_se_removed(dev: *mut nfc_dev, se_idx: u32) -> c_int;
}
extern "C" {
    pub fn nfc_genl_se_connectivity(dev: *mut nfc_dev, se_idx: u8) -> c_int;
}
extern "C" {
    pub fn to_nfc_dev(_arg: d) -> return;
}
extern "C" {
    pub fn nfc_fw_download(dev: *mut nfc_dev, firmware_name: *const c_char) -> c_int;
}
extern "C" {
    pub fn nfc_dev_up(dev: *mut nfc_dev) -> c_int;
}
extern "C" {
    pub fn nfc_dev_down(dev: *mut nfc_dev) -> c_int;
}
extern "C" {
    pub fn nfc_start_poll(dev: *mut nfc_dev, im_protocols: u32, tm_protocols: u32) -> c_int;
}
extern "C" {
    pub fn nfc_stop_poll(dev: *mut nfc_dev) -> c_int;
}
extern "C" {
    pub fn nfc_dep_link_up(dev: *mut nfc_dev, target_idx: c_int, comm_mode: u8) -> c_int;
}
extern "C" {
    pub fn nfc_dep_link_down(dev: *mut nfc_dev) -> c_int;
}
extern "C" {
    pub fn nfc_activate_target(dev: *mut nfc_dev, target_idx: u32, protocol: u32) -> c_int;
}
extern "C" {
    pub fn nfc_deactivate_target(dev: *mut nfc_dev, target_idx: u32, mode: u8) -> c_int;
}
extern "C" {
    pub fn nfc_enable_se(dev: *mut nfc_dev, se_idx: u32) -> c_int;
}
extern "C" {
    pub fn nfc_disable_se(dev: *mut nfc_dev, se_idx: u32) -> c_int;
}
